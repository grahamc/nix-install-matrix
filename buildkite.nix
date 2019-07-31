{ installMethodFilter ? null, imageNameFilter ? null }:
let
  lib = import ./lib.nix {
    inherit installMethodFilter imageNameFilter;
  };

  inherit (lib) pkgs shellcheckedScript filteredImages
    casesToRun;

  rawjson = pkgs.writeText "buildkite.json"
(builtins.toJSON
{ steps = (
  [
    {
      label = "build Nix";
      command = [
        "rm -rf ./nix-co"
        ''git clone --branch="$GIT_BRANCH" "$GIT_URL" ./nix-co''
        "cd ./nix-co"
        ''nix-build ./release.nix -A "binaryTarball.x86_64-linux"''
        "cp ./result/nix-*.tar.bz2 ./nix.x86_64-linux.tar.bz2"
      ];
      artifact_paths = [
        "nix.*.tar.bz2"
      ];
    }
    {
      wait = "~";
      continue_on_failure = false;
    }
  ]
  ++ (builtins.map (case:
  {
    label = "${case.imageName}: ${case.installMethod.name}";
    command = [
      "echo $HOME"
      "buildkite-agent artifact download nix.${case.imageConfig.system}.tar.bz2"
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
