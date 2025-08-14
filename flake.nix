{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    naersk = {
      url = "github:nix-community/naersk";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        fenix.follows = "fenix";
      };
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    fenix,
    naersk,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [fenix.overlays.default];
      };

      naersk' = pkgs.callPackage naersk {};
    in {
      devShell = import ./shell.nix {inherit pkgs;};

      packages = rec {
        default = git-helper;

        git-helper = naersk'.buildPackage {
          src = ./.;

          meta.mainProgram = "git-helper";
        };
      };
    });
}
