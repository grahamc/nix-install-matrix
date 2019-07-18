{ pkgs ? import <nixpkgs> {} }:
(pkgs.callPackage ./Cargo.nix {
  cratesIO = pkgs.callPackage ./crates-io.nix {};
}).nix_install_matrix_tools {}
