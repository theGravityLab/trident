use std::collections::HashMap;
use crate::resolve::{ResolveError, ResolveResult};

pub trait Repository {
    fn id(&self) -> &'static str;
    fn search(&self);
    fn resolve(&self, kind: &str, path: &str, fields: &HashMap<String, String>) -> ResolveResult {
        Err(ResolveError::Unsupported)
    }
}