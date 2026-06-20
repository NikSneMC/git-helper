{
  perSystem = {
    naersk,
    pkgs,
    ...
  }: {
    packages.default = naersk.buildPackage {
      src = ../.;

      buildInputs = with pkgs; [
        openssl
        stdenv.cc.cc.lib
      ];
      nativeBuildInputs = with pkgs; [
        pkg-config
        autoPatchelfHook
      ];
    };
  };
}
