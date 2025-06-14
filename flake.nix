{
  description = "FerrisBot - Rust Discord bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "ferrisbot";
          version = "0.1.0";

          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;

            outputHashes = {
              "mkwrs_scraper-0.1.0" = "sha256-5yJuRE46+S1zrb7ahOJoo6jvkuitEoHfvQRxLw0K4p0=";
            };
          };

          nativeBuildInputs = [ rust ];
        };

        devShells.default = pkgs.mkShell {
          packages = [ rust pkgs.cargo pkgs.rust-analyzer ];
        };
      });
}
