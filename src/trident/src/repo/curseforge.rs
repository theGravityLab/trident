use std::thread::sleep;
use std::time::Duration;
use libtrident::repo::Repository;
use libtrident::resolve::ResolveError;
use libtrident::packages::{Package, PackageKind,};
use url::Url;

pub struct CurseForge {}

impl CurseForge {
    pub fn new() -> Self {
        Self {}
    }
}

impl Repository for CurseForge {
    fn id(&self) -> &'static str {
        "curseforge"
    }

    fn search(&self) {
        todo!()
    }

    fn get_versions(&self, project_id: &str) -> Result<Vec<String>, ResolveError> {
        todo!()
    }

    fn resolve(&self, project_id: &str, version_id: &str) -> Result<Package, ResolveError> {
        sleep(Duration::from_secs(1));
        Ok(Package {
            project_id: project_id.to_string(),
            version_id: version_id.to_string(),
            project_name: "Summon All Monsters".to_string(),
            version_name: "1.1".to_string(),
            download: Url::parse("https://www.example.com/file.zip").unwrap(),
            filename: "".to_string(),
            hash: None,
            kind: PackageKind::Mod,
            summary: "Hello World".to_string(),
            author: "Me".to_string(),
            thumbnail: Url::parse("https://www.example.com/icon.png").unwrap(),
            dependencies: None,
            requirements: None,
        })
    }
}
