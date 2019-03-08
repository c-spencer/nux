use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum RepositoryCommand {
    #[structopt(name = "set")]
    Set(SetCommand),
}

#[derive(StructOpt, Debug)]
pub struct SetCommand {
    path: String,
}

impl SetCommand {
    fn run(&self) {
        println!("REPO {:?}", self);
    }
}

impl RepositoryCommand {
    pub fn run(&self) {
        match self {
            RepositoryCommand::Set(cmd) => cmd.run(),
        }
    }
}
