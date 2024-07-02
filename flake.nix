{
  description = "Project Description"; # TODO: Project Description

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    nix2container.url = "github:nlewo/nix2container";
    nix2container.inputs.nixpkgs.follows = "nixpkgs";
    mk-shell-bin.url = "github:rrbutani/nix-mk-shell-bin";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };


  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem =
        {
          config,
          self',
          inputs',
          lib,
          pkgs,
          system,
          ...
        }:
        {
          devenv.shells.default = {
            difftastic.enable = true;
            packages =
              lib.optionals pkgs.stdenv.isDarwin (
                with pkgs;
                [
                  darwin.apple_sdk.frameworks.Security
                  darwin.apple_sdk.frameworks.SystemConfiguration
                  darwin.apple_sdk.frameworks.AppKit
                  darwin.apple_sdk.frameworks.WebKit
                  llvmPackages.libcxxStdenv
                  llvmPackages.libcxxClang
                  darwin.libobjc
                ]
              );
            env = { };
            languages.rust = {
              enable = true;
              channel = "stable";
              components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            };

            dotenv.enable = true;
          };
        };
    };
}
