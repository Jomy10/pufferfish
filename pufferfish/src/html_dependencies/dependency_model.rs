use std::path::Path;

pub struct File<P: AsRef<Path>> {
    path: P,
    dependencies: Option<Vec<File<P>>>
}

impl<P: AsRef<Path>> File<P> {
    pub fn new(path: P, dependencies: Option<Vec<File<P>>>) -> Self {
        Self {
            path,
            dependencies
        }
    }
    
    pub fn add_dependency(&mut self, dependency: File<P>) {
        self.dependencies.as_mut().unwrap_or(&mut Vec::<File<P>>::new()).push(dependency);
    }
    
    pub unsafe fn add_dependency_unckecked(&mut self, dependency: File<P>) {
        self.dependencies.as_mut().unwrap_unchecked().push(dependency);
    }
}
