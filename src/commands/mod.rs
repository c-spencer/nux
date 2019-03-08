use structopt::StructOpt;

mod install;
mod repo;

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "install")]
    Install(install::InstallCommand),

    #[structopt(name = "repo")]
    Repository(repo::RepositoryCommand),
}

impl Command {
    pub fn run() {
        Command::from_args().exec();
    }

    fn exec(&self) {
        match self {
            Command::Install(cmd) => {
                let result = cmd.run();

                println!("{:?}", result);
            }

            Command::Repository(cmd) => cmd.run(),
        }
    }
}
