# File: packages.nix
# Author: rowsred
# Date: 2026-05-02
# Descriptions:
{ ... }:
{
  perSystem =
    {
      config,
      pkgs,
      system,
      ...
    }:
    {
      #packages.default = pkgs.stdenv.mkDerivation {
      #  pname = "my-project";
      #  version = "0.1.0";
      #  src = ./.;

      #  nativeBuildInputs = [
      #    pkgs.rustc
      #    pkgs.cargo
      #  ];

      #  buildPhase = "cargo build --release";
      #  installPhase = "install -Dm755 target/release/my-project $out/bin/my-project";
      #};
    };
}
