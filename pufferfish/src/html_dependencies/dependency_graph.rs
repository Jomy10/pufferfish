use std::fs;
use std::path::{Path};
use crate::config::PufferfishConfig;

/// Generate dependency graph for html files
pub fn generate(config: &PufferfishConfig) -> String {
    let html_dir = config.html_dir();
    let templates_dir = config.template_dir();
    
    let graph = generate_for_dir(&html_dir, &templates_dir);
    
    "".to_string()
}

fn generate_for_dir<D>(dir: D, templates_dir: &str) where D: AsRef<Path> {
    match fs::read_dir(&dir) {
        Ok(dir_contents) => {
            for file in dir_contents {
                let file = file.as_ref().expect(&format!("Error reading {:?}", &file))
                    .path();
                let graph = if file.is_dir() {
                    generate_for_dir(&file, templates_dir)
                } else {
                    unimplemented!()
                };
            }
        }
        _ => {
            // TODO: return ...
        }
    }
    
}