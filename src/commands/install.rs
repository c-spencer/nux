use config::{Config, ConfigError, File};
use structopt::StructOpt;

mod disk;
mod zfs;

#[derive(StructOpt, Debug)]
pub struct InstallCommand {
    disk: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Settings {
    disk: disk::DiskSettings,
    zfs: zfs::ZfsSettings,
}

impl InstallCommand {
    pub fn run(&self) -> Result<(), ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("nux.toml"))?;

        if let Some(disk) = self.disk.clone() {
            s.set("disk.device", disk)?;
        }

        let settings: Settings = s.try_into::<Settings>()?;

        install(&settings);

        println!("SETTINGS {:?}", settings);

        return Ok(());
    }
}

fn exec<S: Into<String>>(s: S) {
    println!("{}", s.into());
}

fn encrypt_partition(password: &str, device: &str, name: &str) -> String {
    exec(format!(
        "echo {password} | cryptsetup luksFormat {device}",
        password = password,
        device = device
    ));
    exec(format!(
        "echo {password} | cryptsetup open {device} {name}",
        password = password,
        device = device,
        name = name
    ));

    format!("/dev/mapper/{name}", name = name)
}

fn format_boot_partition() {
    exec("mkfs.fat -F 32 /dev/disk/by-partlabel/efiboot");
    exec("mkdir /mnt/efi");
    exec("mount /dev/disk/by-partlabel/efiboot /mnt/efi");
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

fn install(settings: &Settings) {
    let root_disk = settings.disk.get_disk().add_partition(
        disk::Partition::new()
            .label("zfsroot")
            .code("8300")
            .size("0"),
    );

    exec(root_disk.format_cmd());

    // format_disk(settings);

    let luks_partition = encrypt_partition(
        "password",
        "/dev/disk/by-partlabel/zfsroot",
        "decrypted-zfsroot",
    );

    // Pool and defaults

    exec(settings.zfs.zpool_cmd(&luks_partition));

    for cmd in settings.zfs.dataset_cmds() {
        exec(cmd);
    }

    // Boot

    format_boot_partition();
    create_keyfile("password", "/dev/disk/by-partlabel/zfsroot");

    // Nixos config

    exec("nixos-generate-config --root /mnt");
}
