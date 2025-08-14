{
  pkgs ?
    import <nixpkgs> {
      overlays = [
        (import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix")
      ];
    },
}: let
  packages = with pkgs; [
    (fenix.complete.withComponents [
      "cargo"
      "clippy"
      "rust-src"
      "rustc"
      "rustfmt"
    ])

    watchexec
    cargo-udeps
    cargo-audit
    cargo-expand
  ];

  libraries = with pkgs; [
    pkg-config
    openssl
  ];
in
  with pkgs;
    mkShell {
      name = "git-helper";
      buildInputs = packages ++ libraries;

      DIRENV_LOG_FORMAT = "";
      LD_LIBRARY_PATH = "${lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH";
      CFLAGS = "-DJEMALLOC_STRERROR_R_RETURNS_CHAR_WITH_GNU_SOURCE";
    }
