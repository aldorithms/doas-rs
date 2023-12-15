use std::path::PathBuf;

use clap::{Command, command, arg, value_parser};

// Command Line Interface layout
pub fn cmd() -> Command {
    command!()
        .subcommands([
            // Define doas main command
            Command::new("doas")
                .about("Run a command as a different user, default is root")
                .defer(|cmd| { 
                    cmd.args(&[
                        arg!(command: <COMMAND> ... "Command to run")
                            .required(true)
                            .value_parser(value_parser!(String)),

                        arg!(user: -u -user <USER> "Arguments to pass to command")
                            .required(false)
                            .value_parser(value_parser!(String)),
                            
                        arg!(config: -C --config [config] "Path to config file")
                            .required(false)
                            .value_parser(value_parser!(PathBuf)),
                    ])
                }),

            Command::new("doasedit")
                .about("Edit a file as a different user"),

            Command::new("vidoas")
                .about("visual editor in doas"),
        ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli() {
        let matches = cmd().get_matches_from("doas ls -l".split_ascii_whitespace());
        assert_eq!(matches.subcommand_name(), Some("doas"));
    }
}