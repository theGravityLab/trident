use chrono::{DateTime, Utc};
use ron::de::SpannedError;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Default)]
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

// 需要有一种方法来求 Metadata 的 digest，用于 polylock 有效性验证
#[derive(Serialize, Deserialize, Debug, Default, Hash)]
pub struct Metadata {
    pub components: Vec<Component>,
    pub attachments: Vec<Layer>,
}

impl Metadata {
    pub fn digest(&self) -> String {
        todo!()
    }
}

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Component {
    pub id: String,
    pub version: String,
}

impl Component {
    pub fn new(id: &str, version: &str) -> Self {
        Self {
            id: id.to_string(),
            version: version.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Hash)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TimelinePoint {
    time: DateTime<Utc>,
    action: Action,
    result: ActionResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Create(String),
    Restore,
    Play,
    Update(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ActionResult {
    Done,
    Finish(DateTime<Utc>),
    Fail(DateTime<Utc>),
}
