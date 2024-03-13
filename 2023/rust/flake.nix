# This flake was initially generated by fh, the CLI for FlakeHub (version 0.1.8)
{
  # A helpful description of your flake
  description = "Advent of code 2023 written in rust";

  # Flake inputs
  inputs = {
    flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/*.tar.gz";

    flake-schemas.url = "https://flakehub.com/f/DeterminateSystems/flake-schemas/*.tar.gz";

    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1.*.tar.gz";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  # Flake outputs that other flakes can use
  outputs = { self, flake-compat, flake-schemas, nixpkgs, rust-overlay }:
    let
      # Nixpkgs overlays
      overlays = [
        rust-overlay.overlays.default
        (final: prev: {
          rustToolchain = final.rust-bin.stable.latest.default;
        })
      ];

      # Helpers for producing system-specific outputs
      supportedSystems = [ "x86_64-linux" ];
      forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
        pkgs = import nixpkgs { inherit overlays system; };
      });
    in {
      # Schemas tell Nix about the structure of your flake's outputs
      schemas = flake-schemas.schemas;

      # Development environments
      devShells = forEachSupportedSystem ({ pkgs }: {
        default = pkgs.mkShell {
          # Pinned packages available in the environment
          packages = with pkgs; [
            rustToolchain
            cargo-watch
            rust-analyzer
            nixpkgs-fmt
          ];

          # Environment variables
          env = {
            RUST_BACKTRACE = "1";
          };

          # A hook run every time you enter the environment
          shellHook = ''
            echo "DevShell active.
Use 'exit' to exit"
          '';
        };
      });
    };
}