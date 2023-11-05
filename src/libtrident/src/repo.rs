use crate::resolve::ResolveError;
use crate::packages::{Package};
pub trait Repository {
    fn id(&self) -> &'static str;
    fn search(&self);
    fn get_versions(&self, project_id: &str) -> Result<Vec<String>, ResolveError>;
    fn resolve(&self, project_id: &str, version_id: &str) -> Result<Package, ResolveError>;
}
