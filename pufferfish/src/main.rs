use clap::{arg, App, ArgMatches, AppSettings};
use pufferfish::config::PufferfishConfig;

fn main() {
    let matches = App::new("Pufferfish")
        // Info
        .about("Does puffing")
        .version("0.3.0")
        .author("Jonas Everaert, info@jonaseveraert.be")
        .setting(AppSettings::SubcommandRequired)
        // Commands
        .subcommand(
            App::new("test")
                .about("does testing")
        )
        .get_matches();
    
    CLIExecutor::new(matches).execute();
}

pub struct CLIExecutor {
    matches: ArgMatches
}

impl CLIExecutor {
    pub fn new(matches: ArgMatches) -> Self {
        Self { matches }
    }
    pub fn execute(&self) {
        let config = PufferfishConfig::from_toml("example/pufferfish.toml");
        println!("Config: {:?}", config);
        match self.matches.subcommand() {
            Some(("test", matches)) => {
            
            },
            Some((unknown,_)) => eprintln!("Unknown subcommand {unknown}"),
            None => {
            
            }
        }
    }
}


