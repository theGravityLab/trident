use crate::resolve::ResolveError;
use crate::resource::{Project, Version};

pub trait Repository {
    fn id(&self) -> &'static str;
    fn search(&self);
    fn get_project(&self, project_id: &str) -> Result<Project, ResolveError>;
    fn get_versions(&self, project_id: &str) -> Result<Vec<String>, ResolveError>;
    fn get_version(&self, project_id: &str, version_id: &str) -> Result<Version, ResolveError>;
}
