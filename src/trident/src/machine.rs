use std::path::PathBuf;

pub struct Machine {
    root: PathBuf,
}

impl Machine {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub fn scan(&self) -> Vec<String> {
        if let Ok(entries) = std::fs::read_dir(&self.root.join("instances")) {
            entries
                .filter(|e| {
                    if let Ok(entry) = e {
                        entry.path().extension().map(|f| f.to_str() == Some("ron")) == Some(true)
                    } else {
                        false
                    }
                })
                .map(|e| e.unwrap().file_name().to_str().unwrap().to_owned())
                .collect()
        } else {
            vec![]
        }
    }
}
