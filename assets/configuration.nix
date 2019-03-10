{ config, pkgs, ... }:

{
  imports = [
    ./hardware-configuration.nix
  ];

  boot.initrd.luks.devices.decrypted-zfsroot = {
    device = "/dev/disk/by-partlabel/zfsroot";
    keyFile = "/keyfile.bin";
  };

  boot.kernelParams = [ "elevator=noop" "boot.shell_on_fail" ];
  boot.supportedFilesystems = [ "zfs" ];
  boot.zfs = {
    forceImportAll = false;
    forceImportRoot = false;
  };
  
  boot.loader = {
    # Tell NixOS to install Grub as an EFI application in /efi
    efi.efiSysMountPoint = "/efi";

    grub = {
      enable = true;
      version = 2;
      device = "nodev"; # Do not install Grub for BIOS booting.
      efiSupport = true;
      extraInitrd = "/boot/initrd.keys.gz"; # Add our LUKS key to the initrd
      enableCryptodisk = true; # Allow Grub to boot from LUKS devices.
      zfsSupport = true;
      copyKernels = true; # Avoid long dir paths for kernels on ZFS.
    };

    # Different systems may require a different one of the following two
    # options. The first instructs Grub to install itself in an EFI standard
    # location. And the second tells it to install somewhere custom, but
    # mutate the EFI NVRAM so EFI knows where to find it. The former
    # should work on any system. The latter allows you to share one ESP
    # among multiple OSes, but doesn't work on a few systems (namely
    # VirtualBox, which doesn't support persistent NVRAM).
    #
    # Just make sure to only have one of these enabled.
    grub.efiInstallAsRemovable = true;
    efi.canTouchEfiVariables = false;
  };

  # Necessary for ZFS, move to main configuration if you want this stable.
  networking.hostId = "{{host_id}}";

  environment.systemPackages = with pkgs; [
    wget vim git
  ];

  # This value determines the NixOS release with which your system is to be
  # compatible, in order to avoid breaking some software such as database
  # servers. You should change this only after NixOS release notes say you
  # should.
  system.stateVersion = "19.09";
}