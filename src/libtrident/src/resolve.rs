use std::rc::Rc;
use crate::repo::Repository;
use crate::packages::{Package};
use packageurl::PackageUrl;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("Resource is not found in the repository")]
    NotFound,
    #[error("Cannot reach the resource due to network error")]
    UnstableNetwork,
    #[error("String should be formatted as package url")]
    InvalidFormat,
    #[error("Repository specified is not supported")]
    Unsupported,
}

pub struct ResolveEngine {
    repo_factory: fn(&str) -> Option<Rc<dyn Repository>>,
    tasks: Vec<String>,
}

impl ResolveEngine {
    pub fn new(tasks: Vec<String>, repo_factory: fn(&str) -> Option<Rc<dyn Repository>>) -> Self {
        Self {
            repo_factory,
            tasks,
        }
    }
}

impl Iterator for ResolveEngine {
    type Item = ResolveHandle;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.tasks.pop() {
            Some(ResolveHandle {
                task: item,
                repo_factory: self.repo_factory,
            })
        } else {
            None
        }
    }
}

pub struct ResolveHandle {
    repo_factory: fn(&str) -> Option<Rc<dyn Repository>>,
    task: String,
}

impl ResolveHandle {
    pub fn perform(&self) -> Result<Package, ResolveError> {
        if let Ok(purl) = PackageUrl::from_str(&self.task) {
            if let Some(vid) = purl.version() {
                let rid = purl.ty();
                let pid = purl.name();
                let f = self.repo_factory;
                if let Some(repo) = f(rid) {
                    repo.resolve(pid, vid)
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