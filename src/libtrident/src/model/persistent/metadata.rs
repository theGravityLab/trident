use std::collections::HashMap;

pub struct Metadata {
    name: String,
    version: String,
    layers: Vec<Layer>,
    overrides: HashMap<String, String>
}

pub struct Layer {
    active: bool,
    summary: String,
    loaders: Vec<String>,
    packages: Vec<String>,
}