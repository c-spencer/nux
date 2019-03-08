use super::zfs;

#[derive(Debug, Deserialize)]
pub struct DiskSettings {
    boot_size: String,
    device: String,
}

impl DiskSettings {
    pub fn get_disk(&self) -> Disk {
        Disk::new(&*self.device).add_partition(
            Partition::new()
                .size(&*self.boot_size)
                .code("ef00")
                .label("efiboot")
                .filesystem(Filesystem::Efi(EfiFilesystem { mount: "/efi".to_owned() })),
        )
    }
}

pub struct Disk {
    device: String,
    partitions: Vec<Partition>
}

impl Disk {
    pub fn new(device: &str) -> Disk {
        Disk {
            device: device.to_owned(),
            partitions: Vec::new()
        }
    }

    pub fn format_cmd(&self) -> String {
        format!(
            "sgdisk -Z {partitions} {device}",
            partitions = self
                .partitions
                .iter()
                .map(|p| p.sgdisk_opts())
                .collect::<Vec<String>>()
                .join(" "),
            device = self.device
        )
    }

    pub fn cmds(&self) -> Vec<String> {
        self.partitions.iter().flat_map(|p| p.cmds()).collect()
    }

    pub fn add_partition(mut self, partition: Partition) -> Disk {
        self.partitions.push(partition);

        self
    }
}

pub enum Filesystem {
    Efi(EfiFilesystem),
    Luks(LuksFilesystem),
    Zfs(zfs::ZfsSettings)
}

impl Filesystem {
    fn cmds(&self, device: &str, label: &str) -> Vec<String> {
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
    fn cmds(&self, device: &str, label: &str) -> Vec<String> {
        vec![
            format!(
                "mkfs.fat -F 32 /dev/disk/by-partlabel/{label}",
                label = label
            ),
            format!("mkdir /mnt{mount}", mount = self.mount),
            format!(
                "mount /dev/disk/by-partlabel/{label} /mnt{mount}",
                label = label,
                mount = self.mount
            ),
        ]
    }
}

pub struct LuksFilesystem {
    pub passphrase: String,
    pub filesystem: Box<Filesystem>,
}

impl LuksFilesystem {
    fn cmds(&self, device: &str, label: &str) -> Vec<String> {
        let mut a = vec![
            format!(
                "echo {passphrase} | cryptsetup luksFormat /dev/disk/by-partlabel/{device}",
                passphrase = self.passphrase,
                device = device
            ),
            format!(
                "echo {passphrase} | cryptsetup open /dev/disk/by-partlabel/{device} decrypted-{label}",
                passphrase = self.passphrase,
                device = device,
                label = label
            ),
        ];

        a.append(&mut self.filesystem.cmds(&*format!("/dev/mapper/decrypted-{label}", label = label), &*format!("decrypted-{label}", label = label)));

        a
    }
}

pub struct Partition {
    size: Option<String>,
    code: Option<String>,
    label: Option<String>,
    filesystem: Option<Filesystem>
}

impl Partition {
    pub fn new() -> Partition {
        Partition {
            size: None,
            code: None,
            label: None,
            filesystem: None
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

    fn sgdisk_opts(&self) -> String {
        format!(
            "-n 0:0:{size} -t -0:{code} -c 0:{label}",
            size = self.size.clone().unwrap(),
            code = self.code.clone().unwrap(),
            label = self.label.clone().unwrap()
        )
    }

    fn cmds(&self) -> Vec<String> {
        match &self.filesystem {
            Some(fs) => fs.cmds(&*format!("/dev/disk/by-partlabel/{label}", label = self.label.clone().unwrap()), &*self.label.clone().unwrap()),
            None => vec![]
        }
    }
}
