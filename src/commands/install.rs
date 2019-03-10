use config::{Config, ConfigError};
use handlebars::Handlebars;
use rand::prelude::*;
use std::collections::BTreeMap;
use std::fs;
use std::io::prelude::*;
use structopt::StructOpt;

use crate::disk;
use crate::disk::duct_util::Cmd;
use crate::Asset;

#[derive(StructOpt, Debug)]
pub struct InstallCommand {
    #[structopt(long = "dry-run", name = "dry-run")]
    /// Whether to only generate the install commands, default true
    dry_run: Option<bool>,

    #[structopt(short, long)]
    /// Disk device to use as the boot target, e.g. /dev/sda
    disk: Option<String>,

    #[structopt(short = "s", long = "skip-partitioning", name = "skip-partitioning")]
    /// Whether to skip disk formatting and partitioning setup entirely.
    skip_partitioning: bool,

    #[structopt(long = "no-config", name = "no-config")]
    /// Whether to generate and insert the initial configuration.nix
    no_config: bool,
}

fn asset_as_str(name: &str) -> String {
    String::from_utf8(Vec::from(Asset::get(name).unwrap().as_ref())).unwrap()
}

impl InstallCommand {
    pub fn run(&self) -> Result<(), ConfigError> {
        let mut s = Config::new();

        s.merge(config::File::from_str(
            &asset_as_str("disk.toml"),
            config::FileFormat::Toml,
        ))?;

        s.merge(config::File::with_name("disk.toml").required(false))?;

        if let Some(disk) = self.disk.clone() {
            s.set("device", disk)?;
        }

        let settings: disk::DiskSettings = s.try_into::<disk::DiskSettings>()?;

        install(self, &settings);

        return Ok(());
    }
}

fn install(
    command: &InstallCommand,
    settings: &disk::DiskSettings,
) -> Result<(), Box<std::error::Error>> {
    let dry_run = command.dry_run.unwrap_or(true);

    if !command.skip_partitioning {
        let root_disk = settings.get_disk();

        let cmds = root_disk.cmds();

        let pb = indicatif::ProgressBar::new(1 + (cmds.len() as u64));

        for cmd in root_disk.cmds() {
            cmd.exec(!dry_run, &pb);
            pb.inc(1);
        }

        Cmd::new("nixos-generate-config")
            .arg("--root")
            .arg("/mnt")
            .to_expr()
            .exec(!dry_run, &pb);

        pb.finish();
    }

    if !command.no_config {
        let reg = Handlebars::new();

        let mut data: BTreeMap<String, String> = BTreeMap::new();

        let mut rng = rand::thread_rng();

        data.insert("host_id".to_owned(), format!("{:x}", rng.gen::<u32>()));

        let config_file = reg.render_template(&asset_as_str("boot-configuration.nix"), &data)?;
        let stub_file = asset_as_str("configuration.nix");

        if dry_run {
            println!("{}", config_file);
            println!("{}", stub_file);
        } else {
            println!("Generating new configuration at /mnt/etc/nixos/boot-configuration.nix");

            let mut file = fs::File::create("/mnt/etc/nixos/boot-configuration.nix")?;

            file.write_all(config_file.as_bytes())?;

            println!("Inserting stub configuration at /mnt/etc/nixos/configuration.nix");

            let mut file = fs::File::create("/mnt/etc/nixos/configuration.nix")?;

            file.write_all(stub_file.as_bytes())?;
        }
    }

    println!("All ready to go. Adjust any configuration you want, then run `nixos-install`.");

    Ok(())
}
