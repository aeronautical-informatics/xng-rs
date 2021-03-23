{ pkgs ? import ./nix/nixpkgs.nix }:

with pkgs;

let
  ske-server = import ../default.nix { inherit pkgs; };
  generatedBuild = callPackage ./Cargo.nix {
    defaultCrateOverrides = pkgs.defaultCrateOverrides // {
      xng-rs = attrs: {
        buildInputs = [ (stdenv.lib.getLib gcc.cc) pkgs.llvmPackages.clang-unwrapped.lib ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.clang-unwrapped.lib}/lib";
        CPATH = "${ske-server}/include:${gcc-unwrapped}/lib/gcc/x86_64-unknown-linux-gnu/9.2.0/include:${glibc.dev}/include";
        preConfigure = ''
          echo HERE $CPATH
        '';
      };
    };
  };
in generatedBuild.rootCrate.build
