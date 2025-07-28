{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
}:
rustPlatform.buildRustPackage {
  pname = "git-helper";
  version = "0.1.0";
  src = ../.;
  cargoHash = "sha256-uzvm3jSkw1bmmja8koLzAbFDR2ei1ARDHyIpChK+JtQ=";
  buildInputs = [openssl];
  nativeBuildInputs = [pkg-config];
  useFetchCargoVendor = true;

  meta = with lib; {
    description = ''Simple git tool for switching "profiles" and managing repositories'';
    homepage = "https://github.com/NikSneMC/git-helper";
    mainProgram = "git-helper";
    platforms = platforms.linux;
    license = licenses.mit;
  };
}
