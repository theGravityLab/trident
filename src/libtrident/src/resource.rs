// Resource 从 poly-res 中解析出来
// ShaderPack, ResourcePack, World, Mod, Modpack...
// 对上述类型解析得到的 resource 中还包含类型为 File 和 Update 的 poly-res，Update 则能解析出 [File]

// poly-res://mod@curseforge/1919?version=810

use url::Url;
use crate::Res;

pub struct Resource {
    pub project_id: String,
    pub version_id: String,
    pub kind: ResourceKind,
}

pub enum ResourceKind {
    Item(Item),
    File(File),
    Update(Update),
}

pub struct Item {
    pub project_name: String,
    pub version_name: String,
    pub author: String,
    pub summary: String,
    pub kind: ItemKind,
    pub file: Res,
    pub update: Res,
}

pub enum ItemKind {
    Modpack,
    Mod,
    World,
    DataPack,
    ShaderPack,
    ResourcePack,
}

pub struct File {
    pub filename: String,
    pub hash: String,
    pub source: Url,
}

pub struct Update {
    pub versions: Vec<Res>,
}
