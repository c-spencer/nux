use colored::*;
use console::Style;
use dialoguer::theme;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct InitialiseCommand {}

impl InitialiseCommand {
    pub fn run(&self) -> Result<(), Box<std::error::Error>> {
        let theme = theme::ColorfulTheme {
            values_style: Style::new().green().bold(),
            indicator_style: Style::new().yellow().bold(),
            yes_style: Style::new().yellow(),
            no_style: Style::new().yellow(),
            ..theme::ColorfulTheme::default()
        };

        let username: String = dialoguer::Input::with_theme(&theme)
            .with_prompt("Username")
            .interact()?;

        let git_option = dialoguer::Select::with_theme(&theme)
            .with_prompt("Git repository")
            .default(0)
            .item("new")
            .item("existing")
            .interact()?;

        let mut git: String = "".to_owned();

        if git_option == 1 {
            println!("For GitHub repositories, you can enter username/repo.");
            println!("For other git repositories, enter the full URL.");
            git = dialoguer::Input::with_theme(&theme)
                .with_prompt("Git URL")
                .interact()?;
        }

        let directory: String = dialoguer::Input::with_theme(&theme)
            .with_prompt("Configuration directory")
            .default(".nux-config".to_owned())
            .interact()?;

        // println!("{}", "Username must not be blank.".red().bold());

        Ok(())
    }
}
