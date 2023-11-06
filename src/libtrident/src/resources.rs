use packageurl::PackageUrl;
use url::Url;

// NOTE: 把项目和版本结合在一起会让解析效率变低
// 极端情况下会需要原先两倍时间。当包特别多时，两倍问题不大，合理使用缓存能大幅度减少解析时间。

#[derive(Copy, Clone)]
pub enum ResourceKind {
    ModPack,
    Mod,
    World,
    DataPack,
    ResourcePack,
    ShaderPack,
}

#[derive(Clone)]
pub struct Package {
    pub project_id: String,
    pub project_name: String,
    pub version_id: String,
    pub version_name: String,
    pub author: String,
    pub summary: String,
    pub thumbnail: Url,
    pub reference: Url,
    pub kind: ResourceKind,
    pub filename: String,
    pub download: Url,
    pub hash: Option<String>,
    pub dependencies: Option<Vec<Dependency>>,
    pub requirements: Option<Vec<Requirement>>,
}

impl Package {
    pub fn make_purl(repo: &str, project_id: &str, version_id: &str) -> String {
        PackageUrl::new(repo, project_id)
            .unwrap()
            .with_version(version_id)
            .to_string()
    }
}

#[derive(Clone)]
pub struct Dependency {
    pub purl: String,
    pub required: bool,
}

#[derive(Clone)]
pub enum Requirement {
    Versioned(String, Vec<String>),
    Compatible(Vec<String>),
}
