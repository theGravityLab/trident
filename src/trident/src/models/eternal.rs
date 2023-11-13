use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response<T> {
    pub data: T,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mod {
    /// The mod id
    pub id: i32,
    /// The game id this mod is for
    pub game_id: i32,
    /// The name of the mod
    pub name: String,
    /// The mod slug that would appear in the URL
    pub slug: String,
    /// Relevant links for the mod such as Issue tracker and Wiki
    pub links: ModLinks,
    /// Mod summary
    pub summary: String,
    /// Current mod status
    pub status: ModStatus,
    /// Number of downloads for the mod
    pub download_count: i64,
    /// Whether the mod is included in the featured mods list
    pub is_featured: bool,
    /// The main category of the mod as it was chosen by the mod author
    pub primary_category_id: i32,
    /// List of categories that this mod is related to
    pub categories: Vec<Category>,
    /// The class id this mod belongs to
    ///
    /// **NOTE** It may be null which causing parsing failure intentionally
    pub class_id: i32,
    /// List of the mod's authors
    pub authors: Vec<ModAuthor>,
    /// The mod's logo.png asset
    pub logo: ModAsset,
    /// List of screenshots assets
    pub screenshots: Vec<ModAsset>,
    /// The id of the main file of the mod
    pub main_file_id: i32,
    /// List of latest files of the mod
    pub latest_files: Vec<File>,
    /// List of file related details for the latest files of the mod
    pub latest_files_indexes: Vec<FileIndex>,
    /// List of file related details for the latest early access files of the mod
    pub latest_early_access_files_indexes: Vec<FileIndex>,
    /// The creation date of the mod
    pub date_created: DateTime<Utc>,
    /// The last time the mod was modified
    pub date_modified: DateTime<Utc>,
    /// The release date of the mod
    pub date_released: DateTime<Utc>,
    /// Is mod allowed to be distributed
    pub allow_mod_distribution: Option<bool>,
    /// The mod popularity rank for the game
    pub game_popularity_rank: i32,
    /// Is the mod available for search. This can be false when a mod is experimental, in a deleted state or has only alpha files
    pub is_available: bool,
    /// The mod's thumbs up count
    pub thumbs_up_count: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModLinks {
    pub website_url: Option<Url>,
    // pub wiki_url: Option<Url>, maybe Some("invalid url") which causing parsing failure
    // pub issue_url: Option<Url>,
    // pub source_url: Option<Url>,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum ModStatus {
    New = 1,
    ChangesRequired = 2,
    UnderSoftReview = 3,
    Approved = 4,
    Rejected = 5,
    ChangesMade = 6,
    Inactive = 7,
    Abandoned = 8,
    Deleted = 9,
    UnderReview = 10,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    /// The category id
    pub id: i32,
    /// The game id related to the category
    pub game_id: i32,
    /// Category name
    pub name: String,
    /// The category slug as it appear in the URL
    pub slug: String,
    /// The category URL
    pub url: Url,
    /// URL for the category icon
    pub icon_url: Url,
    /// Last modified date of the category
    pub date_modified: DateTime<Utc>,
    /// A top level category for other categories
    pub is_class: Option<bool>,
    /// The class id of the category, meaning - the class of which this category is under
    pub class_id: Option<i32>,
    /// The parent category for this category
    pub parent_category_id: Option<i32>,
    /// The display index for this category
    pub display_index: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAuthor {
    pub id: i32,
    pub name: String,
    pub url: Url,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModAsset {
    pub id: i32,
    pub mod_id: i32,
    pub title: String,
    pub description: String,
    pub thumbnail_url: Url,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// The file id
    pub id: i32,
    /// The game id related to the mod that this file belongs to
    pub game_id: i32,
    /// The mod id
    pub mod_id: i32,
    /// Whether the file is available to download
    pub is_available: bool,
    /// Display name of the file
    pub display_name: String,
    /// Exact file name
    pub file_name: String,
    /// The file release type
    pub release_type: FileReleaseType,
    /// Status of the file
    pub file_status: FileStatus,
    /// The file hash (i.e. md5 or sha1)
    pub hashes: Vec<FileHash>,
    /// The file timestamp
    pub file_date: DateTime<Utc>,
    /// The file length in bytes
    pub file_length: i64,
    /// The number of downloads for the file
    pub download_count: i64,
    /// The file's size on disk
    pub file_size_on_disk: Option<i64>,
    /// The file download URL
    pub download_url: Url,
    /// List of game versions this file is relevant for
    pub game_versions: Vec<String>,
    /// Metadata used for sorting by game versions
    pub sortable_game_versions: Vec<SortableGameVersion>,
    /// List of dependencies files
    pub dependencies: Vec<FileDependency>,
    pub expose_as_alternative: Option<bool>,
    pub parent_project_file_id: Option<i32>,
    pub alternate_file_id: Option<i32>,
    pub is_server_pack: Option<bool>,
    pub server_pack_file_id: Option<i32>,
    pub is_early_access_content: Option<bool>,
    pub early_access_end_date: Option<DateTime<Utc>>,
    pub file_fingerprint: i64,
    pub modules: Vec<FileModule>,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum FileReleaseType {
    Release = 1,
    Beta = 2,
    Alpha = 3,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum FileStatus {
    Processing = 1,
    ChangesRequired = 2,
    UnderReview = 3,
    Approved = 4,
    Rejected = 5,
    MalwareDetected = 6,
    Deleted = 7,
    Archived = 8,
    Testing = 9,
    Released = 10,
    ReadyForReview = 11,
    Deprecated = 12,
    Baking = 13,
    AwaitingPublishing = 14,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileHash {
    pub value: String,
    /// 1 = Sha1
    /// 2 = Md5
    pub algo: HashAlgo,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum HashAlgo {
    Sha1 = 1,
    Md5 = 2,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortableGameVersion {
    /// Original version name (e.g. 1.5b)
    pub game_version_name: String,
    /// Used for sorting (e.g. 0000000001.0000000005)
    pub game_version_padded: String,
    /// game version clean name (e.g. 1.5)
    pub game_version: String,
    /// Game version release date
    pub game_version_release_date: DateTime<Utc>,
    /// Game version type id
    pub game_version_type_id: Option<i32>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileDependency {
    pub mod_id: i32,
    /// 1 = EmbeddedLibrary
    /// 2 = OptionalDependency
    /// 3 = RequiredDependency
    /// 4 = Tool
    /// 5 = Incompatible
    /// 6 = Include
    pub relation_type: FileRelationType,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum FileRelationType {
    EmbeddedLibrary = 1,
    OptionalDependency = 2,
    RequiredDependency = 3,
    Tool = 4,
    Incompatible = 5,
    Include = 6,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileModule {
    pub name: String,
    pub fingerprint: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileIndex {
    pub game_version: String,
    pub file_id: i32,
    pub filename: String,
    /// 1 = Release
    /// 2 = Beta
    /// 3 = Alpha
    pub release_type: FileReleaseType,
    pub game_version_type_id: Option<i32>,
    /// 0 = Any
    /// 1 = Forge
    /// 2 = Cauldron
    /// 3 = LiteLoader
    /// 4 = Fabric
    /// 5 = Quilt
    pub mod_loader: Option<ModLoaderType>,
}

#[derive(Deserialize_repr)]
#[repr(u8)]
pub enum ModLoaderType {
    Any = 0,
    Forge = 1,
    Cauldron = 2,
    LiteLoader = 3,
    Fabric = 4,
    Quilt = 5,
    NeoForge = 6,
}
