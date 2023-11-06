use crate::resolve::ResolveError;
use crate::resources::{Package, ResourceKind};
pub trait Repository {
    fn id(&self) -> &'static str;
    fn search(&self, keyword: &str, context: &RepositoryContext);
    fn resolve(
        &self,
        project_id: &str,
        version_id: &str,
        context: &RepositoryContext,
    ) -> Result<Package, ResolveError>;
}

pub struct RepositoryContext {
    pub game_version: Option<String>,
    pub mod_loader: Option<String>,
    pub kind: Option<ResourceKind>,
}
