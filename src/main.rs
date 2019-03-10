#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rust_embed;

mod commands;
mod disk;

use commands::Command;

#[derive(RustEmbed)]
#[folder = "assets"]
struct Asset;

fn main() {
    Command::run();
}
