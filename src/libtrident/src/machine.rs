mod store;

use std::path::PathBuf;
use thiserror::Error;

use crate::{instance::Instance, profile::Profile};

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

pub struct InstantMachine {
    root: PathBuf,
}

impl InstantMachine {
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self { root: root.into() }
    }

    pub fn entries(&self) -> Vec<String> {
        let dir = self.root.join(INSTANCE_DIR);
        if let Ok(read) = dir.read_dir() {
            read.filter_map(|p| p.ok())
                .filter(|p| {
                    p.path().is_file()
                        && p.path().extension().map(|e| e.to_str()) == Some(Some("ron"))
                })
                .filter_map(|p| {
                    p.path()
                        .file_stem()
                        .map(|s| s.to_str().map(|f| f.to_owned()))
                })
                .filter_map(|f| f)
                .collect()
        } else {
            vec![]
        }
    }

    pub fn get_instance(&self, key: &str) -> Result<Instance, MachineError> {
        let file = self.root.join(INSTANCE_DIR).join(format!("{}.ron", key));
        Instance::from_path(file).map_err(|_| MachineError::Unreachable)
    }
}
