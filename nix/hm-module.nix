{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  simplemoji = import ./. {
    inherit pkgs lib;
    system = pkgs.system;
  };
  cfgSimplemoji = config.programs.simplemoji;
  # Temp config
  simplemojiPackage = lists.optional cfgSimplemoji.enable simplemoji.packages.default;
in
{
  options.programs = {
    simplemoji = {
      enable = mkEnableOption "enable simplemoji";
    };
  };

  config = mkIf cfgSimplemoji.enable {
    home.packages = simplemojiPackage;
  };
}
