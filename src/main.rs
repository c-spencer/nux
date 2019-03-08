#[macro_use]
extern crate serde_derive;

mod commands;

use commands::Command;

fn main() {
    Command::run();
}
