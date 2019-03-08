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
                .label("efiboot"),
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

    pub fn add_partition(mut self, partition: Partition) -> Disk {
        self.partitions.push(partition);

        self
    }
}

trait Filesystem {
    fn cmds(&self) -> Vec<String>;
}

struct EfiFilesystem {
    partition: Partition,
    mount: String,
}

impl Filesystem for EfiFilesystem {
    fn cmds(&self) -> Vec<String> {
        vec![
            format!(
                "mkfs.fat -F 32 /dev/disk/by-partlabel/{label}",
                label = self.partition.label.clone().unwrap()
            ),
            format!("mkdir /mnt{mount}", mount = self.mount),
            format!(
                "mount /dev/disk/by-partlabel/{label} /mnt{mount}",
                label = self.partition.label.clone().unwrap(),
                mount = self.mount
            ),
        ]
    }
}

struct LuksFilesystem {
    partition: Partition,
    passphrase: String,
    filesystem: Filesystem,
}

impl Filesystem for LuksFilesystem {
    fn cmds(&self) -> Vec<String> {
        let mut a = vec![
            format!(
                "echo {passphrase} | cryptsetup luksFormat /dev/disk/by-partlabel/{label}",
                passphrase = self.passphrase,
                label = self.partition.label.clone().unwrap()
            ),
            format!(
                "echo {passphrase} | cryptsetup open /dev/disk/by-partlabel/{label} decrypted-{label}",
                passphrase = self.passphrase,
                label = self.partition.label.clone().unwrap()
            ),
        ];

        a.append(&mut self.filesystem.cmds());

        a
    }
}

pub struct Partition {
    size: Option<String>,
    code: Option<String>,
    label: Option<String>,
}

impl Partition {
    pub fn new() -> Partition {
        Partition {
            size: None,
            code: None,
            label: None,
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

    fn sgdisk_opts(&self) -> String {
        format!(
            "-n 0:0:{size} -t -0:{code} -c 0:{label}",
            size = self.size.clone().unwrap(),
            code = self.code.clone().unwrap(),
            label = self.label.clone().unwrap()
        )
    }
}
