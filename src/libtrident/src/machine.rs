use crate::deploy::DeployEngine;
use crate::instance::{Instance, InstanceError};
use crate::profile::{Component, Profile};
use sanitize_filename::sanitize;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;
use thiserror::Error;
use crate::repo::Repository;

const INSTANCE_DIR: &str = "instances";
const STORAGE_DIR: &str = "storage";
const CACHE_DIR: &str = "cache";

#[derive(Error, Debug)]
pub enum MachineError {
    #[error("Unknown stands for UNKNOWN")]
    Unknown,
    #[error("Not found or inaccessible")]
    Unreachable,
    #[error("Can't create due to object conflict with the same key({0})")]
    Conflict(String),
    #[error("File system reports an error")]
    FileSystemError,
}

pub struct Machine {
    root: PathBuf,
}

impl Machine {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self { root: root.into() }
    }

    pub fn load_profile(&self, file: &str) -> Result<Profile, MachineError> {
        let path = self.root.join(INSTANCE_DIR).join(file);
        if let Ok(text) = fs::read_to_string(path) {
            if let Ok(profile) = Profile::from_ron(&text) {
                Ok(profile)
            } else {
                Err(MachineError::Unreachable)
            }
        } else {
            Err(MachineError::Unreachable)
        }
    }

    pub fn create_profile(
        &self,
        name: &str,
        author: Option<&str>,
        summary: Option<&str>,
        version: Option<&str>,
    ) -> Result<Profile, MachineError> {
        let file = sanitize(name);
        let path = self.root.join(INSTANCE_DIR).join(format!("{}.ron", file));
        if !path.exists() {
            if let Some(parent) = path.parent() {
                if !parent.exists() && fs::create_dir(parent).is_err() {
                    return Err(MachineError::FileSystemError);
                }
            }
            let mut profile = Profile::default();
            if let Some(author) = author {
                profile.author = author.to_string();
            }
            if let Some(summary) = summary {
                profile.summary = summary.to_string();
            }
            if let Some(version) = version {
                let component = Component::new("net.minecraft", version);
                profile.metadata.components.push(component);
            }
            if fs::write(path, profile.to_ron().unwrap()).is_ok() {
                Ok(profile)
            } else {
                Err(MachineError::FileSystemError)
            }
        } else {
            Err(MachineError::Conflict(path.to_str().unwrap().to_string()))
        }
    }

    pub fn deploy(&self, file: &str, force: bool, max_resolve_depth: usize, repo_factory: fn(&str) -> Option<Rc<dyn Repository>>) -> Result<DeployEngine, MachineError> {
        let path = self.root.join(INSTANCE_DIR).join(file);
        match Instance::from_path(path) {
            Ok(instance) => Ok(DeployEngine::new(instance, force, max_resolve_depth, repo_factory)),
            Err(err) => match err {
                InstanceError::FileNotFound => Err(MachineError::Unreachable),
                _ => Err(MachineError::Unknown),
            },
        }
    }
}
