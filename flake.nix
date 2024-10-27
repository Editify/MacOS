{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs@{ nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = nixpkgs.lib.systems.flakeExposed;
      perSystem = {
        lib,
        pkgs,
        system,
        config,
        ...
      }: 
      {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import inputs.rust-overlay)
          ];
        };

        devShells.default = pkgs.mkShell
        {
          nativeBuildInputs = with pkgs; [
						zig
          ] ++ lib.optionals stdenv.isLinux [
          ] ++ lib.optionals stdenv.isDarwin (with pkgs; [
            darwin.apple_sdk.Frameworks.Metal
          ]);
        };
      };
    };
}
