# File: develop.nix
# Author: rowsred
# Date: 2026-05-02
# Descriptions:
{ ... }:
{
  perSystem =
    { config, pkgs, ... }:
    {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
        ];
        buildInputs = with pkgs; [
          pkg-config
          udev
          usbutils

        ];
        shellHook = ''
          echo "Welcome to DevShell"
        '';
      };
    };
}
