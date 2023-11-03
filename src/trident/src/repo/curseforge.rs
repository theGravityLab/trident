use libtrident::repo::Repository;
use libtrident::resolve::ResolveError;
use libtrident::resource::{Project, Version};

pub struct CurseForge {}

impl Repository for CurseForge {
    fn id(&self) -> &'static str {
        "curseforge"
    }

    fn search(&self) {
        todo!()
    }

    fn get_project(&self, project_id: &str) -> Result<Project, ResolveError> {
        todo!()
    }

    fn get_versions(&self, project_id: &str) -> Result<Vec<String>, ResolveError> {
        todo!()
    }

    fn get_version(&self, project_id: &str, version_id: &str) -> Result<Version, ResolveError> {
        todo!()
    }
}
