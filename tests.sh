#!/usr/bin/env bash

export PS4=' ''${BASH_SOURCE}::''${FUNCNAME[0]}::$LINENO '
set -u
set -o pipefail

testexitok() (
    /bin/bash -c "exit 0"
)

testexitfail() (
    /bin/bash -c "exit 1"
)

testshell() (
  nix-shell -p hello --run hello
)

testnixenv() (
  nix-env -iA nixpkgs.hello
  hello
  nix-env --uninstall 'hello.*'
)

testnixchannel() (
  nix-channel --add https://nixos.org/channels/nixos-18.03 nixos-18-03
  nix-channel --update
  nix-env -iA nixos-18-03.hello
  hello
)

runtest() {
    testFn=$1
    start=$(date '+%s')
    echo "Starting: $testFn"
    (
        set -ex
        "$testFn" | sed 's/^/    /'
    )
    exitcode=$?
    end=$(date '+%s')
    echo "Finishing: $testFn, duration:$((end - start)) result:$exitcode"
}

(
    runtest testexitok
    runtest testexitfail
    runtest testshell
    runtest testnixenv
    runtest testnixchannel
) 2>&1
