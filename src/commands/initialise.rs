use colored::*;
use console::Style;
use dialoguer::theme;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct InitialiseCommand {}

#[derive(Debug)]
struct InitConfig {
    user: Option<String>,
    git_repo: Option<String>,
    tmp_repo: Option<String>,
    directory: Option<String>,
}

impl InitialiseCommand {
    pub fn run(&self) -> Result<(), Box<std::error::Error>> {
        let theme = theme::ColorfulTheme {
            values_style: Style::new().green().bold(),
            indicator_style: Style::new().yellow().bold(),
            yes_style: Style::new().yellow(),
            no_style: Style::new().yellow(),
            ..theme::ColorfulTheme::default()
        };

        let mut conf = InitConfig {
            user: None,
            git_repo: None,
            tmp_repo: None,
            directory: None,
        };

        let git_option = dialoguer::Select::with_theme(&theme)
            .with_prompt("Git repository")
            .default(0)
            .item("Existing")
            .item("New")
            .interact()?;

        if git_option == 0 {
            existing_repo(&mut conf, &theme).unwrap();
        } else {
            conf.user = Some(
                dialoguer::Input::with_theme(&theme)
                    .with_prompt("Username")
                    .interact()?,
            );
        }

        conf.directory = Some(
            dialoguer::Input::with_theme(&theme)
                .with_prompt("Configuration directory")
                .default(".nux-config".to_owned())
                .interact()?,
        );

        // println!("{}", "Username must not be blank.".red().bold());

        println!("{:#?}", conf);

        Ok(())
    }
}

fn existing_repo(
    conf: &mut InitConfig,
    theme: &theme::ColorfulTheme,
) -> Result<(), Box<std::error::Error>> {
    let git_provider = dialoguer::Select::with_theme(theme)
        .with_prompt("Git provider")
        .default(0)
        .item("GitHub")
        .item("GitLab")
        .item("BitBucket")
        .item("Other")
        .interact()?;

    match git_provider {
        0...2 => {
            let username: String = dialoguer::Input::with_theme(theme)
                .with_prompt("Username")
                .interact()?;

            let repo: String = dialoguer::Input::with_theme(theme)
                .with_prompt("Repository")
                .default("nux-config".to_owned())
                .interact()?;

            let base_uri = match git_provider {
                0 => "https://github.com",
                1 => "https://gitlab.com",
                _ => "https://bitbucket.com",
            };

            conf.git_repo = Some(format!("{}/{}/{}", base_uri, username, repo));
        }

        _ => {
            conf.git_repo = Some(
                dialoguer::Input::with_theme(theme)
                    .with_prompt("Repository URL")
                    .interact()?,
            );
        }
    }

    Ok(())
}
