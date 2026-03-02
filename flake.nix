# Usage:
#   nix profile add github:asciimoth/aaa
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
        postInstall = ''
          mkdir -p $out/share/applications $out/share/mime/packages
          cp ${./aaa.desktop} $out/share/applications/aaa.desktop
          cp ${./mime.xml} $out/share/mime/packages/3a-mime.xml

          mkdir -p $out/share/bash-completion/completions
          $out/bin/aaa completions bash > $out/share/bash-completion/completions/aaa
          mkdir -p $out/share/fish/vendor_completions.d
          $out/bin/aaa completions fish > $out/share/fish/vendor_completions.d/aaa.fish
          mkdir -p $out/share/zsh/site-functions/
          $out/bin/aaa completions zsh > $out/share/zsh/site-functions/_aaa
          mkdir -p $out/share/nu/completions
          $out/bin/aaa completions nush > $out/share/nu/completions/aaa.nu
        '';
      };
    in {
      devShells.default = pkgs.mkShell {
        inherit (checks.pre-commit-check) shellHook;
        buildInputs = with pkgs; [
          # app

          cargo
          # clippy
          cargo-cross
          cargo-zigbuild
          zig
          goreleaser

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
