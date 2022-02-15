pub mod config;
pub mod cli;
pub mod parser;
pub use puf_dirs::*;
pub mod build_html;
#[cfg(feature = "server")]
pub mod dev_server;
pub mod html_dependencies;
#[cfg(feature = "file_listener")]
pub mod file_listener;
pub mod html_beautifier;

mod puf_dirs {
    use std::fs;
    use std::path::Path;
    
    pub fn make_dirs(cache_dir: &Path) {
        if !cache_dir.exists() {
            if let Err(err) = fs::create_dir(cache_dir) {
                panic!("Couldn't create cache directory {}", err.to_string())
            }
        }
        let template_dir = cache_dir.join("templates");
        if !template_dir.exists() {
            if let Err(err) = fs::create_dir(template_dir) {
                panic!("Couldn't create cached template directory {}", err.to_string())
            }
        }
    }
}
