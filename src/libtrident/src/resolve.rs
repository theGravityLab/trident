use std::rc::Rc;
use std::str::FromStr;

use packageurl::PackageUrl;
use thiserror::Error;

use crate::repo::curseforge::CurseForge;
use crate::repo::modrinth::Modrinth;
use crate::repo::{Repository, RepositoryContext, RepositoryLabel};
use crate::resources::Package;

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
    tasks: Vec<String>,
}

impl ResolveEngine {
    pub fn new(tasks: Vec<String>) -> Self {
        Self {
            tasks,
        }
    }
}

impl Iterator for ResolveEngine {
    type Item = ResolveHandle;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.tasks.pop() {
            Some(ResolveHandle { task: item })
        } else {
            None
        }
    }
}

pub struct ResolveHandle {
    task: String,
}

impl ResolveHandle {
    pub fn perform(&self, context: &RepositoryContext) -> Result<Package, ResolveError> {
        if let Ok(purl) = PackageUrl::from_str(&self.task) {
            if let Some(vid) = purl.version() {
                let rid = purl.ty();
                let pid = purl.name();
                if let Ok(repo) = RepositoryLabel::try_from(rid) {
                    match repo {
                        RepositoryLabel::CurseForge => CurseForge::resolve(pid, vid, context),
                        RepositoryLabel::Modrinth => Modrinth::resolve(pid, vid, context),
                    }
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
