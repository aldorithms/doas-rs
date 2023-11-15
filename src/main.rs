use std::{path::PathBuf, error::Error};

use clap::{arg, value_parser, command};
use libc::EILSEQ;

mod doas;

///
/// Execute commands as another user
/// 
/// # Description
/// 
/// The doas utility executes the given command as another user. 
/// The command argument is mandatory unless -C, -L, or -s is specified.
/// 
/// # Usage
/// 
/// doas [-nSs] [-a style] [-C config] [-u user] command [args]
/// 
/// # Arguments
/// 
/// `-C --config <FILE>`: 
/// Parse and check the configuration file config, then exit. 
/// If command is supplied, doas will also perform command matching.
/// In the latter case either ‘permit’, ‘permit nopass’ or ‘deny’ will be printed on standard output, depending on command matching results. 
/// No command is executed.
/// 
/// `-u --user`: Execute the command as user. The default is root.
/// 
/// # Examples
/// 
fn main() -> Result<(), Box<dyn Error + 'static>> {
    let matches = command!()
        .args(&[
            arg!(Command: [command] ... "Command to run").required(false),
            arg!(Config: -C --config <FILE>  "Specify config file to debug.").required(false).value_parser(value_parser!(PathBuf)),
            arg!(Clear: -L --clear "Clear any persisted authentications from previous invocations.").required(false),
            arg!(NoInteract: -n "Non-Interactive").required(false),
            arg!(User: -u --user <USER> "Specify user to run command as.").required(false),
            arg!(Shell: -s --shell "launch a shell as a user.").required(false),
        ])
        .get_matches();
        
        if matches.args_present() {
            
        }
    
    Ok(())
}
