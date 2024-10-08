{
  description = "A simple Formatter for the hypr ecosystem configs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ self, ... }:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import (inputs.nixpkgs) {
          inherit system;
          overlays = [ (import inputs.rust-overlay) ];
        };

        rustc = pkgs.rust-bin.stable.latest.rustc;
        cargo = pkgs.rust-bin.stable.latest.cargo;
      in
      {
        devShell = pkgs.mkShell {
          name = "rust-dev-shell";

          buildInputs = [
            rustc
            cargo
            (with pkgs.rust-bin; [
              nightly.latest.rustfmt
              nightly.latest.rust-analyzer
            ])
          ];

          RUST_BACKTRACE = "1";
        };
      }
    );
}
