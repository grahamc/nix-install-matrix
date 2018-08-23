{
  installScripts = [
    {
      name = "install-default";
      script = ''
        #!/bin/sh

        set -eux

        curl https://binrepo.target.com/artifactory/zebral-local/target-nix-upstaller-beta | bash
      '';
    }
  ];

  images = {
    "macos-sierra" = {
      # Sketchy :)
      image = "jhcook/macos-sierra";
      preInstall = "";
    };

    "macos-highsierra" = {
      # Sketchy :)
      image = "monsenso/macos-10.13";
      preInstall = "";
    };

    "macos-highsierra-upgrade-simulation" = {
      # Sketchy :)
      image = "monsenso/macos-10.13";
      preInstallUnprivileged = "
        curl https://binrepo.target.com/artifactory/zebral-local/target-nix-upstaller-beta | bash
        sudo cp /etc/bashrc /etc/bashrc~previous
        sudo cp /etc/bashrc.backup-before-nix /etc/bashrc

      ";
    };
  };
}
