use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use regex::Regex;
use crate::cli::build::build_html::build_html_file;
use crate::file_listener::FileListener;
use crate::pufferfish_core::config::PufferfishConfig;

/// Starts the file listeners building files as they get updated.
///
/// The `JoinHandle`s need to be joined
pub fn start_listeners(config: &PufferfishConfig) -> (JoinHandle<()>, JoinHandle<()>) {
    let listener = FileListener::new(config.clone());
    let listener = Arc::new(RwLock::new(listener));
    
    let listener_html = listener.clone();
    let listener_html_closure = listener.clone();
    let config_html = config.clone();
    let html_thread = thread::spawn(move || {
        if config_html.verbose() {
            eprintln!("Starting html listener");
        }
        FileListener::start_html(listener_html, &config_html, |file_name| {
            if config_html.verbose() {
                eprintln!("{file_name} changed.");
            }
            let listener = listener_html_closure.read().unwrap();
            let list = &listener.dependant_list;
            let file_path = PathBuf::from(file_name);
            let relative_file_path = file_path.strip_prefix(std::env::current_dir().unwrap()).unwrap();
            build_html_file(
                PathBuf::from(relative_file_path).strip_prefix(config_html.html_dir()).unwrap(),
                &config_html, false, true);
        });
    });
    
    let listener_templ = listener.clone();
    let listener_templ_closure = listener.clone();
    let config_templ = config.clone();
    let verbose_templ = config.verbose();
    let templ_thread = thread::spawn(move || {
        if verbose_templ {
            eprintln!("Starting template listener");
        }
        // TODO: return PathBuf in closures instead of String
        FileListener::start_templates(listener_templ, &config_templ, |file_name| {
            let listener = listener_templ_closure.read().unwrap();
            let list = &listener.dependant_list;
            let file_path = PathBuf::from(file_name);
            let relative_file_path = file_path.strip_prefix(std::env::current_dir().unwrap()).unwrap();
            // Not for templates build_html_file(&file_path, &config_templ, false, true);
            if let Some(dependants) = list.get(relative_file_path.to_str().unwrap()) {
                println!("Building dependants: {:?}", dependants);
                for dependant in dependants {
                    if Regex::new(&format!("{}{}.*", config_templ.html_dir(), std::path::MAIN_SEPARATOR)).unwrap().is_match(dependant) {
                        let file_path = PathBuf::from(dependant);
                        let file_path = file_path.strip_prefix(config_templ.html_dir()).unwrap();
                        build_html_file(&file_path, &config_templ, false, true);
                    }
                }
            }
        });
    });
    
    (html_thread, templ_thread)
}