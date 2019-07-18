{ lib, buildRustCrate, buildRustCrateHelpers }:
with buildRustCrateHelpers;
let inherit (lib.lists) fold;
    inherit (lib.attrsets) recursiveUpdate;
in
rec {

# aho-corasick-0.6.6

  crates.aho_corasick."0.6.6" = deps: { features?(features_.aho_corasick."0.6.6" deps {}) }: buildRustCrate {
    crateName = "aho-corasick";
    version = "0.6.6";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "0ap5lv1q6ylmzq70bjgg66dsa6p9926gwv2q4z0chfjnii8hczq8";
    libName = "aho_corasick";
    crateBin =
      [{  name = "aho-corasick-dot";  path = "src/main.rs"; }];
    dependencies = mapFeatures features ([
      (crates."memchr"."${deps."aho_corasick"."0.6.6"."memchr"}" deps)
    ]);
  };
  features_.aho_corasick."0.6.6" = deps: f: updateFeatures f (rec {
    aho_corasick."0.6.6".default = (f.aho_corasick."0.6.6".default or true);
    memchr."${deps.aho_corasick."0.6.6".memchr}".default = true;
  }) [
    (features_.memchr."${deps."aho_corasick"."0.6.6"."memchr"}" deps)
  ];


# end
# ansi_term-0.11.0

  crates.ansi_term."0.11.0" = deps: { features?(features_.ansi_term."0.11.0" deps {}) }: buildRustCrate {
    crateName = "ansi_term";
    version = "0.11.0";
    authors = [ "ogham@bsago.me" "Ryan Scheel (Havvy) <ryan.havvy@gmail.com>" "Josh Triplett <josh@joshtriplett.org>" ];
    sha256 = "08fk0p2xvkqpmz3zlrwnf6l8sj2vngw464rvzspzp31sbgxbwm4v";
    dependencies = (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."ansi_term"."0.11.0"."winapi"}" deps)
    ]) else []);
  };
  features_.ansi_term."0.11.0" = deps: f: updateFeatures f (rec {
    ansi_term."0.11.0".default = (f.ansi_term."0.11.0".default or true);
    winapi = fold recursiveUpdate {} [
      { "${deps.ansi_term."0.11.0".winapi}"."consoleapi" = true; }
      { "${deps.ansi_term."0.11.0".winapi}"."errhandlingapi" = true; }
      { "${deps.ansi_term."0.11.0".winapi}"."processenv" = true; }
      { "${deps.ansi_term."0.11.0".winapi}".default = true; }
    ];
  }) [
    (features_.winapi."${deps."ansi_term"."0.11.0"."winapi"}" deps)
  ];


# end
# atty-0.2.11

  crates.atty."0.2.11" = deps: { features?(features_.atty."0.2.11" deps {}) }: buildRustCrate {
    crateName = "atty";
    version = "0.2.11";
    authors = [ "softprops <d.tangren@gmail.com>" ];
    sha256 = "0by1bj2km9jxi4i4g76zzi76fc2rcm9934jpnyrqd95zw344pb20";
    dependencies = (if kernel == "redox" then mapFeatures features ([
      (crates."termion"."${deps."atty"."0.2.11"."termion"}" deps)
    ]) else [])
      ++ (if (kernel == "linux" || kernel == "darwin") then mapFeatures features ([
      (crates."libc"."${deps."atty"."0.2.11"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "windows" then mapFeatures features ([
      (crates."winapi"."${deps."atty"."0.2.11"."winapi"}" deps)
    ]) else []);
  };
  features_.atty."0.2.11" = deps: f: updateFeatures f (rec {
    atty."0.2.11".default = (f.atty."0.2.11".default or true);
    libc."${deps.atty."0.2.11".libc}".default = (f.libc."${deps.atty."0.2.11".libc}".default or false);
    termion."${deps.atty."0.2.11".termion}".default = true;
    winapi = fold recursiveUpdate {} [
      { "${deps.atty."0.2.11".winapi}"."consoleapi" = true; }
      { "${deps.atty."0.2.11".winapi}"."minwinbase" = true; }
      { "${deps.atty."0.2.11".winapi}"."minwindef" = true; }
      { "${deps.atty."0.2.11".winapi}"."processenv" = true; }
      { "${deps.atty."0.2.11".winapi}"."winbase" = true; }
      { "${deps.atty."0.2.11".winapi}".default = true; }
    ];
  }) [
    (features_.termion."${deps."atty"."0.2.11"."termion"}" deps)
    (features_.libc."${deps."atty"."0.2.11"."libc"}" deps)
    (features_.winapi."${deps."atty"."0.2.11"."winapi"}" deps)
  ];


# end
# bitflags-1.0.4

  crates.bitflags."1.0.4" = deps: { features?(features_.bitflags."1.0.4" deps {}) }: buildRustCrate {
    crateName = "bitflags";
    version = "1.0.4";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1g1wmz2001qmfrd37dnd5qiss5njrw26aywmg6yhkmkbyrhjxb08";
    features = mkFeatures (features."bitflags"."1.0.4" or {});
  };
  features_.bitflags."1.0.4" = deps: f: updateFeatures f (rec {
    bitflags."1.0.4".default = (f.bitflags."1.0.4".default or true);
  }) [];


# end
# clap-2.32.0

  crates.clap."2.32.0" = deps: { features?(features_.clap."2.32.0" deps {}) }: buildRustCrate {
    crateName = "clap";
    version = "2.32.0";
    authors = [ "Kevin K. <kbknapp@gmail.com>" ];
    sha256 = "1hdjf0janvpjkwrjdjx1mm2aayzr54k72w6mriyr0n5anjkcj1lx";
    dependencies = mapFeatures features ([
      (crates."bitflags"."${deps."clap"."2.32.0"."bitflags"}" deps)
      (crates."textwrap"."${deps."clap"."2.32.0"."textwrap"}" deps)
      (crates."unicode_width"."${deps."clap"."2.32.0"."unicode_width"}" deps)
    ]
      ++ (if features.clap."2.32.0".atty or false then [ (crates.atty."${deps."clap"."2.32.0".atty}" deps) ] else [])
      ++ (if features.clap."2.32.0".strsim or false then [ (crates.strsim."${deps."clap"."2.32.0".strsim}" deps) ] else [])
      ++ (if features.clap."2.32.0".vec_map or false then [ (crates.vec_map."${deps."clap"."2.32.0".vec_map}" deps) ] else []))
      ++ (if !(kernel == "windows") then mapFeatures features ([
    ]
      ++ (if features.clap."2.32.0".ansi_term or false then [ (crates.ansi_term."${deps."clap"."2.32.0".ansi_term}" deps) ] else [])) else []);
    features = mkFeatures (features."clap"."2.32.0" or {});
  };
  features_.clap."2.32.0" = deps: f: updateFeatures f (rec {
    ansi_term."${deps.clap."2.32.0".ansi_term}".default = true;
    atty."${deps.clap."2.32.0".atty}".default = true;
    bitflags."${deps.clap."2.32.0".bitflags}".default = true;
    clap = fold recursiveUpdate {} [
      { "2.32.0".ansi_term =
        (f.clap."2.32.0".ansi_term or false) ||
        (f.clap."2.32.0".color or false) ||
        (clap."2.32.0"."color" or false); }
      { "2.32.0".atty =
        (f.clap."2.32.0".atty or false) ||
        (f.clap."2.32.0".color or false) ||
        (clap."2.32.0"."color" or false); }
      { "2.32.0".clippy =
        (f.clap."2.32.0".clippy or false) ||
        (f.clap."2.32.0".lints or false) ||
        (clap."2.32.0"."lints" or false); }
      { "2.32.0".color =
        (f.clap."2.32.0".color or false) ||
        (f.clap."2.32.0".default or false) ||
        (clap."2.32.0"."default" or false); }
      { "2.32.0".default = (f.clap."2.32.0".default or true); }
      { "2.32.0".strsim =
        (f.clap."2.32.0".strsim or false) ||
        (f.clap."2.32.0".suggestions or false) ||
        (clap."2.32.0"."suggestions" or false); }
      { "2.32.0".suggestions =
        (f.clap."2.32.0".suggestions or false) ||
        (f.clap."2.32.0".default or false) ||
        (clap."2.32.0"."default" or false); }
      { "2.32.0".term_size =
        (f.clap."2.32.0".term_size or false) ||
        (f.clap."2.32.0".wrap_help or false) ||
        (clap."2.32.0"."wrap_help" or false); }
      { "2.32.0".vec_map =
        (f.clap."2.32.0".vec_map or false) ||
        (f.clap."2.32.0".default or false) ||
        (clap."2.32.0"."default" or false); }
      { "2.32.0".yaml =
        (f.clap."2.32.0".yaml or false) ||
        (f.clap."2.32.0".doc or false) ||
        (clap."2.32.0"."doc" or false); }
      { "2.32.0".yaml-rust =
        (f.clap."2.32.0".yaml-rust or false) ||
        (f.clap."2.32.0".yaml or false) ||
        (clap."2.32.0"."yaml" or false); }
    ];
    strsim."${deps.clap."2.32.0".strsim}".default = true;
    textwrap = fold recursiveUpdate {} [
      { "${deps.clap."2.32.0".textwrap}"."term_size" =
        (f.textwrap."${deps.clap."2.32.0".textwrap}"."term_size" or false) ||
        (clap."2.32.0"."wrap_help" or false) ||
        (f."clap"."2.32.0"."wrap_help" or false); }
      { "${deps.clap."2.32.0".textwrap}".default = true; }
    ];
    unicode_width."${deps.clap."2.32.0".unicode_width}".default = true;
    vec_map."${deps.clap."2.32.0".vec_map}".default = true;
  }) [
    (features_.atty."${deps."clap"."2.32.0"."atty"}" deps)
    (features_.bitflags."${deps."clap"."2.32.0"."bitflags"}" deps)
    (features_.strsim."${deps."clap"."2.32.0"."strsim"}" deps)
    (features_.textwrap."${deps."clap"."2.32.0"."textwrap"}" deps)
    (features_.unicode_width."${deps."clap"."2.32.0"."unicode_width"}" deps)
    (features_.vec_map."${deps."clap"."2.32.0"."vec_map"}" deps)
    (features_.ansi_term."${deps."clap"."2.32.0"."ansi_term"}" deps)
  ];


# end
# lazy_static-1.1.0

  crates.lazy_static."1.1.0" = deps: { features?(features_.lazy_static."1.1.0" deps {}) }: buildRustCrate {
    crateName = "lazy_static";
    version = "1.1.0";
    authors = [ "Marvin Löbel <loebel.marvin@gmail.com>" ];
    sha256 = "1da2b6nxfc2l547qgl9kd1pn9sh1af96a6qx6xw8xdnv6hh5fag0";
    build = "build.rs";
    dependencies = mapFeatures features ([
]);

    buildDependencies = mapFeatures features ([
      (crates."version_check"."${deps."lazy_static"."1.1.0"."version_check"}" deps)
    ]);
    features = mkFeatures (features."lazy_static"."1.1.0" or {});
  };
  features_.lazy_static."1.1.0" = deps: f: updateFeatures f (rec {
    lazy_static = fold recursiveUpdate {} [
      { "1.1.0".default = (f.lazy_static."1.1.0".default or true); }
      { "1.1.0".nightly =
        (f.lazy_static."1.1.0".nightly or false) ||
        (f.lazy_static."1.1.0".spin_no_std or false) ||
        (lazy_static."1.1.0"."spin_no_std" or false); }
      { "1.1.0".spin =
        (f.lazy_static."1.1.0".spin or false) ||
        (f.lazy_static."1.1.0".spin_no_std or false) ||
        (lazy_static."1.1.0"."spin_no_std" or false); }
    ];
    version_check."${deps.lazy_static."1.1.0".version_check}".default = true;
  }) [
    (features_.version_check."${deps."lazy_static"."1.1.0"."version_check"}" deps)
  ];


# end
# libc-0.2.43

  crates.libc."0.2.43" = deps: { features?(features_.libc."0.2.43" deps {}) }: buildRustCrate {
    crateName = "libc";
    version = "0.2.43";
    authors = [ "The Rust Project Developers" ];
    sha256 = "0pshydmsq71kl9276zc2928ld50sp524ixcqkcqsgq410dx6c50b";
    features = mkFeatures (features."libc"."0.2.43" or {});
  };
  features_.libc."0.2.43" = deps: f: updateFeatures f (rec {
    libc = fold recursiveUpdate {} [
      { "0.2.43".default = (f.libc."0.2.43".default or true); }
      { "0.2.43".use_std =
        (f.libc."0.2.43".use_std or false) ||
        (f.libc."0.2.43".default or false) ||
        (libc."0.2.43"."default" or false); }
    ];
  }) [];


# end
# memchr-2.0.1

  crates.memchr."2.0.1" = deps: { features?(features_.memchr."2.0.1" deps {}) }: buildRustCrate {
    crateName = "memchr";
    version = "2.0.1";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" "bluss" ];
    sha256 = "0ls2y47rjwapjdax6bp974gdp06ggm1v8d1h69wyydmh1nhgm5gr";
    dependencies = mapFeatures features ([
    ]
      ++ (if features.memchr."2.0.1".libc or false then [ (crates.libc."${deps."memchr"."2.0.1".libc}" deps) ] else []));
    features = mkFeatures (features."memchr"."2.0.1" or {});
  };
  features_.memchr."2.0.1" = deps: f: updateFeatures f (rec {
    libc = fold recursiveUpdate {} [
      { "${deps.memchr."2.0.1".libc}"."use_std" =
        (f.libc."${deps.memchr."2.0.1".libc}"."use_std" or false) ||
        (memchr."2.0.1"."use_std" or false) ||
        (f."memchr"."2.0.1"."use_std" or false); }
      { "${deps.memchr."2.0.1".libc}".default = (f.libc."${deps.memchr."2.0.1".libc}".default or false); }
    ];
    memchr = fold recursiveUpdate {} [
      { "2.0.1".default = (f.memchr."2.0.1".default or true); }
      { "2.0.1".libc =
        (f.memchr."2.0.1".libc or false) ||
        (f.memchr."2.0.1".default or false) ||
        (memchr."2.0.1"."default" or false) ||
        (f.memchr."2.0.1".use_std or false) ||
        (memchr."2.0.1"."use_std" or false); }
      { "2.0.1".use_std =
        (f.memchr."2.0.1".use_std or false) ||
        (f.memchr."2.0.1".default or false) ||
        (memchr."2.0.1"."default" or false); }
    ];
  }) [
    (features_.libc."${deps."memchr"."2.0.1"."libc"}" deps)
  ];


# end
# proc-macro2-0.4.13

  crates.proc_macro2."0.4.13" = deps: { features?(features_.proc_macro2."0.4.13" deps {}) }: buildRustCrate {
    crateName = "proc-macro2";
    version = "0.4.13";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" ];
    sha256 = "0w9h0jys14g032s5vlz22x83n4z17wcq5xgkcawyk90nvlzc667j";
    build = "build.rs";
    dependencies = mapFeatures features ([
      (crates."unicode_xid"."${deps."proc_macro2"."0.4.13"."unicode_xid"}" deps)
    ]);
    features = mkFeatures (features."proc_macro2"."0.4.13" or {});
  };
  features_.proc_macro2."0.4.13" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "0.4.13".default = (f.proc_macro2."0.4.13".default or true); }
      { "0.4.13".proc-macro =
        (f.proc_macro2."0.4.13".proc-macro or false) ||
        (f.proc_macro2."0.4.13".default or false) ||
        (proc_macro2."0.4.13"."default" or false) ||
        (f.proc_macro2."0.4.13".nightly or false) ||
        (proc_macro2."0.4.13"."nightly" or false); }
    ];
    unicode_xid."${deps.proc_macro2."0.4.13".unicode_xid}".default = true;
  }) [
    (features_.unicode_xid."${deps."proc_macro2"."0.4.13"."unicode_xid"}" deps)
  ];


# end
# quote-0.6.8

  crates.quote."0.6.8" = deps: { features?(features_.quote."0.6.8" deps {}) }: buildRustCrate {
    crateName = "quote";
    version = "0.6.8";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "0dq6j23w6pmc4l6v490arixdwypy0b82z76nrzaingqhqri4p3mh";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."quote"."0.6.8"."proc_macro2"}" deps)
    ]);
    features = mkFeatures (features."quote"."0.6.8" or {});
  };
  features_.quote."0.6.8" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.quote."0.6.8".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.quote."0.6.8".proc_macro2}"."proc-macro" or false) ||
        (quote."0.6.8"."proc-macro" or false) ||
        (f."quote"."0.6.8"."proc-macro" or false); }
      { "${deps.quote."0.6.8".proc_macro2}".default = (f.proc_macro2."${deps.quote."0.6.8".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "0.6.8".default = (f.quote."0.6.8".default or true); }
      { "0.6.8".proc-macro =
        (f.quote."0.6.8".proc-macro or false) ||
        (f.quote."0.6.8".default or false) ||
        (quote."0.6.8"."default" or false); }
    ];
  }) [
    (features_.proc_macro2."${deps."quote"."0.6.8"."proc_macro2"}" deps)
  ];


# end
# redox_syscall-0.1.40

  crates.redox_syscall."0.1.40" = deps: { features?(features_.redox_syscall."0.1.40" deps {}) }: buildRustCrate {
    crateName = "redox_syscall";
    version = "0.1.40";
    authors = [ "Jeremy Soller <jackpot51@gmail.com>" ];
    sha256 = "132rnhrq49l3z7gjrwj2zfadgw6q0355s6a7id7x7c0d7sk72611";
    libName = "syscall";
  };
  features_.redox_syscall."0.1.40" = deps: f: updateFeatures f (rec {
    redox_syscall."0.1.40".default = (f.redox_syscall."0.1.40".default or true);
  }) [];


# end
# redox_termios-0.1.1

  crates.redox_termios."0.1.1" = deps: { features?(features_.redox_termios."0.1.1" deps {}) }: buildRustCrate {
    crateName = "redox_termios";
    version = "0.1.1";
    authors = [ "Jeremy Soller <jackpot51@gmail.com>" ];
    sha256 = "04s6yyzjca552hdaqlvqhp3vw0zqbc304md5czyd3axh56iry8wh";
    libPath = "src/lib.rs";
    dependencies = mapFeatures features ([
      (crates."redox_syscall"."${deps."redox_termios"."0.1.1"."redox_syscall"}" deps)
    ]);
  };
  features_.redox_termios."0.1.1" = deps: f: updateFeatures f (rec {
    redox_syscall."${deps.redox_termios."0.1.1".redox_syscall}".default = true;
    redox_termios."0.1.1".default = (f.redox_termios."0.1.1".default or true);
  }) [
    (features_.redox_syscall."${deps."redox_termios"."0.1.1"."redox_syscall"}" deps)
  ];


# end
# regex-1.0.2

  crates.regex."1.0.2" = deps: { features?(features_.regex."1.0.2" deps {}) }: buildRustCrate {
    crateName = "regex";
    version = "1.0.2";
    authors = [ "The Rust Project Developers" ];
    sha256 = "1dakmqc994kbsbbkks6psfafwyh74zvsnzlw8vqz2yp958di9s0i";
    dependencies = mapFeatures features ([
      (crates."aho_corasick"."${deps."regex"."1.0.2"."aho_corasick"}" deps)
      (crates."memchr"."${deps."regex"."1.0.2"."memchr"}" deps)
      (crates."regex_syntax"."${deps."regex"."1.0.2"."regex_syntax"}" deps)
      (crates."thread_local"."${deps."regex"."1.0.2"."thread_local"}" deps)
      (crates."utf8_ranges"."${deps."regex"."1.0.2"."utf8_ranges"}" deps)
    ]);
    features = mkFeatures (features."regex"."1.0.2" or {});
  };
  features_.regex."1.0.2" = deps: f: updateFeatures f (rec {
    aho_corasick."${deps.regex."1.0.2".aho_corasick}".default = true;
    memchr."${deps.regex."1.0.2".memchr}".default = true;
    regex = fold recursiveUpdate {} [
      { "1.0.2".default = (f.regex."1.0.2".default or true); }
      { "1.0.2".pattern =
        (f.regex."1.0.2".pattern or false) ||
        (f.regex."1.0.2".unstable or false) ||
        (regex."1.0.2"."unstable" or false); }
      { "1.0.2".use_std =
        (f.regex."1.0.2".use_std or false) ||
        (f.regex."1.0.2".default or false) ||
        (regex."1.0.2"."default" or false); }
    ];
    regex_syntax."${deps.regex."1.0.2".regex_syntax}".default = true;
    thread_local."${deps.regex."1.0.2".thread_local}".default = true;
    utf8_ranges."${deps.regex."1.0.2".utf8_ranges}".default = true;
  }) [
    (features_.aho_corasick."${deps."regex"."1.0.2"."aho_corasick"}" deps)
    (features_.memchr."${deps."regex"."1.0.2"."memchr"}" deps)
    (features_.regex_syntax."${deps."regex"."1.0.2"."regex_syntax"}" deps)
    (features_.thread_local."${deps."regex"."1.0.2"."thread_local"}" deps)
    (features_.utf8_ranges."${deps."regex"."1.0.2"."utf8_ranges"}" deps)
  ];


# end
# regex-syntax-0.6.2

  crates.regex_syntax."0.6.2" = deps: { features?(features_.regex_syntax."0.6.2" deps {}) }: buildRustCrate {
    crateName = "regex-syntax";
    version = "0.6.2";
    authors = [ "The Rust Project Developers" ];
    sha256 = "109426mj7nhwr6szdzbcvn1a8g5zy52f9maqxjd9agm8wg87ylyw";
    dependencies = mapFeatures features ([
      (crates."ucd_util"."${deps."regex_syntax"."0.6.2"."ucd_util"}" deps)
    ]);
  };
  features_.regex_syntax."0.6.2" = deps: f: updateFeatures f (rec {
    regex_syntax."0.6.2".default = (f.regex_syntax."0.6.2".default or true);
    ucd_util."${deps.regex_syntax."0.6.2".ucd_util}".default = true;
  }) [
    (features_.ucd_util."${deps."regex_syntax"."0.6.2"."ucd_util"}" deps)
  ];


# end
# strsim-0.7.0

  crates.strsim."0.7.0" = deps: { features?(features_.strsim."0.7.0" deps {}) }: buildRustCrate {
    crateName = "strsim";
    version = "0.7.0";
    authors = [ "Danny Guo <dannyguo91@gmail.com>" ];
    sha256 = "0fy0k5f2705z73mb3x9459bpcvrx4ky8jpr4zikcbiwan4bnm0iv";
  };
  features_.strsim."0.7.0" = deps: f: updateFeatures f (rec {
    strsim."0.7.0".default = (f.strsim."0.7.0".default or true);
  }) [];


# end
# structopt-0.2.10

  crates.structopt."0.2.10" = deps: { features?(features_.structopt."0.2.10" deps {}) }: buildRustCrate {
    crateName = "structopt";
    version = "0.2.10";
    authors = [ "Guillaume Pinot <texitoi@texitoi.eu>" "others" ];
    sha256 = "0bnhmx7i23a65vn0wp0rrll0rxlznlnia5kn20rip2870agmjfm8";
    dependencies = mapFeatures features ([
      (crates."clap"."${deps."structopt"."0.2.10"."clap"}" deps)
      (crates."structopt_derive"."${deps."structopt"."0.2.10"."structopt_derive"}" deps)
    ]);
    features = mkFeatures (features."structopt"."0.2.10" or {});
  };
  features_.structopt."0.2.10" = deps: f: updateFeatures f (rec {
    clap = fold recursiveUpdate {} [
      { "${deps.structopt."0.2.10".clap}"."color" =
        (f.clap."${deps.structopt."0.2.10".clap}"."color" or false) ||
        (structopt."0.2.10"."color" or false) ||
        (f."structopt"."0.2.10"."color" or false); }
      { "${deps.structopt."0.2.10".clap}"."debug" =
        (f.clap."${deps.structopt."0.2.10".clap}"."debug" or false) ||
        (structopt."0.2.10"."debug" or false) ||
        (f."structopt"."0.2.10"."debug" or false); }
      { "${deps.structopt."0.2.10".clap}"."default" =
        (f.clap."${deps.structopt."0.2.10".clap}"."default" or false) ||
        (structopt."0.2.10"."default" or false) ||
        (f."structopt"."0.2.10"."default" or false); }
      { "${deps.structopt."0.2.10".clap}"."doc" =
        (f.clap."${deps.structopt."0.2.10".clap}"."doc" or false) ||
        (structopt."0.2.10"."doc" or false) ||
        (f."structopt"."0.2.10"."doc" or false); }
      { "${deps.structopt."0.2.10".clap}"."lints" =
        (f.clap."${deps.structopt."0.2.10".clap}"."lints" or false) ||
        (structopt."0.2.10"."lints" or false) ||
        (f."structopt"."0.2.10"."lints" or false); }
      { "${deps.structopt."0.2.10".clap}"."no_cargo" =
        (f.clap."${deps.structopt."0.2.10".clap}"."no_cargo" or false) ||
        (structopt."0.2.10"."no_cargo" or false) ||
        (f."structopt"."0.2.10"."no_cargo" or false); }
      { "${deps.structopt."0.2.10".clap}"."suggestions" =
        (f.clap."${deps.structopt."0.2.10".clap}"."suggestions" or false) ||
        (structopt."0.2.10"."suggestions" or false) ||
        (f."structopt"."0.2.10"."suggestions" or false); }
      { "${deps.structopt."0.2.10".clap}"."wrap_help" =
        (f.clap."${deps.structopt."0.2.10".clap}"."wrap_help" or false) ||
        (structopt."0.2.10"."wrap_help" or false) ||
        (f."structopt"."0.2.10"."wrap_help" or false); }
      { "${deps.structopt."0.2.10".clap}"."yaml" =
        (f.clap."${deps.structopt."0.2.10".clap}"."yaml" or false) ||
        (structopt."0.2.10"."yaml" or false) ||
        (f."structopt"."0.2.10"."yaml" or false); }
      { "${deps.structopt."0.2.10".clap}".default = (f.clap."${deps.structopt."0.2.10".clap}".default or false); }
    ];
    structopt."0.2.10".default = (f.structopt."0.2.10".default or true);
    structopt_derive = fold recursiveUpdate {} [
      { "${deps.structopt."0.2.10".structopt_derive}"."nightly" =
        (f.structopt_derive."${deps.structopt."0.2.10".structopt_derive}"."nightly" or false) ||
        (structopt."0.2.10"."nightly" or false) ||
        (f."structopt"."0.2.10"."nightly" or false); }
      { "${deps.structopt."0.2.10".structopt_derive}".default = true; }
    ];
  }) [
    (features_.clap."${deps."structopt"."0.2.10"."clap"}" deps)
    (features_.structopt_derive."${deps."structopt"."0.2.10"."structopt_derive"}" deps)
  ];


# end
# structopt-derive-0.2.10

  crates.structopt_derive."0.2.10" = deps: { features?(features_.structopt_derive."0.2.10" deps {}) }: buildRustCrate {
    crateName = "structopt-derive";
    version = "0.2.10";
    authors = [ "Guillaume Pinot <texitoi@texitoi.eu>" ];
    sha256 = "1sck1szr077c2sb7ri896gyhycicbwzi2x7yx3zmy6r1m42l39n0";
    procMacro = true;
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."structopt_derive"."0.2.10"."proc_macro2"}" deps)
      (crates."quote"."${deps."structopt_derive"."0.2.10"."quote"}" deps)
      (crates."syn"."${deps."structopt_derive"."0.2.10"."syn"}" deps)
    ]);
    features = mkFeatures (features."structopt_derive"."0.2.10" or {});
  };
  features_.structopt_derive."0.2.10" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.structopt_derive."0.2.10".proc_macro2}"."nightly" =
        (f.proc_macro2."${deps.structopt_derive."0.2.10".proc_macro2}"."nightly" or false) ||
        (structopt_derive."0.2.10"."nightly" or false) ||
        (f."structopt_derive"."0.2.10"."nightly" or false); }
      { "${deps.structopt_derive."0.2.10".proc_macro2}".default = true; }
    ];
    quote."${deps.structopt_derive."0.2.10".quote}".default = true;
    structopt_derive."0.2.10".default = (f.structopt_derive."0.2.10".default or true);
    syn."${deps.structopt_derive."0.2.10".syn}".default = true;
  }) [
    (features_.proc_macro2."${deps."structopt_derive"."0.2.10"."proc_macro2"}" deps)
    (features_.quote."${deps."structopt_derive"."0.2.10"."quote"}" deps)
    (features_.syn."${deps."structopt_derive"."0.2.10"."syn"}" deps)
  ];


# end
# syn-0.14.9

  crates.syn."0.14.9" = deps: { features?(features_.syn."0.14.9" deps {}) }: buildRustCrate {
    crateName = "syn";
    version = "0.14.9";
    authors = [ "David Tolnay <dtolnay@gmail.com>" ];
    sha256 = "1ia0qbrnqz40s8886b2jpcjiqfbziigd96lqjfin06xk6i28vr7b";
    dependencies = mapFeatures features ([
      (crates."proc_macro2"."${deps."syn"."0.14.9"."proc_macro2"}" deps)
      (crates."unicode_xid"."${deps."syn"."0.14.9"."unicode_xid"}" deps)
    ]
      ++ (if features.syn."0.14.9".quote or false then [ (crates.quote."${deps."syn"."0.14.9".quote}" deps) ] else []));
    features = mkFeatures (features."syn"."0.14.9" or {});
  };
  features_.syn."0.14.9" = deps: f: updateFeatures f (rec {
    proc_macro2 = fold recursiveUpdate {} [
      { "${deps.syn."0.14.9".proc_macro2}"."proc-macro" =
        (f.proc_macro2."${deps.syn."0.14.9".proc_macro2}"."proc-macro" or false) ||
        (syn."0.14.9"."proc-macro" or false) ||
        (f."syn"."0.14.9"."proc-macro" or false); }
      { "${deps.syn."0.14.9".proc_macro2}".default = (f.proc_macro2."${deps.syn."0.14.9".proc_macro2}".default or false); }
    ];
    quote = fold recursiveUpdate {} [
      { "${deps.syn."0.14.9".quote}"."proc-macro" =
        (f.quote."${deps.syn."0.14.9".quote}"."proc-macro" or false) ||
        (syn."0.14.9"."proc-macro" or false) ||
        (f."syn"."0.14.9"."proc-macro" or false); }
      { "${deps.syn."0.14.9".quote}".default = (f.quote."${deps.syn."0.14.9".quote}".default or false); }
    ];
    syn = fold recursiveUpdate {} [
      { "0.14.9".clone-impls =
        (f.syn."0.14.9".clone-impls or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".default = (f.syn."0.14.9".default or true); }
      { "0.14.9".derive =
        (f.syn."0.14.9".derive or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".parsing =
        (f.syn."0.14.9".parsing or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".printing =
        (f.syn."0.14.9".printing or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".proc-macro =
        (f.syn."0.14.9".proc-macro or false) ||
        (f.syn."0.14.9".default or false) ||
        (syn."0.14.9"."default" or false); }
      { "0.14.9".quote =
        (f.syn."0.14.9".quote or false) ||
        (f.syn."0.14.9".printing or false) ||
        (syn."0.14.9"."printing" or false); }
    ];
    unicode_xid."${deps.syn."0.14.9".unicode_xid}".default = true;
  }) [
    (features_.proc_macro2."${deps."syn"."0.14.9"."proc_macro2"}" deps)
    (features_.quote."${deps."syn"."0.14.9"."quote"}" deps)
    (features_.unicode_xid."${deps."syn"."0.14.9"."unicode_xid"}" deps)
  ];


# end
# termion-1.5.1

  crates.termion."1.5.1" = deps: { features?(features_.termion."1.5.1" deps {}) }: buildRustCrate {
    crateName = "termion";
    version = "1.5.1";
    authors = [ "ticki <Ticki@users.noreply.github.com>" "gycos <alexandre.bury@gmail.com>" "IGI-111 <igi-111@protonmail.com>" ];
    sha256 = "02gq4vd8iws1f3gjrgrgpajsk2bk43nds5acbbb4s8dvrdvr8nf1";
    dependencies = (if !(kernel == "redox") then mapFeatures features ([
      (crates."libc"."${deps."termion"."1.5.1"."libc"}" deps)
    ]) else [])
      ++ (if kernel == "redox" then mapFeatures features ([
      (crates."redox_syscall"."${deps."termion"."1.5.1"."redox_syscall"}" deps)
      (crates."redox_termios"."${deps."termion"."1.5.1"."redox_termios"}" deps)
    ]) else []);
  };
  features_.termion."1.5.1" = deps: f: updateFeatures f (rec {
    libc."${deps.termion."1.5.1".libc}".default = true;
    redox_syscall."${deps.termion."1.5.1".redox_syscall}".default = true;
    redox_termios."${deps.termion."1.5.1".redox_termios}".default = true;
    termion."1.5.1".default = (f.termion."1.5.1".default or true);
  }) [
    (features_.libc."${deps."termion"."1.5.1"."libc"}" deps)
    (features_.redox_syscall."${deps."termion"."1.5.1"."redox_syscall"}" deps)
    (features_.redox_termios."${deps."termion"."1.5.1"."redox_termios"}" deps)
  ];


# end
# textwrap-0.10.0

  crates.textwrap."0.10.0" = deps: { features?(features_.textwrap."0.10.0" deps {}) }: buildRustCrate {
    crateName = "textwrap";
    version = "0.10.0";
    authors = [ "Martin Geisler <martin@geisler.net>" ];
    sha256 = "1s8d5cna12smhgj0x2y1xphklyk2an1yzbadnj89p1vy5vnjpsas";
    dependencies = mapFeatures features ([
      (crates."unicode_width"."${deps."textwrap"."0.10.0"."unicode_width"}" deps)
    ]);
  };
  features_.textwrap."0.10.0" = deps: f: updateFeatures f (rec {
    textwrap."0.10.0".default = (f.textwrap."0.10.0".default or true);
    unicode_width."${deps.textwrap."0.10.0".unicode_width}".default = true;
  }) [
    (features_.unicode_width."${deps."textwrap"."0.10.0"."unicode_width"}" deps)
  ];


# end
# thread_local-0.3.6

  crates.thread_local."0.3.6" = deps: { features?(features_.thread_local."0.3.6" deps {}) }: buildRustCrate {
    crateName = "thread_local";
    version = "0.3.6";
    authors = [ "Amanieu d'Antras <amanieu@gmail.com>" ];
    sha256 = "02rksdwjmz2pw9bmgbb4c0bgkbq5z6nvg510sq1s6y2j1gam0c7i";
    dependencies = mapFeatures features ([
      (crates."lazy_static"."${deps."thread_local"."0.3.6"."lazy_static"}" deps)
    ]);
  };
  features_.thread_local."0.3.6" = deps: f: updateFeatures f (rec {
    lazy_static."${deps.thread_local."0.3.6".lazy_static}".default = true;
    thread_local."0.3.6".default = (f.thread_local."0.3.6".default or true);
  }) [
    (features_.lazy_static."${deps."thread_local"."0.3.6"."lazy_static"}" deps)
  ];


# end
# ucd-util-0.1.1

  crates.ucd_util."0.1.1" = deps: { features?(features_.ucd_util."0.1.1" deps {}) }: buildRustCrate {
    crateName = "ucd-util";
    version = "0.1.1";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "02a8h3siipx52b832xc8m8rwasj6nx9jpiwfldw8hp6k205hgkn0";
  };
  features_.ucd_util."0.1.1" = deps: f: updateFeatures f (rec {
    ucd_util."0.1.1".default = (f.ucd_util."0.1.1".default or true);
  }) [];


# end
# unicode-width-0.1.5

  crates.unicode_width."0.1.5" = deps: { features?(features_.unicode_width."0.1.5" deps {}) }: buildRustCrate {
    crateName = "unicode-width";
    version = "0.1.5";
    authors = [ "kwantam <kwantam@gmail.com>" ];
    sha256 = "0886lc2aymwgy0lhavwn6s48ik3c61ykzzd3za6prgnw51j7bi4w";
    features = mkFeatures (features."unicode_width"."0.1.5" or {});
  };
  features_.unicode_width."0.1.5" = deps: f: updateFeatures f (rec {
    unicode_width."0.1.5".default = (f.unicode_width."0.1.5".default or true);
  }) [];


# end
# unicode-xid-0.1.0

  crates.unicode_xid."0.1.0" = deps: { features?(features_.unicode_xid."0.1.0" deps {}) }: buildRustCrate {
    crateName = "unicode-xid";
    version = "0.1.0";
    authors = [ "erick.tryzelaar <erick.tryzelaar@gmail.com>" "kwantam <kwantam@gmail.com>" ];
    sha256 = "05wdmwlfzxhq3nhsxn6wx4q8dhxzzfb9szsz6wiw092m1rjj01zj";
    features = mkFeatures (features."unicode_xid"."0.1.0" or {});
  };
  features_.unicode_xid."0.1.0" = deps: f: updateFeatures f (rec {
    unicode_xid."0.1.0".default = (f.unicode_xid."0.1.0".default or true);
  }) [];


# end
# utf8-ranges-1.0.0

  crates.utf8_ranges."1.0.0" = deps: { features?(features_.utf8_ranges."1.0.0" deps {}) }: buildRustCrate {
    crateName = "utf8-ranges";
    version = "1.0.0";
    authors = [ "Andrew Gallant <jamslam@gmail.com>" ];
    sha256 = "0rzmqprwjv9yp1n0qqgahgm24872x6c0xddfym5pfndy7a36vkn0";
  };
  features_.utf8_ranges."1.0.0" = deps: f: updateFeatures f (rec {
    utf8_ranges."1.0.0".default = (f.utf8_ranges."1.0.0".default or true);
  }) [];


# end
# vec_map-0.8.1

  crates.vec_map."0.8.1" = deps: { features?(features_.vec_map."0.8.1" deps {}) }: buildRustCrate {
    crateName = "vec_map";
    version = "0.8.1";
    authors = [ "Alex Crichton <alex@alexcrichton.com>" "Jorge Aparicio <japaricious@gmail.com>" "Alexis Beingessner <a.beingessner@gmail.com>" "Brian Anderson <>" "tbu- <>" "Manish Goregaokar <>" "Aaron Turon <aturon@mozilla.com>" "Adolfo Ochagavía <>" "Niko Matsakis <>" "Steven Fackler <>" "Chase Southwood <csouth3@illinois.edu>" "Eduard Burtescu <>" "Florian Wilkens <>" "Félix Raimundo <>" "Tibor Benke <>" "Markus Siemens <markus@m-siemens.de>" "Josh Branchaud <jbranchaud@gmail.com>" "Huon Wilson <dbau.pp@gmail.com>" "Corey Farwell <coref@rwell.org>" "Aaron Liblong <>" "Nick Cameron <nrc@ncameron.org>" "Patrick Walton <pcwalton@mimiga.net>" "Felix S Klock II <>" "Andrew Paseltiner <apaseltiner@gmail.com>" "Sean McArthur <sean.monstar@gmail.com>" "Vadim Petrochenkov <>" ];
    sha256 = "1jj2nrg8h3l53d43rwkpkikq5a5x15ms4rf1rw92hp5lrqhi8mpi";
    dependencies = mapFeatures features ([
]);
    features = mkFeatures (features."vec_map"."0.8.1" or {});
  };
  features_.vec_map."0.8.1" = deps: f: updateFeatures f (rec {
    vec_map = fold recursiveUpdate {} [
      { "0.8.1".default = (f.vec_map."0.8.1".default or true); }
      { "0.8.1".serde =
        (f.vec_map."0.8.1".serde or false) ||
        (f.vec_map."0.8.1".eders or false) ||
        (vec_map."0.8.1"."eders" or false); }
    ];
  }) [];


# end
# version_check-0.1.4

  crates.version_check."0.1.4" = deps: { features?(features_.version_check."0.1.4" deps {}) }: buildRustCrate {
    crateName = "version_check";
    version = "0.1.4";
    authors = [ "Sergio Benitez <sb@sergio.bz>" ];
    sha256 = "1ghi6bw2qsj53x2vyprs883dbrq8cjzmshlamjsxvmwd2zp13bck";
  };
  features_.version_check."0.1.4" = deps: f: updateFeatures f (rec {
    version_check."0.1.4".default = (f.version_check."0.1.4".default or true);
  }) [];


# end
# winapi-0.3.5

  crates.winapi."0.3.5" = deps: { features?(features_.winapi."0.3.5" deps {}) }: buildRustCrate {
    crateName = "winapi";
    version = "0.3.5";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "0cfdsxa5yf832r5i2z7dhdvnryyvhfp3nb32gpcaq502zgjdm3w6";
    build = "build.rs";
    dependencies = (if kernel == "i686-pc-windows-gnu" then mapFeatures features ([
      (crates."winapi_i686_pc_windows_gnu"."${deps."winapi"."0.3.5"."winapi_i686_pc_windows_gnu"}" deps)
    ]) else [])
      ++ (if kernel == "x86_64-pc-windows-gnu" then mapFeatures features ([
      (crates."winapi_x86_64_pc_windows_gnu"."${deps."winapi"."0.3.5"."winapi_x86_64_pc_windows_gnu"}" deps)
    ]) else []);
    features = mkFeatures (features."winapi"."0.3.5" or {});
  };
  features_.winapi."0.3.5" = deps: f: updateFeatures f (rec {
    winapi."0.3.5".default = (f.winapi."0.3.5".default or true);
    winapi_i686_pc_windows_gnu."${deps.winapi."0.3.5".winapi_i686_pc_windows_gnu}".default = true;
    winapi_x86_64_pc_windows_gnu."${deps.winapi."0.3.5".winapi_x86_64_pc_windows_gnu}".default = true;
  }) [
    (features_.winapi_i686_pc_windows_gnu."${deps."winapi"."0.3.5"."winapi_i686_pc_windows_gnu"}" deps)
    (features_.winapi_x86_64_pc_windows_gnu."${deps."winapi"."0.3.5"."winapi_x86_64_pc_windows_gnu"}" deps)
  ];


# end
# winapi-i686-pc-windows-gnu-0.4.0

  crates.winapi_i686_pc_windows_gnu."0.4.0" = deps: { features?(features_.winapi_i686_pc_windows_gnu."0.4.0" deps {}) }: buildRustCrate {
    crateName = "winapi-i686-pc-windows-gnu";
    version = "0.4.0";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "05ihkij18r4gamjpxj4gra24514can762imjzlmak5wlzidplzrp";
    build = "build.rs";
  };
  features_.winapi_i686_pc_windows_gnu."0.4.0" = deps: f: updateFeatures f (rec {
    winapi_i686_pc_windows_gnu."0.4.0".default = (f.winapi_i686_pc_windows_gnu."0.4.0".default or true);
  }) [];


# end
# winapi-x86_64-pc-windows-gnu-0.4.0

  crates.winapi_x86_64_pc_windows_gnu."0.4.0" = deps: { features?(features_.winapi_x86_64_pc_windows_gnu."0.4.0" deps {}) }: buildRustCrate {
    crateName = "winapi-x86_64-pc-windows-gnu";
    version = "0.4.0";
    authors = [ "Peter Atashian <retep998@gmail.com>" ];
    sha256 = "0n1ylmlsb8yg1v583i4xy0qmqg42275flvbc51hdqjjfjcl9vlbj";
    build = "build.rs";
  };
  features_.winapi_x86_64_pc_windows_gnu."0.4.0" = deps: f: updateFeatures f (rec {
    winapi_x86_64_pc_windows_gnu."0.4.0".default = (f.winapi_x86_64_pc_windows_gnu."0.4.0".default or true);
  }) [];


# end
}
