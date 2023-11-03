use crate::repo::Repository;
use crate::resource::Version;
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

#[derive(Default)]
pub struct ResolveEngine {
    repositories: Vec<Box<dyn Repository>>,
    tasks: Vec<String>,
}

impl ResolveEngine {
    pub fn add_repository<R: Repository + 'static>(&mut self, repo: R) {
        self.repositories.push(Box::new(repo))
    }

    pub fn add_task(&mut self, res: String) {
        self.tasks.push(res)
    }
}

impl IntoIterator for ResolveEngine {
    type Item = Result<Version, ResolveError>;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { engine: self }
    }
}

pub struct IntoIter {
    engine: ResolveEngine,
}

impl Iterator for IntoIter {
    type Item = Result<Version, ResolveError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.engine.tasks.pop() {
            if let Ok(purl) = PackageUrl::from_str(&item) {
                if let Some(vid) = purl.version() {
                    let rid = purl.ty();
                    let pid = purl.name();
                    for repo in &self.engine.repositories {
                        if repo.id() == rid {
                            return Some(repo.get_version(pid, vid));
                        }
                    }
                    Some(Err(ResolveError::NotFound))
                } else {
                    Some(Err(ResolveError::InvalidFormat))
                }
            } else {
                Some(Err(ResolveError::InvalidFormat))
            }
        } else {
            None
        }
    }
}
