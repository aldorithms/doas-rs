use std::path::PathBuf;
use clap::{arg, value_parser};

pub fn cmd() -> clap::Command {
    clap::Command::new("doas").about("Execute commands as another user, default is root").args(&[
        arg!(Command: [command] ... "Command to run").required_unless_present_any(["Config", "Clear", "Shell"]).value_parser(value_parser!(String)),
        arg!(Config: -C --config <FILE>  "Specify config file to debug.").required(false).value_parser(value_parser!(PathBuf)),
        arg!(Clear: -L --clear "Clear any persisted authentications from previous invocations.").required(false),
        arg!(NonInteract: -n "Non-Interactive").required(false),
        arg!(User: -u --user <USER> "Specify user to run command as.").required(false),
        arg!(Shell: -s --shell "launch a shell as a user.").required(false),
    ])
}