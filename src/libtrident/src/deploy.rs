// deploy 分为多个阶段

// [...] Check polylock availability
// [...] Resolve attachments
//     Resolved Modern UI
//    Resolving pkg:curseforge/1919@810
// [...] Install components
//     [x] net.minecraft
//     [ ] builtin.trident.storage
// [...] Download data and files
//   Downloaded http://example.com/more_file_to_download.txt
//  Downloading http://example.com/file.txt
// [...] Restore instance

use crate::deploy::polylock::PolylockData;
use crate::download::DownloadEngine;
use crate::instance::Instance;
use crate::resolve::ResolveEngine;

pub mod polylock;

pub struct DeployMachine {
    instance: Instance,
}

impl DeployMachine {
    pub fn new(instance: Instance) -> Self {
        Self { instance }
    }

    pub fn check_polylock(&self) -> Option<PolylockData> {
        // 匹配 hash 来选择读取或丢弃
        None
    }
    
    pub fn resolve_attachments(&self) -> ResolveEngine {
        todo!()
    }
    
    pub fn download_files(&self, polylock: PolylockData) -> DownloadEngine{
        todo!()
    }
}
