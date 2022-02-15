use std::fs;
use regex::Regex;
use serde::Deserialize;
use crate::{puf_arg, puf_arg_string};

#[derive(Deserialize, Debug, Clone)]
/// Pufferfish config specified in toml
pub struct PufferfishConfig {
    project: ProjectConfig,
    minify: Option<MinifyConfig>,
    server: Option<ServerConfig>
}

#[derive(Deserialize, Debug, Clone)]
struct ProjectConfig {
    html_dir: Option<String>,
    template_dir: Option<String>,
    output_dir: Option<String>,
    cache_dir: Option<String>,
    dev_dir: Option<String>,
    pretty: Option<bool>,
    minify: Option<bool>,
    verbose: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
struct MinifyConfig {
    method: Option<String>,
    minify_doctype: Option<bool>,
    ensure_spec_compliant_unquoted_attribute_values: Option<bool>,
    keep_closing_tags: Option<bool>,
    keep_html_and_head_opening_tags: Option<bool>,
    keep_spaces_between_attributes : Option<bool>,
    keep_comments : Option<bool>,
    minify_css: Option<bool>,
    minify_js: Option<bool>,
    remove_bangs : Option<bool>,
    remove_processing_instructions : Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
struct ServerConfig {
    port: Option<String>
}

impl PufferfishConfig {
    pub fn from_toml(file_path: &str) -> Self {
        toml::from_str(
            fs::read_to_string(file_path)
                .expect(&format!("Couldn't find {}", file_name_from_path(file_path))).as_str()
        ).expect(&format!("Couldn't parse {}", file_name_from_path(file_path)))
    }
    
    pub fn new() -> Self {
        Self {
            project: ProjectConfig {
                html_dir: None,
                template_dir: None,
                output_dir: None,
                cache_dir: None,
                dev_dir: None,
                pretty: None,
                minify: None,
                verbose: None
            },
            minify: None,
            server: None
        }
    }
    
    pub fn set_server_port(&mut self, port: &str) {
        if let Some(conf) = &mut self.server {
            conf.port = Some(port.to_string());
        } else {
            self.server = Some(ServerConfig {
                port: Some(port.to_string())
            });
        }
    }
    
    pub fn set_verbose(&mut self, verbose: bool) {
        self.project.verbose = Some(verbose);
    }
}
// Project
impl PufferfishConfig {
    pub fn html_dir(&self) -> String {
        self.project.html_dir.clone().unwrap_or("html".to_string())
    }
    
    pub fn template_dir(&self) -> String {
        self.project.template_dir.clone().unwrap_or("templates".to_string())
    }
    
    pub fn output_dir(&self) -> String {
        self.project.output_dir.clone().unwrap_or("output".to_string())
    }
    
    pub fn cache_dir(&self) -> String {
        self.project.cache_dir.clone().unwrap_or(".pufferfish".to_string())
    }
    
    pub fn dev_dir(&self) -> String {
        self.project.dev_dir.clone().unwrap_or(self.output_dir())
    }
    
    pub fn pretty(&self) -> bool {
        self.project.pretty.unwrap_or(false)
    }
    
    pub fn minify(&self) -> bool {
        self.project.minify.unwrap_or(false)
    }
    
    pub fn verbose(&self) -> bool {
        self.project.verbose.unwrap_or(false)
    }
}
// Minify
impl PufferfishConfig {
    /// The minify method
    pub fn method(&self) -> MinifyMethod {
        if let Some(minify) = &self.minify {
            match minify.method.clone().unwrap_or("default".to_string()).as_str() {
                "default" => MinifyMethod::Default,
                "onepass" => MinifyMethod::Onepass,
                _ => panic!("Please specify a valid method for minify. Valid values are (default|onepass)")
            }
        } else {
            MinifyMethod::Default
        }
    }
    
    pub fn minify_doctype(&self) -> bool {
        puf_arg!(self.minify, minify_doctype, true)
    }
    
    pub fn ensure_spec_compliant_unquoted_attribute_values(&self) -> bool {
        puf_arg!(self.minify, ensure_spec_compliant_unquoted_attribute_values, true)
    }
    
    pub fn keep_closing_tags(&self) -> bool {
        puf_arg!(self.minify, keep_closing_tags, true)
    }
    
    pub fn keep_html_and_head_opening_tags(&self) -> bool {
        puf_arg!(self.minify, keep_html_and_head_opening_tags, true)
    }
    
    pub fn keep_spaces_between_attributes(&self) -> bool {
        puf_arg!(self.minify, keep_spaces_between_attributes, false)
    }
    
    pub fn keep_comments(&self) -> bool {
        puf_arg!(self.minify, keep_comments, false)
    }
    
    pub fn minify_css(&self) -> bool {
        puf_arg!(self.minify, minify_css, true)
    }
    
    pub fn minify_js(&self) -> bool {
        puf_arg!(self.minify, minify_js, true)
    }
    
    pub fn remove_bangs(&self) -> bool {
        puf_arg!(self.minify, remove_bangs, false)
    }
    
    pub fn remove_processing_instructions(&self) -> bool {
        puf_arg!(self.minify, remove_processing_instructions, false)
    }
}
// Server
impl PufferfishConfig {
    pub fn server_port(&self) -> String {
        puf_arg_string!(self.server, port, "8080".to_string())
    }
}

#[derive(PartialEq)]
pub enum MinifyMethod {
    Default,
    /// Use onepass minifier
    Onepass
}

fn file_name_from_path(file_path: &str) -> &str {
    let r = Regex::new(r".*/?(.*)?").unwrap().find(file_path).unwrap();
    &file_path[r.start()..r.end()]
}

#[macro_export]
/// Get value from a field of a setting that still has to be unwrapped
macro_rules! puf_arg {
    ( $setting: expr, $field: ident, $default_value: expr ) => (
        if let Some(val) = &$setting {
            val.$field.unwrap_or($default_value)
        } else {
            $default_value
        }
    )
}

#[macro_export]
macro_rules! puf_arg_string {
    ( $setting: expr, $field: ident, $default_value: expr ) => (
        if let Some(val) = &$setting {
            val.$field.clone().unwrap_or($default_value)
        } else {
            $default_value
        }
    )
}