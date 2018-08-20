{ installMethodFilter ? null, imageNameFilter ? null }:
let
  pkgs = import <nixpkgs> {};
  srcs = import ./matrix.nix;

  testScript = name: details: shellcheckedScript "test.sh"
    (builtins.readFile ./tests.sh);

  shellcheckedScript = name: text:
    pkgs.runCommand "shellchecked-${name}" {
      src = pkgs.writeScript name text;
      buildInputs = with pkgs; [ shellcheck ];
    } ''
      shellcheck $src
      cp $src $out
    '';

  mkVagrantfile = name: details: pkgs.writeText "Vagrantfile" ''
    Vagrant.configure("2") do |config|
      config.vm.box = "${details.image}"
      config.vm.provision "shell", inline: <<-SHELL
    ${details.preInstall}
      SHELL
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

    cp ${mkVagrantfile name details} ./Vagrantfile
    vagrant up

    vagrant ssh -- tee install < ${shellcheckedScript installScript.name installScript.script}
    vagrant ssh -- chmod +x install

    vagrant ssh -- tee testscript < ${testScript name details}
    vagrant ssh -- chmod +x testscript

    vagrant ssh -- ./install 2>&1 \
      | sed -e "s/^/${name}-install    /"

    vagrant ssh -- bash --login -i ./testscript 2>&1 \
      | sed -e "s/^/${name}-test-login-interactive    /"

    vagrant ssh -- bash -i ./testscript 2>&1 \
      | sed -e "s/^/${name}-test-interactive    /"

    vagrant ssh -- ./testscript 2>&1 \
      | sed -e "s/^/${name}-test-ssh    /"
  '';

  testCases = installMethods: images:
    pkgs.lib.flatten ((pkgs.lib.flip builtins.map installMethods)
      (installMethod:
        (pkgs.lib.flip pkgs.lib.mapAttrsToList images)
          (imageName: imageConfig: {
            installMethod = installMethod;
            imageName = imageName;
            imageConfig = imageConfig;
          })
    ));

  casesToRun = testCases
    (builtins.filter
      (installer: if installMethodFilter == null then true
        else installer.name == installMethodFilter) srcs.installScripts)
      (pkgs.lib.filterAttrs
        (name: _: if imageNameFilter == null then true
          else name == imageNameFilter) srcs.images);

in pkgs.writeScript "run-tests.sh"
''
  #!/bin/sh

  set -x

  ${pkgs.lib.concatStringsSep "\n"
    (builtins.map (case: mkTestScript case.installMethod case.imageName case.imageConfig) casesToRun
  )}
''
