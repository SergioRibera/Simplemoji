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
  cfg = config.programs.simplemoji;
in {
  options.programs.simplemoji = {
    enable = mkEnableOption "cli to take simplemoji";
  };

  config = mkIf cfg.enable {
    environment.systemPackages = [simplemoji.packages.default];
  };
}
