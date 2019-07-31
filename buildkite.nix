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

  rawjson = pkgs.writeText "buildkite.json"
(builtins.toJSON
{ steps = (

  (builtins.map (case:
  let cmd = mkTestScript case.installMethod case.imageName case.imageConfig;
  in {
    label = "${case.imageName}: ${case.installMethod.name}";
    command = [
      "echo $HOME"
      "rm -rf ./output"
      "mkdir ./output"
      "nix-build ./test-script.nix --argstr imageNameFilter '${case.imageName}' --argstr installMethodFilter '${case.installMethod.name}'"
      "./result ./output"
      "tar -C output -czf ${case.installMethod.name}-${case.imageName}.tar.gz ${case.installMethod.name}-${case.imageName}"
    ];
    agents = {
      nix-install-matrix = true;
    };
    artifact_paths = [
      "${case.installMethod.name}-${case.imageName}.tar.gz"
    ];
  }) casesToRun
  )
  ++ [
    {
      wait = "~";
      continue_on_failure = true;
    }
    {
      label = "report";
      command =
        [
          "rm -rf output"
          "mkdir output"
          "echo '--- Fetching artifacts'"
        ]
        ++ (builtins.map (case:
          "buildkite-agent artifact download ${case.installMethod.name}-${case.imageName}.tar.gz output/ || true"
        ) casesToRun)
        ++ [
          "echo '--- Extracting artifacts'"
        ]
        ++ (builtins.map (case:
          "(cd output && tar -xf ${case.installMethod.name}-${case.imageName}.tar.gz) || true"
        )  casesToRun)
        ++ [
          "echo '--- Building report tool'"
          "nix-build ./nix-install-matrix-tools/"
          "echo '--- Building report'"
          "./result/bin/treeport --input ./output --output ./report.html"
        ];
        artifact_paths = [
          "./report.html"
        ];
    }
  ]);
});

in pkgs.runCommand "buildkite.pretty.json" { buildInputs = [ pkgs.jq ]; }
''
  jq .  ${rawjson} > $out
''
