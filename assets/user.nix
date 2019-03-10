{ config, pkgs, ... }:

{
  # Select internationalisation properties.
  i18n = {
    consoleFont = "Lat2-Terminus16";
    consoleKeyMap = "uk";
    defaultLocale = "en_GB.UTF-8";
  };

  time.timeZone = "Europe/London";

  # Configure ZFS
  services.zfs = {
    autoScrub.enable = true;
    autoSnapshot.enable = true;
  };
}