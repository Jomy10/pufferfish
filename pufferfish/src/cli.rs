use std::fs;
use std::path::{Path, PathBuf};
use clap::ArgMatches;
use minify_html::minify;
use crate::config::{MinifyMethod, PufferfishConfig};
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
    
    // TODO: move to separete module
    /// Build & write file
    fn _build(file: &Path, config: &PufferfishConfig, clean: bool) {
        let parsed = parser::parse_file(&file, &config.html_dir(), &config.template_dir(), clean, Some(&config.cache_dir()));
        let output_dir = &config.output_dir();
        if config.minify() {
            let parsed = parsed.as_bytes();
            if config.method() == MinifyMethod::Default {
                let mut cfg = minify_html::Cfg::new();
                cfg.do_not_minify_doctype = !config.minify_doctype();
                cfg.ensure_spec_compliant_unquoted_attribute_values = config.ensure_spec_compliant_unquoted_attribute_values();
                cfg.keep_closing_tags = config.keep_closing_tags();
                cfg.keep_html_and_head_opening_tags = config.keep_html_and_head_opening_tags();
                cfg.keep_spaces_between_attributes = config.keep_spaces_between_attributes();
                cfg.keep_comments = config.keep_comments();
                cfg.minify_css = config.minify_css();
                cfg.minify_js = config.minify_js();
                cfg.remove_bangs = config.remove_bangs();
                cfg.remove_processing_instructions = config.remove_processing_instructions();
                let minified = minify(&parsed, &cfg);
                fs::write(PathBuf::from(output_dir).join(&file.file_name().unwrap()), minified)
                    .expect(&format!("Couldn't write file {:?}", PathBuf::from(output_dir).join(&file.file_name().unwrap())));
            } else {
                unimplemented!()
            }
        } else {
            fs::write(PathBuf::from(output_dir).join(&file.file_name().unwrap()), parsed)
                .expect(&format!("Couldn't write file {:?}", PathBuf::from(output_dir).join(&file.file_name().unwrap())));
        }
        if config.verbose() {
            eprintln!("Written file {:?}", file);
        }
    }
}