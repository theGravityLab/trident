use url::Url;

pub struct Project {
    pub id: String,
    pub name: String,
    pub author: String,
    pub summary: String,
    pub thumbnail: Url,
}

pub struct Version {}
