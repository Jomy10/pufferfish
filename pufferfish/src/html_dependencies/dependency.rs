use std::path::{PathBuf};

#[derive(Debug, Clone)]
pub struct File {
    pub path: PathBuf,
    pub dependencies: Option<Vec<File>>
}

impl File {
    pub fn new(path: PathBuf, dependencies: Option<Vec<File>>) -> Self {
        Self {
            path,
            dependencies
        }
    }
    
    pub fn add_dependency(&mut self, dependency: File) {
        match self.dependencies.as_mut() {
            Some(dependencies) => {dependencies.push(dependency)}
            None => {
                self.dependencies = Some(Vec::new());
                unsafe { self.add_dependency_unchecked(dependency) }
            }
        }
    }
    
    pub unsafe fn add_dependency_unchecked(&mut self, dependency: File) {
        self.dependencies.as_mut().unwrap_unchecked().push(dependency);
    }
}
