use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
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
    
    fn read_dependencies(&mut self) {
        self.dependant_list.add_all(dependency_graph::generate(&self.config));
    }
    
    // TODO: add closure that gets called on updates to be called to refresh the server
    //       also for `start_templates`
    /// Starts listening for file changes in html dir
    pub fn start_html<Closure: Fn(&str) -> () + Copy>(this: Arc<Mutex<FileListener>>, tx: Sender<()>, config: &PufferfishConfig, on_change: Closure) {
        let (tx_watcher, rx) = channel();
        let mut watcher_html = watcher(tx_watcher, Duration::from_secs(10)).unwrap();
        watcher_html.watch(config.html_dir(), RecursiveMode::Recursive).unwrap();
    
        loop {
            match rx.recv() {
                Ok(event) => {
                    this.lock().unwrap().handle_event(event, on_change);
                    tx.send(()).unwrap();
                }, // TODO: If matches /.*~/ -> ignore, else -> match NoticeWriter, NoticeRemove or Wirte, Remove (potentially others) => Act accordingly
                Err(e) => eprintln!("File listener error: {:?}", e)
            }
        }
    }
    
    /// Starts listening for file changes in templates dir
    pub fn start_templates<Closure: Fn(&str) -> () + Copy>(this: Arc<Mutex<FileListener>>, tx: Sender<()>, config: &PufferfishConfig, on_change: Closure) {
        let (tx_watcher, rx) = channel();
        let mut watcher_templates = watcher(tx_watcher, Duration::from_secs(10)).unwrap();
        watcher_templates.watch(config.template_dir(), RecursiveMode::Recursive).unwrap();
        
        loop {
            match rx.recv() {
                Ok(event) => {
                    this.lock().unwrap().handle_event(event, on_change);
                    tx.send(()).unwrap();
                }
                Err(e) => eprintln!("File listener error: {:?}", e)
            }
        }
    }
    
    fn handle_event<Closure: Fn(&str) -> ()>(&mut self, event: DebouncedEvent, on_change: Closure) {
        match event {
            DebouncedEvent::NoticeWrite(file) | DebouncedEvent::Write(file) |
            DebouncedEvent::NoticeRemove(file) | DebouncedEvent::Remove(file) |
            DebouncedEvent::Create(file) => {
                if !Regex::new(r".*~").unwrap().is_match(file.to_str().unwrap()) {
                    // file is changed
                    // TODO: update only part of the listened files instead of recalculating all
                    //          -> Currently doesn't scale well for big projects
                    *self = Self::new(self.config.clone());
                    on_change(file.to_str().unwrap());
                }
            }
            DebouncedEvent::Rename(from_file, to_file) => {
                // TODO: update only part of the listened files instead of recalculating all
                *self = Self::new(self.config.clone());
                on_change(from_file.to_str().unwrap());
            }
            DebouncedEvent::Chmod(_file) => { /*Ignored*/ }
            DebouncedEvent::Rescan => {
                eprintln!("{}", red!("Rescan appeared."))
            }
            DebouncedEvent::Error(error, file) => {
                match file {
                    Some(file) => {
                        eprintln!("{}", red!(&format!("Error occurred: {}, for file {:?}", error, file)))
                    }
                    None => {
                        eprintln!("{}", red!(&format!("Error occured: {}", error)))
                    }
                }
            }
        }
    }
}
