#!/usr/bin/env bash

export PS4=' ''${BASH_SOURCE}::''${FUNCNAME[0]}::$LINENO '
set -u
set -o pipefail

testnixinfo() (
    nix-shell -p nix-info --run "nix-info -m"
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
  nix-env --uninstall 'hello.*'
)

runtest() {
    testFn=$1

    local testdir="$TESTDIR/tests/$testFn"
    mkdir -p "$testdir"

    start=$(date '+%s')
    echo "Starting: $testFn"
    (
        set -ex
        "$testFn"
    ) 2>&1 | tee "$testdir/log" | sed 's/^/    /'
    exitcode=$?
    end=$(date '+%s')
    duration=$((end - start))
    echo "$exitcode" > "$testdir/exitcode"
    echo "$duration" > "$testdir/duration"

    echo "Finishing: $testFn, duration:$duration result:$exitcode"
}

main() {
    readonly TESTDIR=./nix-test-matrix-log
    rm -rf "$TESTDIR"
    mkdir "$TESTDIR"

    uname -a > "$TESTDIR/uname"
    nix-shell -p nix-info --run "nix-info -m" > "$TESTDIR/nix-info"
    (
        runtest testnixinfo
        runtest testshell
        runtest testnixenv
        runtest testnixchannel
    )

    tar -cf ./nix-test-matrix-log.tar "$TESTDIR"
}

main
