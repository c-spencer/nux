use config::{Config, ConfigError, File};
use structopt::StructOpt;

use crate::disk::duct_util::Cmd;

use crate::disk;

#[derive(StructOpt, Debug)]
pub struct InstallCommand {
    #[structopt(long)]
    /// Whether to execute the install
    exec: bool,
    #[structopt(short, long)]
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

        install(self, &settings);

        println!("SETTINGS {:?}", settings);

        return Ok(());
    }
}

fn install(command: &InstallCommand, settings: &disk::DiskSettings) {
    let root_disk = settings.get_disk();

    for cmd in root_disk.cmds() {
        cmd.exec(command.exec);
    }

    println!("--------------------------------------------------------------------");

    // Nixos config

    Cmd::new("nixos-generate-config")
        .arg("--root")
        .arg("/mnt")
        .to_expr()
        .exec(command.exec);
}
