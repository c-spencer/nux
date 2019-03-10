use crate::Asset;
use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum GenerateCommand {
    #[structopt(name = "disk")]
    ZfsDisk,
}

impl GenerateCommand {
    pub fn run(&self) -> std::io::Result<()> {
        let contents = Asset::get("disk.toml").unwrap();

        let mut file = File::create("disk.toml")?;

        file.write_all(contents.as_ref())?;

        Ok(())
    }
}
