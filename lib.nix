{ installMethodFilter ? null, imageNameFilter ? null }:
rec {
  pkgs = import <nixpkgs> {};
  srcs = import ./matrix.nix;

  shellcheckedScript = name: text:
    pkgs.runCommand "shellchecked-${name}" {
      src = pkgs.writeScript name text;
      buildInputs = with pkgs; [ shellcheck ];
    } ''
      shellcheck $src
      cp $src $out
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
      filteredImages;

  filteredImages = (pkgs.lib.filterAttrs
    (name: _: if imageNameFilter == null then true
    else name == imageNameFilter) srcs.images);

}
