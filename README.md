# nux

Nominally "NixOS User eXperience", `nux` is intended to be a CLI tool for helping improve the end-to-end experience of using NixOS. Fundamentally it's actually a tool to help me learn Rust, NixOS, ZFS and other things I want to learn about.

`nux` is intended to be an opinionated tool, which helps automate best practice procedures with regards to everything from initial partitioning through to development environments.

# Status

## Install

- [ ] Partition disks
  - [x] Read disk configuration from config file
  - [ ] Pull saved disk configs from git
  - [x] Generate partition and filesystem commands
    - [x] Generate `sgdisk` commands
    - [x] Generate efi partition commands
    - [x] Generate ZFS commands for zpool + sensible core datasets
    - [x] Generate encrypted luks partition
  - [x] Setup boot with single-unlock password via keyfiles
  - [ ] Execute commands
  - [ ] Save disk config to git
- [ ] Initial NixOS config
  - [ ] Inject bootloader config
  - [ ] Inject sensible nix zfs configuration
  - [ ] Store nix config in git
  - [ ] Pull nix config from existing git
  - [ ] Setup initial user
  - [ ] Setup [home manager](https://github.com/rycee/home-manager)

# References

- https://nixos.wiki/wiki/NixOS_on_ZFS
- https://elvishjerricco.github.io/2018/12/06/encrypted-boot-on-zfs-with-nixos.html
- https://github.com/a-schaefers/themelios
- https://github.com/yacinehmito/yarn-nix
- https://github.com/barrucadu/nixfiles
- https://github.com/jgillich/nixos
- https://developer.atlassian.com/blog/2016/02/best-way-to-store-dotfiles-git-bare-repo/
