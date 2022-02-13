use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use crate::make_dirs;

// TODO: move to a struct

/// Parses a Pufferfish file, replacing the template strings with the correct files.
///
/// # Parameters
/// - `file`: The filename of the file where the templates should be replaced
/// - `base_dir`: The base directory for the `file` parameter
/// - `template_dir`: The base directory for the files to be replaced
/// - `cached`: Whether to use cached templates
/// - `cache_dir`: needs to be specified if cached is set to true
pub fn parse_file<P, B, T>(file: P, base_dir: B, template_dir: T, cached: bool, cache_dir: Option<&str>) -> String
    where
        P: AsRef<Path>,
        B: AsRef<Path> + Display + Clone,
        T: AsRef<Path> + Display + Clone {
    if cached {
        make_dirs(&PathBuf::from(cache_dir.unwrap()))
    }
    _parse_file(file, base_dir, template_dir, cached).replace("\\%", "%")
}

fn _parse_file<P, B, T> (file: P, base_dir: B, template_dir: T, cached: bool) -> String
    where
        P: AsRef<Path>,
        B: AsRef<Path> + Display + Clone,
        T: AsRef<Path> + Display + Clone {
    // Maybe for verbose? println!("{:?}", &PathBuf::new().join(&base_dir).join(&file));
    let contents = fs::read_to_string(&PathBuf::new().join(&base_dir).join(&file)).expect(&format!("Couldn't read file {}", &file.as_ref().to_str().unwrap()));
    parse_contents(&contents, base_dir, template_dir, cached)
}

/// Parses the contents of the `file` given to [`parse_file`](parse_file).
fn parse_contents<B,T>(contents: &str, base_dir: B, template_dir: T, cached: bool) -> String
    where
        B: AsRef<Path> + Display + Clone,
        T: AsRef<Path> + Display + Clone {
    let regex = Regex::new(r"%.*?[^\\]%").unwrap();
    return if let Some(range) = regex.find(&contents) {
        let file_name = &contents[range.start()..range.end()];
        let mut file_name = file_name.replace("%", "");
        // If file has no extension, add html
        if !Regex::new(r".*\..*").unwrap().is_match(&file_name) {
            file_name.push_str(".html"); // TODO: config for default file extension (can be set to none)
        }
        let replaced = regex.replace(&contents, _parse_file(file_name, template_dir.clone(), template_dir.clone(), cached));
        parse_contents(&replaced.to_string(), base_dir.clone(), template_dir, cached)
    } else {
        // No templates in contents
        contents.to_string()
    }
}