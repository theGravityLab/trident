use crate::repo::{Repository, RepositoryContext, RepositoryLabel};
use crate::resolve::ResolveError;
use crate::resources::Package;

pub struct Modrinth {}

impl Repository for Modrinth {
    const LABEL: RepositoryLabel = RepositoryLabel::Modrinth;

    fn search(keyword: &str, context: &RepositoryContext) {
        todo!()
    }

    fn resolve(
        project_id: &str,
        version_id: &str,
        context: &RepositoryContext,
    ) -> Result<Package, ResolveError> {
        todo!()
    }
}
