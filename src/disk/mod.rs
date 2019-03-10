use duct_util::{Cmd, Expr};

use std::collections::HashMap;

pub mod duct_util;
mod zfs;

#[derive(Debug, Deserialize)]
pub struct DiskSettings {
    boot_size: String,
    device: String,
    properties: Option<HashMap<String, String>>,
    pool: zfs::ZfsPool,
    datasets: Vec<zfs::ZfsDataset>,
}

impl DiskSettings {
    pub fn get_disk(&self) -> Disk {
        let passphrase =
            rpassword::read_password_from_tty(Some("Passphrase for LUKS partition: ")).unwrap();

        println!();

        let passphrase_confirm =
            rpassword::read_password_from_tty(Some("Confirm passphrase: ")).unwrap();

        println!();

        if passphrase != passphrase_confirm {
            panic!("Passphrases didn't match!");
        }

        Disk::new(&*self.device)
            .add_partition(
                Partition::new()
                    .size(&*self.boot_size)
                    .code("ef00")
                    .label("efiboot")
                    .filesystem(Filesystem::Efi(EfiFilesystem {
                        mount: "/efi".to_owned(),
                    })),
            )
            .add_partition(
                Partition::new()
                    .label("zfsroot")
                    .code("8300")
                    .size("0")
                    .filesystem(Filesystem::Luks(LuksFilesystem {
                        passphrase: passphrase,
                        filesystem: Box::new(Filesystem::Zfs(zfs::ZfsSettings {
                            pool: self.pool.clone(),
                            properties: self.properties.clone(),
                            datasets: self.datasets.clone(),
                        })),
                    })),
            )
    }
}

pub struct Disk {
    device: String,
    partitions: Vec<Partition>,
}

impl Disk {
    pub fn new(device: &str) -> Disk {
        Disk {
            device: device.to_owned(),
            partitions: Vec::new(),
        }
    }

    fn format_cmd(&self) -> Expr {
        Cmd::new("sgdisk")
            .arg("-Z")
            .args(
                &self
                    .partitions
                    .iter()
                    .flat_map(|p| p.sgdisk_opts())
                    .collect::<Vec<String>>(),
            )
            .arg(&self.device)
            .to_expr_with_wait(500)
    }

    pub fn cmds(&self) -> Vec<Expr> {
        let mut cmds = vec![self.format_cmd()];

        for p in &self.partitions {
            for cmd in p.cmds() {
                cmds.push(cmd);
            }
        }

        // TODO: This needs a proper home.

        cmds.push(Cmd::new("mkdir").arg("-p").arg("/mnt/boot").to_expr());

        let cpio = Cmd::new("cpio")
            .arg("-o")
            .opt("-H", "newc")
            .opt("-R", "+0:+0")
            .arg("--reproducible")
            .to_duct();

        let gzip = Cmd::new("gzip").arg("-9").to_duct();

        cmds.push(Expr::new(
            cpio.input("./keyfile.bin".as_bytes())
                .pipe(gzip)
                .stdout("/mnt/boot/initrd.keys.gz"),
        ));

        cmds
    }

    pub fn add_partition(mut self, partition: Partition) -> Disk {
        self.partitions.push(partition);

        self
    }
}

pub enum Filesystem {
    Efi(EfiFilesystem),
    Luks(LuksFilesystem),
    Zfs(zfs::ZfsSettings),
}

impl Filesystem {
    fn cmds(&self, device: &str, label: &str) -> Vec<Expr> {
        match self {
            Filesystem::Efi(efi) => efi.cmds(device, label),
            Filesystem::Luks(luks) => luks.cmds(device, label),
            Filesystem::Zfs(zfsfs) => {
                let mut cmds = vec![zfsfs.zpool_cmd(device)];
                cmds.append(&mut zfsfs.dataset_cmds());
                cmds
            }
        }
    }
}

pub struct EfiFilesystem {
    mount: String,
}

impl EfiFilesystem {
    fn cmds(&self, device: &str, _label: &str) -> Vec<Expr> {
        let mount = format!("/mnt{mount}", mount = self.mount);

        vec![
            Cmd::new("mkfs.fat")
                .opt("-F", "32")
                .arg(device)
                .to_expr_with_wait(500),
            Cmd::new("mkdir")
                .arg("-p")
                .arg(mount.clone())
                .to_expr_with_wait(500),
            Cmd::new("mount")
                .arg(device)
                .arg(mount)
                .to_expr_with_wait(500),
        ]
    }
}

pub struct LuksFilesystem {
    pub passphrase: String,
    pub filesystem: Box<Filesystem>,
}

impl LuksFilesystem {
    fn cmds(&self, device: &str, label: &str) -> Vec<Expr> {
        let mut a = vec![
            // Format and open the partition
            Cmd::new("cryptsetup")
                .arg("luksFormat")
                .arg(device)
                .as_expr(|e| e.input(self.passphrase.as_bytes())),
            Cmd::new("cryptsetup")
                .arg("open")
                .arg(device)
                .arg(format!("decrypted-{}", label))
                .as_expr(|e| e.input(self.passphrase.as_bytes())),
            // Generate a keyfile and register it with the partition
            Cmd::new("dd")
                .arg("if=/dev/urandom")
                .arg("of=./keyfile.bin")
                .arg("bs=1024")
                .arg("count=4")
                .to_expr(),
            Cmd::new("cryptsetup")
                .arg("luksAddKey")
                .arg(device)
                .arg("./keyfile.bin")
                .as_expr(|e| e.input(self.passphrase.as_bytes())),
        ];

        a.append(&mut self.filesystem.cmds(
            &*format!("/dev/mapper/decrypted-{label}", label = label),
            &*format!("decrypted-{label}", label = label),
        ));

        a
    }
}

pub struct Partition {
    size: Option<String>,
    code: Option<String>,
    label: Option<String>,
    filesystem: Option<Filesystem>,
}

impl Partition {
    pub fn new() -> Partition {
        Partition {
            size: None,
            code: None,
            label: None,
            filesystem: None,
        }
    }

    pub fn label(mut self, label: &str) -> Partition {
        self.label = Some(label.to_owned());
        self
    }

    pub fn code(mut self, code: &str) -> Partition {
        self.code = Some(code.to_owned());
        self
    }

    pub fn size(mut self, size: &str) -> Partition {
        self.size = Some(size.to_owned());
        self
    }

    pub fn filesystem(mut self, filesystem: Filesystem) -> Partition {
        self.filesystem = Some(filesystem);
        self
    }

    fn sgdisk_opts(&self) -> Vec<String> {
        vec![
            "-n".to_owned(),
            format!("0:0:{size}", size = self.size.clone().unwrap()),
            "-t".to_owned(),
            format!("0:{code}", code = self.code.clone().unwrap()),
            "-c".to_owned(),
            format!("0:{label}", label = self.label.clone().unwrap()),
        ]
    }

    fn cmds(&self) -> Vec<Expr> {
        match &self.filesystem {
            Some(fs) => fs.cmds(
                &*format!(
                    "/dev/disk/by-partlabel/{label}",
                    label = self.label.clone().unwrap()
                ),
                &*self.label.clone().unwrap(),
            ),
            None => vec![],
        }
    }
}
