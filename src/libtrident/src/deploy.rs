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

use std::cell::{Ref, RefCell};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{PathBuf};
use std::rc::Rc;
use crate::deploy::polylock::PolylockData;
use crate::instance::Instance;
use crate::repo::Repository;
use crate::resolve::{ResolveEngine, ResolveError, ResolveHandle};
use crate::packages::{Package};

pub mod polylock;

struct DeployContext {
    instance: Instance,
    polylock: Option<PolylockData>,
    polylock_data_path: PathBuf,
    polylock_hash_path: PathBuf,
    resolved: Option<Vec<Package>>,
    installed: Option<()>,
    checked: bool,
    downloaded: bool,
    restored: bool,
}

impl DeployContext {
    fn new(instance: Instance) -> Self {
        Self {
            polylock_hash_path: instance.home().join("polylock.hash"),
            polylock_data_path: instance.home().join("polylock.json"),
            instance,
            installed: None,
            polylock: None,
            resolved: None,
            checked: false,
            downloaded: false,
            restored: false,
        }
    }
}

pub struct DeployEngine {
    context: Rc<RefCell<DeployContext>>,
    repo_factory: fn(&str) -> Option<Rc<dyn Repository>>,
    forced: bool,
    max_resolve_depth: usize,
}

impl DeployEngine {
    pub fn new(instance: Instance, force: bool, max_resolve_depth: usize, repo_factory: fn(&str) -> Option<Rc<dyn Repository>>) -> Self {
        Self {
            context: Rc::new(RefCell::new(DeployContext::new(instance))),
            repo_factory,
            forced: force,
            max_resolve_depth,
        }
    }
}

impl Iterator for DeployEngine {
    type Item = DeployStage;

    // 具体任务需要在 stage 中实施，stage 内报错不影响 engine，对 engine 继续迭代将从失败的 stage 继续。
    // 如果要中断需要在出错后停止迭代。
    fn next(&mut self) -> Option<Self::Item> {
        if self.forced {
            self.forced = false;
            self.context.borrow_mut().checked = true;
            Some(DeployStage::Resolve(ResolveStage::new(Rc::clone(&self.context), self.max_resolve_depth, self.repo_factory)))
        } else if self.context.borrow().polylock.is_some() {
            if self.context.borrow().downloaded {
                if self.context.borrow().restored {
                    // finished
                    None
                } else {
                    Some(DeployStage::Restore)
                }
            } else {
                // TODO: go download
                Some(DeployStage::Download)
            }
        } else if self.context.borrow().checked {
            if self.context.borrow().resolved.is_some() {
                if self.context.borrow().installed.is_some() {
                    // TODO: build PolylockData from resolved and installed
                    todo!()
                } else {
                    Some(DeployStage::Install)
                }
            } else {
                Some(DeployStage::Resolve(ResolveStage::new(Rc::clone(&self.context), self.max_resolve_depth, self.repo_factory)))
            }
        } else {
            Some(DeployStage::Check(CheckStage::new(Rc::clone(&self.context))))
        }
    }
}

pub enum DeployStage {
    Check(CheckStage),
    Resolve(ResolveStage),
    Install,
    Download,
    Restore,
}

pub struct CheckStage {
    context: Rc<RefCell<DeployContext>>,
}

impl CheckStage {
    fn new(context: Rc<RefCell<DeployContext>>) -> Self {
        Self {
            context
        }
    }

    pub fn perform(&mut self) {
        if let Some(hash) = self.read_hash() {
            let mut hasher = DefaultHasher::new();
            self.context.borrow().instance.profile().metadata.hash(&mut hasher);
            if hash == hasher.finish().to_string() {
                if let Some(data) = self.read_data() {
                    self.context.borrow_mut().polylock = Some(data)
                }
            }
        }
        self.context.borrow_mut().checked = true;
    }

    fn read_hash(&self) -> Option<String> {
        if self.context.borrow().polylock_hash_path.exists() {
            if let Ok(content) = fs::read_to_string(&self.context.borrow().polylock_hash_path) {
                Some(content)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn read_data(&self) -> Option<PolylockData> {
        if self.context.borrow().polylock_data_path.exists() {
            if let Ok(content) = fs::read_to_string(&self.context.borrow().polylock_data_path) {
                ron::from_str::<PolylockData>(&content).ok()
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub struct ResolveStage {
    context: Rc<RefCell<DeployContext>>,
    sub: Rc<RefCell<ResolveStageContext>>,
    repo_locate: fn(&str) -> Option<Rc<dyn Repository>>,
    engine: Option<ResolveEngine>,
    depth: usize,
    max_depth: usize,
}

impl ResolveStage {
    fn new(context: Rc<RefCell<DeployContext>>, max_depth: usize, repo_factory: fn(&str) -> Option<Rc<dyn Repository>>) -> Self {
        let tasks = context.borrow().instance.profile().metadata.attachments.iter().filter(|l| l.enabled).flat_map(|l| &l.content).cloned().collect();
        Self {
            context,
            sub: Rc::new(RefCell::new(ResolveStageContext {
                finished: Vec::new(),
                processed: Vec::new(),
                appended: Some(tasks),
            })),
            repo_locate: repo_factory,
            engine: None,
            depth: 0,
            max_depth,
        }
    }
}

impl Iterator for ResolveStage {
    type Item = ResolveStageHandle;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(engine) = &mut self.engine {
            if let Some(next) = engine.next() {
                Some(ResolveStageHandle {
                    handle: next,
                    context: Rc::clone(&self.sub),
                })
            } else {
                self.engine = None;
                self.next()
            }
        } else if self.max_depth > self.depth {
            let mut has_next = false;
            if let Some(appended) = self.sub.borrow_mut().appended.take() {
                if !appended.is_empty() {
                    let mut context = self.sub.borrow_mut();
                    let to_add = appended.iter().filter(|s| !context.processed.contains(s)).cloned().collect::<Vec<String>>();
                    for add in &to_add {
                        context.processed.push(add.clone());
                    }
                    let engine = ResolveEngine::new(to_add, self.repo_locate);
                    self.engine = Some(engine);
                    self.depth += 1;
                    has_next = true;
                }
            };
            if has_next {
                self.next()
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Drop for ResolveStage {
    fn drop(&mut self) {
        self.context.borrow_mut().resolved = Some(self.sub.borrow().finished.clone())
    }
}

pub struct ResolveStageContext {
    finished: Vec<Package>,
    processed: Vec<String>,
    appended: Option<Vec<String>>,
}

pub struct ResolveStageHandle {
    handle: ResolveHandle,
    context: Rc<RefCell<ResolveStageContext>>,
}

impl ResolveStageHandle {
    pub fn perform(&mut self) -> Result<Package, ResolveError> {
        let v = self.handle.perform()?;
        let mut context = self.context.borrow_mut();
        context.finished.push(v.clone());
        if let Some(dependencies) = &v.dependencies {
            for d in dependencies {
                if d.required {
                    if let Some(appended) = &mut context.appended {
                        appended.push(d.purl.clone());
                    } else {
                        context.appended = Some(vec![d.purl.clone()]);
                    }
                }
            }
        }
        Ok(v)
    }

    pub fn task(&self) -> &str {
        self.handle.task()
    }
}