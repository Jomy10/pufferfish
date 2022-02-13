use std::fs;
use std::path::Path;
use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// Pufferfish config specified in toml
pub struct PufferfishConfig {
    project: Option<ProjectConfig>
}

#[derive(Deserialize, Debug)]
struct ProjectConfig {
    html_dir: Option<String>,
    template_dir: Option<String>,
    output_dir: Option<String>,
    pretty: Option<bool>,
    minify: Option<bool>,
    verbose: Option<bool>
}

impl PufferfishConfig {
    pub fn from_toml(file_path: &str) -> Self {
        toml::from_str(
            fs::read_to_string(file_path)
                .expect(&format!("Coudn't find {}", file_name_from_path(file_path))).as_str()
        ).expect(&format!("Couldn't parse {}", file_name_from_path(file_path)))
    }
}
// Project
impl PufferfishConfig {
    pub fn html_dir(&self) -> String {
        if let Some(project) = &self.project {
            project.html_dir.clone().unwrap_or("html".to_string())
        } else {
            "html".to_string()
        }
    }
    
    pub fn template_dir(&self) -> String {
        if let Some(project) = &self.project {
            project.template_dir.clone().unwrap_or("templates".to_string())
        } else {
            "templates".to_string()
        }
    }
    
    pub fn output_dir(&self) -> String {
        if let Some(project) = &self.project {
            project.output_dir.clone().unwrap_or("output".to_string())
        } else {
            "output".to_string()
        }
    }
    
    pub fn pretty(&self) -> bool {
        if let Some(project) = &self.project {
            project.pretty.unwrap_or(false)
        } else {
            false
        }
    }
    
    pub fn minify(&self) -> bool {
        if let Some(project) = &self.project {
            project.minify.unwrap_or(false)
        } else {
            false
        }
    }
    
    pub fn verbose(&self) -> bool {
        if let Some(project) = &self.project {
            project.verbose.unwrap_or(false)
        } else {
            false
        }
    }
}

fn file_name_from_path(file_path: &str) -> &str {
    let r = Regex::new(r".*/(.*)?").unwrap().find(file_path).unwrap();
    &file_path[r.start()..r.end()]
}