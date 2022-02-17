use std::fs;
use std::path::PathBuf;
use std::process::exit;
use clap::ArgMatches;
use simple_colors::red;
use crate::cli::build::build_html::build_html_file;
use crate::cli::dev_server;
use crate::pufferfish_core::config::PufferfishConfig;

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
            Some(("start", matches)) => Self::execute_start(matches, &config),
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
            build_html_file(PathBuf::from(name).as_path(), config, clean, false);
            copy_assets(&config);
        } else {
            let clean = if matches.is_present("clean") {
                true
            } else { false };
            let html_dir = &config.html_dir();
            let html_files = fs::read_dir(&html_dir).unwrap();
            for file in html_files {
                let file = file.unwrap().path();
                let file = file.strip_prefix(&html_dir).unwrap(); // Strip base html path
                build_html_file(file, config, clean, false);
            }
            copy_assets(&config)
        }
    }
    
    fn execute_start(_matches: &ArgMatches, config: &PufferfishConfig) {
        dev_server::start_development_server(config);
    }
}

fn copy_assets(config: &PufferfishConfig) {
    // Copy assets
    match fs_extra::dir::copy(
        config.assets_dir(),
        config.output_dir(),
        &fs_extra::dir::CopyOptions::new()
    ) {
        Ok(_) => {
            if config.verbose() {
                println!("Copied assets");
            }
        }
        Err(err) => {
            eprintln!("{}", red!(&format!("Could not copy assets: {}", err)));
            exit(1)
        }
    }
}