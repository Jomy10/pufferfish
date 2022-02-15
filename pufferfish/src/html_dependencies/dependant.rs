//! Models a file with all of its dependants to be able to update them when necessary.

use std::collections::HashMap;
use std::path::{PathBuf};
use crate::html_dependencies::dependency;

/*
pub fn new_dependants_list<'a>() -> HashMap<String, Vec<String>> {
    HashMap::new()
}

pub fn add_as_dependant<'a>(dependants_list: &mut  HashMap<String, Vec<String>>, file: dependency::File) {
    let current = file.path.to_str().unwrap();
    
    if file.dependencies.is_some() {
        for dep in unsafe { file.dependencies.unwrap_unchecked() } {
            if let Some(depandants) = dependants_list.get(&*dep.path.to_str().unwrap()) {
                let dependant_to_add = current.to_string();
                // Make sure we don't add duplicates
                if !depandants.contains(&dependant_to_add) {
                    // Add the dependant
                    unsafe { dependants_list.get_mut(&*dep.path.to_str().unwrap()).unwrap_unchecked() }.push(dependant_to_add);
                }
                
            } else {
                dependants_list.insert(
                    dep.path.to_str().unwrap().to_string(),
                    [current.to_string()].to_vec()
                );
            }

            add_as_dependant(dependants_list, dep);
        }
    }
}
*/

/// A list of all the dependants of a specific file
pub trait DependantList {
    fn new_dependant_list() -> Self;
    fn add_as_dependant(&mut self, file: dependency::File);
    fn add_all(&mut self, dependants: Vec<dependency::File>);
    fn get_dependants(&self, key: &str) -> Vec<String>;
}

impl DependantList for HashMap<String, Vec<String>> {
    fn new_dependant_list() -> Self {
        HashMap::new()
    }
    
    fn add_as_dependant(&mut self, file: dependency::File) {
        let current = file.path.to_str().unwrap();
        
        if file.dependencies.is_some() {
            for dep in unsafe { file.dependencies.unwrap_unchecked() } {
                if let Some(depandants) = self.get(&*dep.path.to_str().unwrap()) {
                    let dependant_to_add = current.to_string();
                    // Make sure we don't add duplicates
                    if !depandants.contains(&dependant_to_add) {
                        // Add the dependant
                        unsafe { self.get_mut(&*dep.path.to_str().unwrap()).unwrap_unchecked() }.push(dependant_to_add);
                    }
                    
                } else {
                    self.insert(
                        dep.path.to_str().unwrap().to_string(),
                        [current.to_string()].to_vec()
                    );
                }
                
                self.add_as_dependant(dep);
            }
        }
    }
    
    fn add_all(&mut self, dependants: Vec<dependency::File>) {
        for file in dependants {
            self.add_as_dependant(file);
        }
    }
    
    fn get_dependants(&self, key: &str) -> Vec<String> {
        if let Some(dependants) = self.get(key) {
            let dependants = dependants.iter().map(|dependant| {
                // Add dependants of dependant, because they also depend on the file defined by key
                let mut dependants_of_dependant: Vec<String> = self.get_dependants(dependant);
                // Add the dependant
                dependants_of_dependant.push(dependant.to_string());
                dependants_of_dependant
            }).collect::<Vec<Vec<String>>>();
            dependants.into_iter().flat_map(|dependants_vec| dependants_vec).collect::<Vec<String>>()
        } else {
            return Vec::<String>::new();
        }
    }
}


struct File {
    pub path: PathBuf,
    dependants: Option<Vec<File>>
}

impl File {
    pub fn new(path: PathBuf, dependants: Option<Vec<File>>) -> Self {
        Self {
            path,
            dependants
        }
    }
    
    pub fn add_dependant(&mut self, dependant: File) {
        match self.dependants.as_mut() {
            Some(dependants) => dependants.push(dependant),
            None => {
                self.dependants = Some(Vec::new());
                unsafe { self.add_dependant_unchecked(dependant) }
            }
        }
    }
    
    pub unsafe fn add_dependant_unchecked(&mut self, dependant: File) {
        self.dependants.as_mut().unwrap_unchecked().push(dependant);
    }
}

