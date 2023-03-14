{ pkgs ? import <nixpkgs> {}}:
with pkgs; let packages = rec {
    swycle = callPackage ./swycle.nix {};
};
in packages
