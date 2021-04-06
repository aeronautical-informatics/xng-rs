{ pkgs ? import ./nix/nixpkgs.nix }:

with pkgs;

let
  generatedBuild = callPackage ./Cargo.nix { inherit pkgs; };
in generatedBuild.rootCrate.build
