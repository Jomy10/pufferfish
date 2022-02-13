use std::fs;
use std::path::{PathBuf};
use clap::ArgMatches;
use crate::build_html::build_html_file;
use crate::config::{PufferfishConfig};

pub struct CLIExecutor {
    matches: ArgMatches
}

impl CLIExecutor {
    pub fn new(matches: ArgMatches) -> Self {
        Self { matches }
    }
    pub fn execute(&self) {
        let config = PufferfishConfig::from_toml("pufferfish.toml");
        match self.matches.subcommand() {
            Some(("build", matches)) => Self::execute_build(matches, &config),
            Some((unknown, _)) => eprintln!("Unknown subcommand {unknown}"),
            None => {}
        }
    }
}
impl CLIExecutor {
    /// Builds the html files
    fn execute_build(matches: &ArgMatches, config: &PufferfishConfig) {
        if let Some(name) = matches.value_of("file") {
            let clean = if matches.is_present("clean") {
                true
            } else { false };
            build_html_file(PathBuf::from(name).as_path(), config, clean);
        } else {
            let clean = if matches.is_present("clean") {
                true
            } else { false };
            let html_dir = &config.html_dir();
            let html_files = fs::read_dir(&html_dir).unwrap();
            for file in html_files {
                let file = file.unwrap().path();
                let file = file.strip_prefix(&html_dir).unwrap(); // Strip base html path
                build_html_file(file, config, clean);
            }
        }
    }
}