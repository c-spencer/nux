#[macro_use]
extern crate serde_derive;

mod commands;
mod disk;

use commands::Command;

fn main() {
    Command::run();
}
