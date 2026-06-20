{
  perSystem = {
    pkgs,
    toolchain,
    ...
  }: {
    devShells.default = let
      packages = with pkgs; [
        git

        toolchain

        watchexec
        cargo-audit
        cargo-udeps
        cargo-expand
      ];

      libraries = with pkgs; [
        pkg-config
      ];
    in
      with pkgs;
        mkShell {
          name = "git-helper";
          buildInputs = packages ++ libraries;

          DIRENV_LOG_FORMAT = "";
          LD_LIBRARY_PATH = "${lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH";
        };
  };
}
