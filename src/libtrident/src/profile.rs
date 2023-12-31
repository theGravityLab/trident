use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use chrono::{DateTime, Utc};
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};
use url::Url;

pub const COMPONENT_MINECRAFT: &str = "net.minecraft";
pub const COMPONENT_FORGE: &str = "net.minecraftforge";
pub const COMPONENT_NEOFORGE: &str = "net.neoforged";
pub const COMPONENT_FABRIC: &str = "net.fabricmc";
pub const COMPONENT_QUILT: &str = "org.quiltmc";
pub const COMPONENT_BUILTIN_STORAGE: &str = "builtin.trident.storage";

pub const LOADERS: [&str; 4] = [
    COMPONENT_FABRIC,
    COMPONENT_QUILT,
    COMPONENT_FORGE,
    COMPONENT_NEOFORGE,
];

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Profile {
    pub name: String,
    pub author: String,
    pub summary: String,
    pub thumbnail: Option<Url>,
    pub reference: Option<String>,
    pub metadata: Metadata,
    pub timeline: Vec<TimelinePoint>,
}

impl Profile {
    pub fn from_ron(text: &str) -> Result<Profile, SpannedError> {
        ron::from_str(text)
    }

    pub fn to_ron(&self) -> Result<String, ron::Error> {
        ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::new().struct_names(true))
    }
}

pub struct Entry {
    pub key: String,
    pub profile: Profile,
}

impl Entry {
    pub fn from_profile(key: String, profile: Profile) -> Self {
        Self {
            key,
            profile,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Hash, Clone)]
pub struct Metadata {
    pub version: String,
    pub loaders: Vec<Loader>,
    pub attachments: Vec<Layer>,
}

impl Metadata {
    pub fn digest(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone)]
pub struct Loader {
    pub id: String,
    pub version: String,
}

impl Loader {
    pub fn new(id: &str, version: &str) -> Self {
        Self {
            id: id.to_string(),
            version: version.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash, Clone)]
pub struct Layer {
    pub from: Option<String>,
    pub summary: String,
    pub enabled: bool,
    pub content: Vec<String>,
}

impl Layer {
    pub fn new(summary: Option<&str>, from: Option<String>) -> Self {
        Layer {
            summary: if let Some(s) = summary {
                s.to_string()
            } else {
                "".to_string()
            },
            from,
            enabled: true,
            content: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimelinePoint {
    time: DateTime<Utc>,
    action: Action,
    result: ActionResult,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    Create(String),
    Restore,
    Play,
    Update(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActionResult {
    Done,
    Finish(DateTime<Utc>),
    Fail(DateTime<Utc>),
}
