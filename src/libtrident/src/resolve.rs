use crate::repo::{Repository, RepositoryContext};
use crate::resources::Package;
use packageurl::PackageUrl;
use std::rc::Rc;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("Unspecific error")]
    Unknown,
    #[error("Resource is not found in the repository")]
    NotFound,
    #[error("Cannot reach the resource due to network error")]
    UnstableNetwork,
    #[error("String should be formatted as package url")]
    InvalidFormat,
    #[error("Repository specified is not supported")]
    Unsupported,
    #[error("Response is invalid or cannot parsed into")]
    UnableToParse,
}

pub struct ResolveEngine {
    repos: Rc<Vec<Box<dyn Repository>>>,
    tasks: Vec<String>,
}

impl ResolveEngine {
    pub fn new(tasks: Vec<String>, repos: Rc<Vec<Box<dyn Repository>>>) -> Self {
        Self { repos, tasks }
    }
}

impl Iterator for ResolveEngine {
    type Item = ResolveHandle;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.tasks.pop() {
            Some(ResolveHandle {
                task: item,
                repos: Rc::clone(&self.repos),
            })
        } else {
            None
        }
    }
}

pub struct ResolveHandle {
    repos: Rc<Vec<Box<dyn Repository>>>,
    task: String,
}

impl ResolveHandle {
    pub fn perform(&self, context: &RepositoryContext) -> Result<Package, ResolveError> {
        if let Ok(purl) = PackageUrl::from_str(&self.task) {
            if let Some(vid) = purl.version() {
                let rid = purl.ty();
                let pid = purl.name();
                if let Some(repo) = self.repos.iter().find(|r| r.id() == rid) {
                    repo.resolve(pid, vid, context)
                } else {
                    Err(ResolveError::NotFound)
                }
            } else {
                Err(ResolveError::InvalidFormat)
            }
        } else {
            Err(ResolveError::InvalidFormat)
        }
    }

    pub fn task(&self) -> &str {
        self.task.as_str()
    }
}
