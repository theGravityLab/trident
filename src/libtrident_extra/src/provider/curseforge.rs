use libtrident::repository::{Filter, Repository, LABEL_CURSEFORGE};

pub struct CurseForgeRepository {}

impl CurseForgeRepository {
    pub fn new() -> Self {
        todo!()
    }
}

impl Repository for CurseForgeRepository {
    const LABEL: &'static str = LABEL_CURSEFORGE;

    async fn search(keywords: &str, page: usize, size: usize, filter: Filter) {
        todo!()
    }

    async fn resolve(namespace: Option<&str>, identity: &str, version: Option<&str>, filter: Filter) {
        todo!()
    }
}