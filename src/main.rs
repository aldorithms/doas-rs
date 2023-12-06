use std::error::Error;
use human_panic::setup_panic;

mod doas;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    setup_panic!(); 

    let matches = doas::cmd()
        .get_matches();
    
    let commands: Vec<_> = matches
        .get_many::<String>("Command")
        .unwrap().map(|s| s.as_str())
        .collect();
    
    Ok(())
}   