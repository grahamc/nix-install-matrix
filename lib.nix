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

  squidConfig = pkgs.writeText "squid.conf"
    ''
      #
      # Recommended minimum configuration (3.5):
      #
      # Example rule allowing access from your local networks.
      # Adapt to list your (internal) IP networks from where browsing
      # should be allowed
      acl localnet src 10.0.0.0/8     # RFC 1918 possible internal network
      acl localnet src 172.16.0.0/12  # RFC 1918 possible internal network
      acl localnet src 192.168.0.0/16 # RFC 1918 possible internal network
      acl localnet src 169.254.0.0/16 # RFC 3927 link-local (directly plugged) machines
      acl localnet src fc00::/7       # RFC 4193 local private network range
      acl localnet src fe80::/10      # RFC 4291 link-local (directly plugged) machines
      acl SSL_ports port 443          # https
      acl Safe_ports port 80          # http
      acl Safe_ports port 21          # ftp
      acl Safe_ports port 443         # https
      acl Safe_ports port 70          # gopher
      acl Safe_ports port 210         # wais
      acl Safe_ports port 1025-65535  # unregistered ports
      acl Safe_ports port 280         # http-mgmt
      acl Safe_ports port 488         # gss-http
      acl Safe_ports port 591         # filemaker
      acl Safe_ports port 777         # multiling http
      acl CONNECT method CONNECT
      #
      # Recommended minimum Access Permission configuration:
      #
      # Deny requests to certain unsafe ports
      http_access deny !Safe_ports
      # Deny CONNECT to other than secure SSL ports
      http_access deny CONNECT !SSL_ports
      # Only allow cachemgr access from localhost
      http_access allow localhost manager
      http_access deny manager
      http_port 3128
      # We strongly recommend the following be uncommented to protect innocent
      # web applications running on the proxy server who think the only
      # one who can access services on "localhost" is a local user
      http_access deny to_localhost
      # Application logs to syslog, access and store logs have specific files
      cache_log       stdio:/dev/stdout
      access_log      stdio:/dev/stdout
      cache_store_log stdio:/dev/stdout

      pid_filename    ./squid.pid
      #
      # INSERT YOUR OWN RULE(S) HERE TO ALLOW ACCESS FROM YOUR CLIENTS
      #
      # !!!
      #
      # Example rule allowing access from your local networks.
      # Adapt localnet in the ACL section to list your (internal) IP networks
      # from where browsing should be allowed
      http_access allow localnet
      http_access allow localhost
      # And finally deny all other access to this proxy
      http_access deny all
      # Squid normally listens to port 3128
      # Leave coredumps in the first cache dir
      coredump_dir ./squid-state
      #
      # Add any of your own refresh_pattern entries above these.
      #
      refresh_pattern ^ftp:           1440    20%     10080
      refresh_pattern ^gopher:        1440    0%      1440
      refresh_pattern -i (/cgi-bin/|\?) 0     0%      0
      refresh_pattern .               0       20%     4320
    '';


}
