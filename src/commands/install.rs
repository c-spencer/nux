use config::{Config, ConfigError, File};
use structopt::StructOpt;

use crate::disk;

#[derive(StructOpt, Debug)]
pub struct InstallCommand {
    disk: Option<String>,
}

impl InstallCommand {
    pub fn run(&self) -> Result<(), ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("disk.toml"))?;

        if let Some(disk) = self.disk.clone() {
            s.set("device", disk)?;
        }

        let settings: disk::DiskSettings = s.try_into::<disk::DiskSettings>()?;

        install(&settings);

        println!("SETTINGS {:?}", settings);

        return Ok(());
    }
}

fn exec<S: AsRef<str>>(s: S) {
    println!("{}", s.as_ref());
}

fn create_keyfile(password: &str, encrypted_target: &str) {
    exec("dd if=/dev/urandom of=./keyfile.bin bs=1024 count=4");
    exec(format!(
        "echo {password} | cryptsetup luksAddKey {target} ./keyfile.bin",
        password = password,
        target = encrypted_target
    ));

    exec("mkdir /mnt/boot");

    exec("echo ./keyfile.bin | cpio -o -H newc -R +0:+0 --reproducible | gzip -9 > /mnt/boot/initrd.keys.gz");
}

fn install(settings: &disk::DiskSettings) {
    let root_disk = settings.get_disk();
    // .add_partition(
    //     disk::Partition::new()
    //         .label("zfsroot")
    //         .code("8300")
    //         .size("0").filesystem(disk::Filesystem::Luks(disk::LuksFilesystem {
    //             passphrase: "jimminy".to_owned(),
    //             filesystem: Box::new(disk::Filesystem::Zfs(settings.zfs.clone()))
    //         })),
    // );

    for cmd in root_disk.cmds() {
        println!("{:?}", cmd);
    }

    exec("--------------------------------------------------------------------");

    // Boot

    // format_boot_partition();

    create_keyfile("password", "/dev/disk/by-partlabel/zfsroot");

    // Nixos config

    exec("nixos-generate-config --root /mnt");
}
