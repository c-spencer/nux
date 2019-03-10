# nux

[![Build Status](https://travis-ci.com/c-spencer/nux.svg?branch=master)](https://travis-ci.com/c-spencer/nux) [![Pre-release](https://img.shields.io/github/release-pre/c-spencer/nux.svg)](https://github.com/c-spencer/nux/releases)

Nominally "NixOS User eXperience", `nux` is intended to be a CLI tool for helping improve the end-to-end experience of using NixOS. Fundamentally it's actually a tool to help me learn Rust, NixOS, ZFS and other things I want to learn about.

`nux` is intended to be an opinionated tool, which helps automate best practice procedures with regards to everything from initial partitioning through to development environments.

**You should not be using this project yet, outside of a VM you don't care about.**

---

## Install

Hacky install (in minimal unstable live CD)

```bash
nix-env -iA nixos.carnix nixos.git
# Needing to do this seems like a problem?
nix-env --set-flag priority 4 rust_carnix

git clone https://github.com/c-spencer/nux.git
carnix generate-nix --standalone --src .

nix-build Cargo.nix -A nux
nix-env -i ./result
```

Binary:

```bash
curl https://github.com/c-spencer/nux/releases/download/v0.1.1/nux-v0.1.1-x86_64-unknown-linux-gnu.tar.gz -L --output nux.tar.gz
tar xvzf nux.tar.gz

# Note ld-linux location:
ldd nux
patchelf --set-interpreter /nix/store/...-glibc-2.27/ld-linux-x86_64.so.2 nux
chmod u+x nux
```

Run:

```bash
# Dry run install
./nux install --disk /dev/nvme01

# Real deal
./nux install --disk /dev/nvme01 --dry-run=false
```

On first reboot, you will need to add `zfs_force=1` to kernel params from grub. After first reboot, this is no longer necessary.

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
  - [x] Execute commands
  - [ ] Save disk config to git
- [ ] Initial NixOS config
  - [x] Inject bootloader config
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
