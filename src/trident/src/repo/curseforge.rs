use lazy_static::lazy_static;
use libtrident::profile;
use libtrident::repo::{Repository, RepositoryContext};
use libtrident::resolve::ResolveError;
use libtrident::resources::{Dependency, Package, Requirement, ResourceKind};
use reqwest::header::HeaderValue;
use reqwest::{header::HeaderMap, StatusCode};
use serde::de::DeserializeOwned;

use crate::models::eternal::{File, FileRelationType, HashAlgo, Mod, Response};

const API_KEY: &str = "$2a$10$cjd5uExXA6oMi3lSnylNC.xsFJiujI8uQ/pV1eGltFe/hlDO2mjzm";
const ENDPOINT: &str = "https://api.curseforge.com";
const USER_AGENT: &str = "trident/*";

const GAME_ID: i32 = 432;

const CLASS_MOD: i32 = 6;
const CLASS_WORLD: i32 = 17;
const CLASS_MODPACK: i32 = 4471;
const CLASS_RESOURCEPACK: i32 = 12;
const CLASS_SHADERPACK: i32 = 6552;

lazy_static! {
    static ref CLIENT: reqwest::blocking::Client = {
        let mut headers = HeaderMap::default();
        headers.insert("x-api-key", HeaderValue::from_static(API_KEY));
        reqwest::blocking::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .unwrap()
    };
}

pub struct CurseForge {}

impl CurseForge {
    pub fn new() -> Self {
        Self {}
    }

    fn get<R: DeserializeOwned>(&self, service: &str) -> Result<R, reqwest::Error> {
        let req = CLIENT.get(format!("{}{}", ENDPOINT, service));
        let res = req.send()?;
        res.json::<Response<R>>().map(|r| r.data)
    }

    fn into_error(error: reqwest::Error) -> ResolveError {
        if error.is_redirect() || error.is_timeout() {
            ResolveError::UnstableNetwork
        } else if error.is_decode() || error.is_body() {
            ResolveError::UnableToParse
        } else if error.is_status() {
            if let Some(StatusCode::NOT_FOUND) = error.status() {
                ResolveError::NotFound
            } else {
                ResolveError::Unknown
            }
        } else {
            ResolveError::Unknown
        }
    }

    fn into_kind(class_id: i32) -> Option<ResourceKind> {
        match class_id {
            CLASS_MOD => Some(ResourceKind::Mod),
            CLASS_MODPACK => Some(ResourceKind::ModPack),
            CLASS_RESOURCEPACK => Some(ResourceKind::ResourcePack),
            CLASS_SHADERPACK => Some(ResourceKind::ShaderPack),
            CLASS_WORLD => Some(ResourceKind::World),
            _ => None,
        }
    }

    fn into_loader_id(loader_type: &str) -> Option<&str> {
        match loader_type {
            "NeoForge" => Some(profile::COMPONENT_NEOFORGE),
            "Forge" => Some(profile::COMPONENT_FORGE),
            "Fabric" => Some(profile::COMPONENT_FABRIC),
            "Quilt" => Some(profile::COMPONENT_QUILT),
            _ => None,
        }
    }

    fn into_loader_type(loader_id: &str) -> Option<&str> {
        match loader_id {
            profile::COMPONENT_FORGE => Some("Forge"),
            profile::COMPONENT_NEOFORGE => Some("NeoForge"),
            profile::COMPONENT_FABRIC => Some("Fabric"),
            profile::COMPONENT_QUILT => Some("Quilt"),
            _ => None,
        }
    }
}

impl Repository for CurseForge {
    fn id(&self) -> &'static str {
        "curseforge"
    }

    fn search(&self, keyword: &str, context: &RepositoryContext) {
        todo!()
    }

    fn resolve(
        &self,
        project_id: &str,
        version_id: &str,
        context: &RepositoryContext,
    ) -> Result<Package, ResolveError> {
        let p = self
            .get::<Mod>(&format!("/v1/mods/{}", project_id))
            .map_err(Self::into_error)?;
        let v = if version_id != "*" {
            self.get::<File>(&format!("/v1/mods/{}/files/{}", project_id, version_id))
                .map_err(Self::into_error)?
        } else {
            let versions = self
                .get::<Vec<File>>(&format!("/v1/mods/{}/files", project_id))
                .map_err(Self::into_error)?
                .into_iter()
                .filter(|f| {
                    let mut pred = true;
                    if let Some(required) = &context.game_version {
                        pred &= f.game_versions.contains(required);
                    };
                    if let Some(required) = &context.mod_loader {
                        if let Some(loader) = Self::into_loader_type(&required) {
                            pred &= f.game_versions.iter().any(|j| j == loader);
                        }
                    }
                    pred
                });
            versions
                .max_by_key(|x| x.file_date)
                .ok_or(ResolveError::NotFound)?
        };
        let kind = Self::into_kind(p.class_id).ok_or(ResolveError::UnableToParse)?;
        let hash = v
            .hashes
            .iter()
            .find(|h| matches!(h.algo, HashAlgo::Sha1))
            .map(|h| h.value.to_owned());
        let dependencies = if !v.dependencies.is_empty() {
            Some(
                v.dependencies
                    .iter()
                    .map(|d| Dependency {
                        required: matches!(d.relation_type, FileRelationType::RequiredDependency),
                        purl: Package::make_purl(self.id(), &d.mod_id.to_string(), "*"),
                    })
                    .collect(),
            )
        } else {
            None
        };
        let mut requirements = Vec::<Requirement>::new();
        let mut versioned = Vec::<String>::new();
        let mut compatible = Vec::<String>::new();
        for other in v.sortable_game_versions {
            match other.game_version_type_id {
                Some(75125) => versioned.push(other.game_version),
                _ => {
                    if let Some(id) = Self::into_loader_id(&other.game_version_name) {
                        compatible.push(id.to_owned());
                    } else {
                        compatible.push(other.game_version_name)
                    }
                }
            }
        }
        if !versioned.is_empty() {
            requirements.push(Requirement::Versioned(
                "net.minecraft".to_owned(),
                versioned,
            ));
        }
        if !compatible.is_empty() {
            requirements.push(Requirement::Compatible(compatible));
        }
        Ok(Package {
            project_id: project_id.to_string(),
            project_name: p.name,
            version_id: version_id.to_string(),
            version_name: v.display_name,
            author: p
                .authors
                .iter()
                .map(|a| a.name.to_owned())
                .collect::<Vec<_>>()
                .join(","),
            summary: p.summary,
            thumbnail: p.logo.thumbnail_url,
            reference: p.links.website_url.unwrap(),
            kind,
            filename: v.file_name,
            download: v.download_url,
            hash,
            dependencies,
            requirements: if !requirements.is_empty() {
                Some(requirements)
            } else {
                None
            },
        })
    }
}
