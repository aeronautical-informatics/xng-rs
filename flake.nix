{
  inputs = {
    fenix.url = "github:nix-community/fenix";
    fenix.inputs.nixpkgs.follows = "nixpkgs";

    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, fenix, naersk }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages."${system}";
      mkShell = pkgs.mkShell.override { stdenv = pkgs.clangMultiStdenv; };
      release-channel = "stable";
      rust-toolchain = with fenix.packages."${system}"; combine (
        (with  fenix.packages."${system}"."${release-channel}"; [
          cargo
          clippy
          rustc
          rustfmt
        ])
        ++
        (with fenix; [
          targets.armv7a-none-eabi."${release-channel}".rust-std
          targets.armv7r-none-eabi."${release-channel}".rust-std
          targets.armv7r-none-eabihf."${release-channel}".rust-std
        ])
      );
      naersk-lib = naersk.lib.${system}.override {
        cargo = rust-toolchain;
        rustc = rust-toolchain;
      };
      LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
    in
    rec {
      packages."${system}".xng-rs = naersk-lib.buildPackage {
        pname = "xng-rs";
        root = ./.;
        doCheck = true;
        doDoc = true;
        doDocFail = true;
        copyTarget = true;
      };
      defaultPackage."${system}" = packages.${system}.xng-rs;

      devShell."${system}" = mkShell {
        inherit LIBCLANG_PATH;
        inputsFrom = [ packages."${system}".xng-rs ];
        shellHook = ''
          echo -e 'Remember to set the `C_INCLUDE_PATH` env var so that the XNG
          headers can be found! For example:\n'
          echo 'C_INCLUDE_PATH=/my/xng/installation/include cargo build'
        '';
      };

      hydraJobs = packages;
    };
}
