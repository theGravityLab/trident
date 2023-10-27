use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Instance {
    pub name: String,
    pub author: String,
    pub summary: String,
    pub thumbnail: Option<Url>,
    pub reference: Option<Url>,
    pub metadata: Metadata,
    pub timeline: Vec<TimelinePoint>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    pub components: Vec<Component>,
    pub attachments: Vec<Layer>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Component {
    pub id: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Layer {
    pub id: String,
    pub from: Option<Url>,
    pub summary: String,
    pub enabled: bool,
    pub content: Vec<Url>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimelinePoint {
    time: DateTime<Utc>,
    action: Action,
    result: Result,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Create(Url),
    Restore,
    Play,
    Update(Url),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Result {
    Finish(DateTime<Utc>),
    Fail(DateTime<Utc>),
}
