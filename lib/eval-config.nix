{ pkgs ? import <nixpkgs> { }
, lib ? pkgs.lib
, flake ? null
, flakeAttr ? null
, diskoFile ? null
, rootMountPoint ? "/mnt"
, ...
}@args:
let
  disko = import ./. {
    inherit rootMountPoint;
    inherit lib;
  };

  flake' = (builtins.getFlake flake);

  hasDiskoFile = diskoFile != null;

  hasFlakeDiskoConfig = lib.hasAttrByPath [ "diskoConfigurations" flakeAttr ] flake';

  hasFlakeDiskoModule =
    lib.hasAttrByPath [ "nixosConfigurations" flakeAttr "config" "disko" "devices" ] flake';

  diskFormat =
    let
      diskoConfig =
        if hasDiskoFile then
          import diskoFile
        else
          flake'.diskoConfigurations.${flakeAttr};
    in
    if builtins.isFunction diskoConfig then
      diskoConfig ({ inherit lib; } // args)
    else
      diskoConfig;

  evaluatedConfig =
    if hasDiskoFile || hasFlakeDiskoConfig then
      disko.eval diskFormat
    else if (lib.traceValSeq hasFlakeDiskoModule) then
      flake'.nixosConfigurations.${flakeAttr}
    else
      (builtins.abort "couldn't find `diskoConfigurations.${flakeAttr}` or `nixosConfigurations.${flakeAttr}.config.disko.devices`");

  diskoConfig = evaluatedConfig.config.disko.devices;

  finalConfig = lib.filterAttrsRecursive (name: value: !lib.hasPrefix "_" name) diskoConfig;
in
finalConfig