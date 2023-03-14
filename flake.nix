{
  description =
    "Use sway ipc to move workspaces on a cyclic grid";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.configurable-flakes.url = "github:sents/configurable-flakes";

  outputs = inputs@{ self, nixpkgs, flake-utils, configurable-flakes }:
    let
      lib = nixpkgs.lib;
      utils = flake-utils.lib;
    in
    configurable-flakes.lib.configurableFlake inputs
      {
        options = {
          systems = lib.mkOption {
            type = with lib.types; listOf (enum utils.allSystems);
            default = utils.defaultSystems;
          };
        };
      }
      ({ config, ... }:
        utils.eachSystem config.systems (system:
          let
            pkgs = nixpkgs.legacyPackages.${system};
            packages = import ./default.nix { inherit pkgs; };
          in
          { packages = packages // { default = packages.swycle; }; }));
}
