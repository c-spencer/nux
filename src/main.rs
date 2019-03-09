#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate rust_embed;

mod commands;
mod disk;

use commands::Command;

fn main() {
    Command::run();
}
