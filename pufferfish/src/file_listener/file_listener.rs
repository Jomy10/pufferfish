use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Duration;
use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};
use regex::Regex;
use simple_colors::red;
use crate::config::PufferfishConfig;
use crate::html_dependencies::dependant::DependantList;
use crate::html_dependencies::dependency_graph;

#[derive(Debug)]
pub struct FileListener {
    pub config: PufferfishConfig,
    pub dependant_list: HashMap<String, Vec<String>>
}

impl FileListener {
    pub fn new(config: PufferfishConfig) -> Self {
        let mut s = Self {
            config,
            dependant_list: HashMap::new_dependant_list()
        };
        s.read_dependencies();
        s
    }
    
    pub fn read_dependencies_clean(&mut self) {
        self.dependant_list = HashMap::new_dependant_list();
        self.read_dependencies();
    }
    
    fn read_dependencies(&mut self) {
        self.dependant_list.add_all(dependency_graph::generate(&self.config));
    }
    
    // TODO: add closure that gets called on updates to be called to refresh the server
    //       also for `start_templates`
    /// Starts listening for file changes in html dir
    pub fn start_html<Closure: Fn(&str) -> () + Copy>(listener: Arc<RwLock<FileListener>>, config: &PufferfishConfig, on_change: Closure) {
        let (tx_watcher, rx) = channel();
        let mut watcher_html = watcher(tx_watcher, Duration::from_secs(10)).unwrap();
        watcher_html.watch(config.html_dir(), RecursiveMode::Recursive).unwrap();
    
        loop {
            match rx.recv() {
                Ok(event) => {
                    let file = {
                        let mut _listener = listener.write().unwrap();
                        _listener.handle_event(event)
                    };
                    if let Some(file) = file {
                        on_change(&file);
                    }
                },
                Err(e) => eprintln!("File listener error: {:?}", e)
            }
        }
    }
    
    /// Starts listening for file changes in templates dir
    pub fn start_templates<Closure: Fn(&str) -> () + Copy>(listener: Arc<RwLock<FileListener>>, config: &PufferfishConfig, on_change: Closure) {
        let (tx_watcher, rx) = channel();
        let mut watcher_templates = watcher(tx_watcher, Duration::from_secs(10)).unwrap();
        watcher_templates.watch(config.template_dir(), RecursiveMode::Recursive).unwrap();
        
        loop {
            match rx.recv() {
                Ok(event) => {
                    let file = {
                       listener.write().unwrap().handle_event(event)
                    };
                    if let Some(file) = file {
                        on_change(&file);
                    }
                }
                Err(e) => eprintln!("File listener error: {:?}", e)
            }
        }
    }
   
    /// Handle a file change event
    ///
    /// # Returns
    /// - The file if it was the correct event
    fn handle_event(&mut self, event: DebouncedEvent) -> Option<String> {
        match event {
            DebouncedEvent::NoticeWrite(file) | DebouncedEvent::Write(file) |
            DebouncedEvent::NoticeRemove(file) | DebouncedEvent::Remove(file) |
            DebouncedEvent::Create(file) => {
                println!("Handling event");
                if !Regex::new(r".*~").unwrap().is_match(file.to_str().unwrap()) {
                    // file is changed
                    // TODO: update only part of the listened files instead of recalculating all
                    //          -> Currently doesn't scale well for big projects
                    *self = Self::new(self.config.clone());
                    Some(file.to_str().unwrap().to_string())
                } else {
                    println!("temp change");
                    None
                }
            }
            DebouncedEvent::Rename(from_file, to_file) => {
                // TODO: update only part of the listened files instead of recalculating all
                *self = Self::new(self.config.clone());
                Some(from_file.to_str().unwrap().to_string())
            }
            DebouncedEvent::Chmod(_file) => { /*Ignored*/
                None
            }
            DebouncedEvent::Rescan => {
                eprintln!("{}", red!("Rescan appeared."));
                None
            }
            DebouncedEvent::Error(error, file) => {
                match file {
                    Some(file) => {
                        eprintln!("{}", red!(&format!("Error occurred: {}, for file {:?}", error, file)));
                        None
                    }
                    None => {
                        eprintln!("{}", red!(&format!("Error occured: {}", error)));
                        None
                    }
                }
            }
        }
    }
}
