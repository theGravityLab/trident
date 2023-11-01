use thiserror::Error;
use crate::repo::Repository;
use crate::Res;
use crate::resource::Resource;

pub type ResolveResult = Result<Resource, ResolveError>;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("This type of resource is not supported to be resolved")]
    Unsupported,
    #[error("Resource is not found in the repository")]
    NotFound,
    #[error("Cannot reach the resource due to network error")]
    UnstableNetwork,
}

pub struct ResolveEngine {
    repositories: Vec<Box<dyn Repository>>,
    tasks: Vec<Res>,
}

impl ResolveEngine {
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
            tasks: Vec::new(),
        }
    }

    pub fn add_repository<R: Repository + Sized>(&mut self, repo: R) {
        self.repositories.push(Box::new(repo))
    }

    pub fn add_task(&mut self, res: Res) {
        self.tasks.push(res)
    }
}

impl IntoIterator for ResolveEngine {
    type Item = ResolveResult;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            repositories: self.repositories,
            tasks: self.tasks,
        }
    }
}

pub struct IntoIter {
    repositories: Vec<Box<dyn Repository>>,
    tasks: Vec<Res>,
}

impl Iterator for IntoIter {
    type Item = ResolveResult;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
