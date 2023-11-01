use std::collections::HashMap;
use libtrident::repo::Repository;
use libtrident::resolve::ResolveResult;

pub struct CurseForge {}

impl Repository for CurseForge {
    fn id() -> &'static str {
        "curseforge"
    }

    fn search() {
        todo!()
    }

    fn resolve(&self, kind: &str, path: &str, fields: &HashMap<String, String>) -> ResolveResult {
        todo!()
    }
}