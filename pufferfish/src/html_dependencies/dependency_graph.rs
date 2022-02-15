use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use crate::config::PufferfishConfig;
use crate::html_dependencies::dependency;

/// Generate dependency graph for html files
pub fn generate(config: &PufferfishConfig) -> Vec<dependency::File> {
    let html_dir = config.html_dir();
    let templates_dir = config.template_dir();
    
    let graph = generate_for_dir(&html_dir, &templates_dir);
    
    graph.expect("Html directory is empty")
}

/// Generate for all files in a `html` dir
fn generate_for_dir<D>(dir: D, templates_dir: &str) -> Option<Vec<dependency::File>> where D: AsRef<Path> {
    let mut dependencies: Vec<dependency::File> = Vec::new();
    match fs::read_dir(&dir) {
        Ok(dir_contents) => {
            for file in dir_contents {
                let file = file.as_ref().expect(&format!("Error reading {:?}", &file))
                    .path();
                if file.is_dir() {
                    let mut _dependencies = generate_for_dir(&file, templates_dir);
                    if _dependencies.is_some() {
                        dependencies.append(unsafe { &mut _dependencies.unwrap_unchecked() });
                    }
                } else {
                    let file_dependencies = generate_for_file(&file, templates_dir);
                    dependencies.push(file_dependencies);
                };
            }
        }
        _ => {
            return None
        }
    }
    return if dependencies.is_empty() { None } else { Some(dependencies) }
}

fn generate_for_file<D>(file: D, templates_dir: &str) -> dependency::File where D: AsRef<Path> + Debug {
    let contents = fs::read_to_string(&file).expect(&format!("Couldn't read file '{:?}'", &file));
    let mut dependency = dependency::File::new(file.as_ref().to_path_buf(), None);
    // Get dependencies of file
    let mut _dependencies: Vec<&str> = Vec::new();
    Regex::new(r"%.*?[^\\]%").unwrap().find_iter(&contents).for_each(|m| {
        _dependencies.push(&contents[m.start()+1..m.end()-1]);
    });
    for dep in _dependencies {
        let dep = if !Regex::new(r".*\..*").unwrap().is_match(dep) {
            format!("{}.html", dep)
        } else {
            dep.to_string()
        };
        dependency.add_dependency(generate_for_file(PathBuf::from(templates_dir).join(&dep), templates_dir));
    }
    dependency
}