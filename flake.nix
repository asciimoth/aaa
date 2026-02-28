# Usage:
#   nix profile add github:sciimoth/aaa
#   nix profile remove aaa
#   nix shell github:asciimoth/aaa
# Update: nix flake update
{
  description = "aaa";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    flake-utils,
    pre-commit-hooks,
    naersk,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
      };
      naersk' = pkgs.callPackage naersk {};

      checks = {
        pre-commit-check = pre-commit-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            cargo-check.enable = true;
            commitizen.enable = true;
            typos.enable = true;
            typos-commit = {
              enable = true;
              description = "Find typos in commit message";
              entry = let script = pkgs.writeShellScript "typos-commit" ''
                typos "$1"
              ''; in builtins.toString script;
              stages = [ "commit-msg" ];
            };
            cargo-test = {
              enable = true;
              name = "cargo test (workspace)";
              entry = "cargo test --workspace --all-features";
              pass_filenames = false;
              stages = [ "pre-commit" ];
            };
          };
        };
      };

      app = naersk'.buildPackage {
        src = ./.;
      };
    in {
      devShells.default = pkgs.mkShell {
        inherit (checks.pre-commit-check) shellHook;
        buildInputs = with pkgs; [
          # app

          cargo
          # clippy

          typos
          commitizen

          lolcat

          neofetch
          fastfetch
          screenfetch
          nitch
          profetch
          leaf
          fetch-scm
        ];
      };

      packages.default = app;
    });
}
