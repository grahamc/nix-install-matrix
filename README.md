Most software testing is prescriptive: a suite of tests are written
and executed in specific scenarios to ensure they all pass. A
prescriptive test failure indicates a serious problem which needs to
be addressed.

The Nix Install Matrix test suite is _descriptive_, *not*
prescriptive.

The goal of the test matrix is to _describe_ the current state of the
Nix installer, and environments it works in, and environments it
doesn't work in. If the descriptive tests fail, it is not, by default
a problem -- just the reality of the current software.

Over time we make work to improve the installer to allow more of the
tests to pass, either on specific distributions, specific
configurations, or in cross-cutting ways.

The goal of the install matrix is to help evaluate the changes to the
installation procedure to determine if the change has improved or
degraded the installation process.

---

# Nix Test Matrix

# Background

The Nix installer interacts with the host operating system directly,
and this exposes it to many more opportunities to fail or behave
funny. Different operating systems will also be able to support
different sets of Nix’s features.

# Problem

Testing the Nix installer is complicated and involves spawning a lot
of VMs to validate it, and a lot of different things to check. The
current testing strategy is fairly ad-hoc and has inconsistent
coverage. Additionally, validating changes is slow, manual,
error-prone, and no fun.

# Contained Solution

A test matrix which automatically spawns VMs of many different
operating systems, flavors, and versions.

Each VM can be subjected to many tests from basic "does the
installer create a working Nix?” to “Is the Nix install multi-user?"
to more complicated checks like sandboxing, internal network proxies,
and more.


## Hacking and Implementation Notes

./matrix.nix defines a list of ways to install Nix and a list of
images with Vagrant boxes to try. The list of tests to run is the
cartesian product of these two lists.

New test cases should be added by creating either a new image
definition, or a new install definition. The actual tests executed are
defined by ./test.sh and all test cases run the exact same test suite.

For example, if you wanted to add a test to see how the Nix installer
handles when / is all chmod'd to 777, you could add the following
image:

```
  images = {
    "debian-9-chmod-777" = {
      image = "debian/stretch64";
      preInstall = ''
        apt-get update
        apt-get install -y curl
        chmod -R 777 /
      '';
    };
```

### Running Tests

Run all the tests:

    nix-build ./test-script.nix && ./result ./output-directory

Run the tests with all the distros for only the `install-default`
installation method:

    nix-build ./test-script.nix --argstr installMethodFilter install-default
    ./result

Run the tests with all the install methods, on only Arch Linux:

    nix-build ./test-script.nix --argstr imageNameFilter arch
    ./result

Run only one test, of Arch Linux and the default installation method:

    nix-build ./test-script.nix --argstr imageNameFilter arch --argstr installMethodFilter install-default
    ./result

## Generating reports

    cd nix-install-matrix-tools
    nix-build
    ./result/bin/treeport --input ./the-input-directory --output ./report.html

### System Requirements:

You will need a lot of disk space, Nix, and Virtualbox. If you're on
NixOS you can enable VirtualBox by adding the following to your
configuration.nix:

    virtualisation.virtualbox.host.enable = true;

### License Notes:

The matrix defines tests for macOS using the macOS operating system.
Their EULA says you can only run those on Apple hardware. If you're
running these tests on hardware which is not by Apple you should
take care to not run those tests.



## Results

The test matrix’s results are not pass-fail, but instead generates a
report of supported features per os/distro/version.

In the future, an easy to review report will be generated. A report
might look like this:


| OS                          | Installation succeeds | Installation Time  | multi-user? | sandboxing? | nix-shell succeeds? | nix-env as a user? | nix-env as root? | nix-store over ssh? |
| --------------------------- | --------------------- | ------------------ | ----------- | ----------- | ------------------- | ------------------ | ---------------- | ------------------- |
| Debian Jessie               | yes                   | 35s                | yes         | yes         | yes                 | yes                | yes              | no                  |
| Debian Jessie —daemon       | yes                   | 1yr17m45d27h99m66s | yes         | yes         |                     |                    |                  |                     |
| Debian Jessie —single-user  | yes                   |                    | no          |             |                     |                    |                  |                     |
| Docker container            | yes                   |                    | no          | no          | yes                 | yes                | yes              | no                  |
| Debian Jessie w/ HTTP proxy | yes                   |                    | yes         | yes         | yes                 | yes                | yes              | no                  |
| macOS                       | …                     |                    |             |             |                     |                    |                  |                     |

## Requirements

 - The tests should be easy to execute automatically on a moderately
   powerful laptop.
 - The tests will not be required to be run in a tightly sandboxed
   environment, like inside of Hydra.
 - The test results should be human and machine-parsable.
 - This shouldn’t be written in bash.
 - It should be trivial to add new operating systems and test cases.
 - It should be easy to only run a subset of the matrix.
 - It should try to be based on qemu.
 - The test matrix should test upgrades
 - The test matrix should test against machines in weird installation
   states, like Nix was installed then a macos upgrade, and now Nix is
   broken


## Future Requirements

 - The tests should be able to be run in a tightly sandboxed
   environment, like inside of Hydra.
 - The tests should run under nix-build
 - There could be a really cool visualization of the support changes
   over time.
