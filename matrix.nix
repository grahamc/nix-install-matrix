{
  installScripts = let
    installUrls = {
      pre = "https://nixos.org/releases/nix/nix-2.1pre6385_d16ff76c/install";
      stable = "https://nixos.org/nix/install";
      "2.0.4" = "https://nixos.org/releases/nix/nix-2.0.4/install";
      "2.1.3" = "https://nixos.org/releases/nix/nix-2.1.3/install";
    };
    installUrl = installUrls."2.1.3";
  in [
    {
      name = "install-default";
      script = ''
        #!/bin/sh

        set -eux

        curl "${installUrl}" | sh
      '';
    }

    {
      name = "install-force-no-daemon";
      script = ''
        #!/bin/sh

        set -eux

        curl "${installUrl}" | sh -s -- --no-daemon
      '';
    }

    {
      name = "install-force-daemon";
      script = ''
        #!/bin/sh

        set -eux

        curl "${installUrl}" | sh -s -- --daemon
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

    "arch" = {
      image = "generic/arch";
      preInstall = ''
        packman -S --no-confirm rsync
      '';
    };

    "alpine-3-8" = {
      image = "generic/alpine38";
      preInstall = ''
        apk --no-cache add curl
      '';
    };

    "alpine-3-7" = {
      image = "generic/alpine37";
      preInstall = ''
        apk --no-cache add curl
      '';
    };

    "alpine-3-6" = {
      image = "generic/alpine36";
      preInstall = ''
        apk --no-cache add curl
      '';
    };

    "alpine-3-5" = {
      image = "generic/alpine35";
      preInstall = ''
        apk --no-cache add curl
      '';
    };

    "fedora-28" = {
      image = "generic/fedora28";
      preInstall = ''
        yum install curl
      '';
    };

    "fedora-27" = {
      image = "generic/fedora27";
      preInstall = ''
        yum install curl
      '';
    };

    "fedora-26" = {
      image = "generic/fedora26";
      preInstall = ''
        yum install curl
      '';
    };

    "fedora-25" = {
      image = "generic/fedora25";
      preInstall = ''
        yum install curl
      '';
    };

    "gentoo" = {
      image = "generic/gentoo";
      preInstall = ''
        emerge curl
      '';
    };

    "centos-7" = {
      image = "centos/7";
      preInstall = ''
        yum install curl
      '';
    };

    "centos-6" = {
      image = "centos/6";
      preInstall = ''
        yum install curl
      '';
    };

    "debian-9" = {
      image = "debian/stretch64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };

    "debian-8" = {
      image = "debian/jessie64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };

    "ubuntu-18-10" = {
      image = "ubuntu/cosmic64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };

    "ubuntu-18-04" = {
      image = "ubuntu/bionic64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };

    "ubuntu-16-04" = {
      image = "ubuntu/xenial64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };

    "ubuntu-14-04" = {
      image = "ubuntu/trusty64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };

    "ubuntu-12-04" = {
      image = "ubuntu/precise64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };
  };
}
