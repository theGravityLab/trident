use crate::metrology::{MemorySize, WindowSize};

pub struct Metadata {
    name: String,
    version: String,
    overrides: Overrides,
    layers: Vec<Layer>,
}

pub struct Overrides {
    // Window
    window_title: Option<String>,
    window_width: Option<WindowSize>,
    window_height: Option<WindowSize>,
    // Jvm
    java_home: Option<String>,
    java_additional_arguments: Option<String>,
    java_allocated_memory: Option<MemorySize>,
}

pub struct Layer {
    active: bool,
    summary: String,
    packages: Vec<Package>,
}

pub struct Loader {
    identity: String,
    qualifier: String,
}

pub struct Package {
    label: String,
    namespace: Option<String>,
    identity: String,
    version: Option<String>,
}