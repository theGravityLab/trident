use std::fmt::{Display, Formatter};

use crate::resolve::ResolveError;
use crate::resources::{Package, ResourceKind};

pub mod curseforge;
pub mod modrinth;

pub trait Repository {
    const LABEL: RepositoryLabel;
    fn search(keyword: &str, context: &RepositoryContext);
    fn resolve(
        project_id: &str,
        version_id: &str,
        context: &RepositoryContext,
    ) -> Result<Package, ResolveError>;
}

pub struct RepositoryContext {
    pub client: reqwest::blocking::Client,
    pub game_version: Option<String>,
    pub mod_loader: Option<String>,
    pub kind: Option<ResourceKind>,
}

#[derive(Debug, Clone, Copy)]
pub enum RepositoryLabel {
    CurseForge,
    Modrinth,
}

pub const LABEL_CURSEFORGE: &str = "curseforge";
pub const LABEL_MODRINTH: &str = "modrinth";

impl Display for RepositoryLabel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TryFrom<&str> for RepositoryLabel {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            LABEL_CURSEFORGE => Ok(RepositoryLabel::CurseForge),
            LABEL_MODRINTH => Ok(RepositoryLabel::Modrinth),
            _ => Err(())
        }
    }
}

impl Into<&str> for RepositoryLabel {
    fn into(self) -> &'static str {
        match self {
            RepositoryLabel::CurseForge => LABEL_CURSEFORGE,
            RepositoryLabel::Modrinth => LABEL_MODRINTH
        }
    }
}