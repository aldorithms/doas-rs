use std::{process::Command, os::unix::process::CommandExt};
use human_panic::setup_panic;
use doas_rs::cli::cmd;
use color_eyre::Result;
use pam::PasswordConv;

fn main() -> Result<()> {
    setup_panic!(); // <-- initialize human-panic                                  
    let matches = cmd().get_matches();

    match matches.subcommand_name() {
        Some("doas") => {
            let doas_matches = matches
                .subcommand_matches("doas")
                .unwrap();
            let commands = doas_matches
                .get_many::<String>("command")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>();
            let default_user = format!("root");
            let user = doas_matches
                .get_one::<String>("user")
                .unwrap_or(&default_user);
            
            // Authenticate session.

            let execute_commands = Command::new(commands[0])
                .args(&commands[1..])
                .exec();

        },
        Some("doasedit") => {
            todo!("doasedit");
        },
        Some("vidoas") => {
            todo!("vidoas");
        },
        _ => {
            cmd().print_help()?;
        },
    }
    Ok(())
}