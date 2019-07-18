{ installMethodFilter ? null, imageNameFilter ? null }:
let
  lib = import ./lib.nix {
    inherit installMethodFilter imageNameFilter;
  };

  inherit (lib) pkgs shellcheckedScript filteredImages
    casesToRun;

  testScript = name: details: shellcheckedScript "test.sh"
    (builtins.readFile ./tests.sh);

  mkVagrantfile = name: details: pkgs.writeText "Vagrantfile" ''
    Vagrant.configure("2") do |config|
      config.vm.box = "${details.image}"
      config.vm.provision "shell", inline: <<-SHELL
    ${details.preInstall}
      SHELL
      config.vm.synced_folder ".", "/vagrant", disabled: true
      config.vm.box_check_update = false
      config.vm.provider "virtualbox" do |vb|
        vb.memory = "2048"

        # for macos:
        vb.customize ["modifyvm", :id, "--usb", "off"]
        vb.customize ["modifyvm", :id, "--usbehci", "off"]
      end
    end
  '';

  mkTestScript = installScript: name: details: shellcheckedScript "run-${installScript.name}-${name}.sh" ''
    #!/bin/sh

    PATH=${pkgs.vagrant}/bin/:${pkgs.coreutils}/bin/:$PATH

    printf "\\n\\n\\n\\n\\n"
    echo "Test script for ${installScript.name}-${name}"
    printf "\\n\\n\\n\\n\\n"

    set -eu

    destdir=$1
    shift

    scratch=$(mktemp -d -t tmp.XXXXXXXXXX)
    finish() {
      rm -rf "$scratch"
    }
    trap finish EXIT

    cd "$scratch"

    finish() {
      vagrant destroy --force
      rm -rf "$scratch"
    }
    trap finish EXIT

    mkdir log-results

    cp ${mkVagrantfile name details} ./Vagrantfile
    cp ./Vagrantfile ./log-results/

    echo "${name}" > ./log-results/image-name
    echo "${installScript.name}" > ./log-results/install-method

    (
      vagrant up

      vagrant ssh -- tee install < ${shellcheckedScript installScript.name installScript.script}
      vagrant ssh -- chmod +x install

      vagrant ssh -- tee testscript < ${testScript name details}
      vagrant ssh -- chmod +x testscript

      vagrant ssh -- ./install 2>&1 \
        | sed -e "s/^/${name}-install    /"

      set +e

      runtest() {
        name=$1
        shift
        vagrant ssh -- "$@" ./testscript 2>&1 \
          | sed -e "s/^/${name}-test-$name    /"
        mkdir -p "./log-results/test-results/$name"
        vagrant ssh -- cat ./nix-test-matrix-log.tar | tar -xC "./log-results/test-results/$name"
      }

      runtest login bash --login
      runtest login-interactive bash --login -i
      runtest interactive bash -i
      runtest ssh
    ) 2>&1 | tee ./log-results/run-log

    mv ./log-results "$destdir"
  '';

  mkImageFetchScript = imagename:
    shellcheckedScript "fetch-image"
      ''
        #!/bin/sh

        set -euxo pipefail

        PATH=${pkgs.vagrant}/bin/:${pkgs.coreutils}/bin/:${pkgs.gnugrep}/bin/:${pkgs.curl}/bin/

        if ! vagrant box list | grep -q "${imagename}"; then
          vagrant box add "${imagename}" --provider=virtualbox
        fi
      '';
in shellcheckedScript "run-tests.sh"
''
  #!/bin/sh

  set -eux

  PATH="${pkgs.coreutils}/bin/:$PATH"

  destdir=$(realpath "$1")
  mkdir -p "$destdir"
  shift

  set +e

  echo "Pre-fetching images"
  cat <<EOF | ${pkgs.findutils}/bin/xargs -L 1 -P 4 bash
  ${pkgs.lib.concatStringsSep "\n"
  (builtins.map (image: mkImageFetchScript image.image)
    (builtins.attrValues filteredImages)
    )}
  EOF

  echo "Running tests"

  cat <<EOF | ${pkgs.findutils}/bin/xargs -L 1 -P 4 bash
  ${pkgs.lib.concatStringsSep "\n"
  (builtins.map (case:
    let cmd = mkTestScript case.installMethod case.imageName case.imageConfig;
    in "${cmd} \"$destdir/${case.installMethod.name}-${case.imageName}\"") casesToRun
    )}
  EOF
''
