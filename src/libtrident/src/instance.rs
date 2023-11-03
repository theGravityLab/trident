use crate::profile::Profile;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstanceError {
    #[error("File not found")]
    FileNotFound,
    #[error("File system error")]
    FileSystemError,
    #[error("Invalid profile.ron")]
    InvalidProfile,
}

pub struct Instance {
    profile: Profile,
    profile_path: PathBuf,
    home_path: PathBuf,
}

impl Instance {
    pub fn from_path<P: AsRef<Path>>(file: P) -> Result<Self, InstanceError> {
        let path = file.as_ref();
        if path.exists() {
            let home = path.with_extension("");
            if let Ok(profile_content) = fs::read_to_string(path) {
                if let Ok(profile) = ron::from_str(&profile_content) {
                    Ok(Self {
                        home_path: home,
                        profile_path: path.to_path_buf(),
                        profile,
                    })
                } else {
                    Err(InstanceError::InvalidProfile)
                }
            } else {
                Err(InstanceError::FileSystemError)
            }
        } else {
            Err(InstanceError::FileNotFound)
        }
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    pub fn home(&self) -> &Path {
        &self.home_path
    }

    // 得到软连接组成的快照
    pub fn snapshot() {}

    // 将当前状态转移到输入快照描述的状态
    pub fn apply() {}

    // 得到资产清单
    pub fn scan() {}
}
