use std::fs;
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use crate::config::PufferfishConfig;
use crate::parser;

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
            Self::_build(PathBuf::from(name).as_path(), config, clean);
        } else {
            let clean = if matches.is_present("clean") {
                true
            } else { false };
            let html_dir = &config.html_dir();
            let html_files = fs::read_dir(&html_dir).unwrap();
            for file in html_files {
                let file = file.unwrap().path();
                let file = file.strip_prefix("html").unwrap(); // Strip base html path
                Self::_build(file, config, clean);
            }
        }
    }
    
    /// Build & write file
    fn _build(file: &Path, config: &PufferfishConfig, clean: bool) {
        let parsed = parser::parse_file(&file, &config.html_dir(), &config.template_dir(), clean, Some(&config.cache_dir()));
        let output_dir = &config.output_dir();
        fs::write(PathBuf::from(output_dir).join(&file.file_name().unwrap()), parsed)
            .expect(&format!("Couldn't write file {:?}", PathBuf::from(output_dir).join(&file.file_name().unwrap())));
        if config.verbose() {
            eprintln!("Written file {:?}", file);
        }
    }
}