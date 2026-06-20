{inputs, ...}: {
  imports = [
    ./package.nix
    ./shell.nix
  ];

  perSystem = {system, ...}: {
    _module.args = let
      pkgs = import inputs.nixpkgs {
        inherit system;
        overlays = [inputs.naersk.inputs.fenix.overlays.default];
      };

      toolchain = pkgs.fenix.fromToolchainFile {
        file = ../rust-toolchain.toml;
        sha256 = "sha256-rNsOYVHiSXXSDRGdg/StkiKCsyCTEPBfsP3R9spCu1c=";
      };

      naersk = pkgs.callPackage inputs.naersk {
        cargo = toolchain;
        rustc = toolchain;
      };
    in {
      inherit pkgs toolchain naersk;
    };
  };
}
