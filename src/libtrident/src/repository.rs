pub const LABEL_CURSEFORGE: &str = "curseforge";
pub const LABEL_MODRINTH: &str = "modrinth";
pub const LABEL_GITHUB: &str = "github";

pub trait Repository {
    const LABEL: &'static str;

    async fn search(keywords: &str, page: usize, size: usize, filter: Filter);

    async fn resolve(namespace: Option<&str>, identity: &str, version: Option<&str>, filter: Filter);
}

#[derive(Default)]
pub struct Filter {
    game_version: Option<String>,
    mod_loader: Option<String>,
}