{
  installScripts = [
    {
      name = "install-default";
      script = ''
        #!/bin/sh

        set -eux

        curl https://nixos.org/nix/install | sh
      '';
    }

    {
      name = "install-force-no-daemon";
      script = ''
        #!/bin/sh

        set -eux

        curl https://nixos.org/nix/install | sh -s -- --no-daemon
      '';
    }

    {
      name = "install-force-daemon";
      script = ''
        #!/bin/sh

        set -eux

        curl https://nixos.org/nix/install | sh -s -- --daemon
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
      preInstall = "";
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

    "freebsd-11" = {
      image = "generic/freebsd11";
      preInstall = ''
        pkg install curl
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

    "debian-7" = {
      image = "debian/wheezy64";
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

    "ubuntu-17-10" = {
      image = "ubuntu/artful64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };

    "ubuntu-17-04" = {
      image = "ubuntu/zesty64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };
    "ubuntu-16-10" = {
      image = "ubuntu/yakkety64";
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
    "ubuntu-15-10" = {
      image = "ubuntu/wily64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };

    "ubuntu-15-04" = {
      image = "ubuntu/vivid64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
      '';
    };
    "ubuntu-14-10" = {
      image = "ubuntu/utopic64";
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
