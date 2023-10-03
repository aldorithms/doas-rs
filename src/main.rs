use std::path::PathBuf;

use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    // usage: doas [-nSs] [-a style] [-C config] [-u user] command [args]
    let matches = command!() // requires `cargo` feature
        .arg(arg!(-L "Clear any persisted authentications from previous invocations, then exit."))
        .arg(arg!(-n "Non interactive mode, fail if the matching rule doesn't have the nopass option."))
        .arg(arg!(-s "Execute the shell from SHELL"))
        // .arg(arg!(-a --style "Stylization"))
        .arg(arg!(-C --config <FILE> "Sets a custom config file").required(false).value_parser(value_parser!(PathBuf)))
        .arg(arg!(-u --user [user] "Run command as user, default is root"))
        .arg(arg!(<command> ...).required(true));
}
