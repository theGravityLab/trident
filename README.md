# Trident

Polymerium in Rust!

```toml
name = "hello_kitty"
version = "1.20.1"

[[layers]]
active = false
summary = "Builtin"
loaders = [
    "net.minecraftforge:40.0.1"
]
packages = [
    "curseforge:114@514",
    "modrinth:1919",
    "github:d3ara1n/ModularFormula@v0.6"
]
```

`packages` 对应

```rust
use std::iter::Filter;

pub const LABEL_CURSEFORGE: &str = "curseforge";
pub const LABEL_MODRINTH: &str = "modrinth";
pub const LABEL_GITHUB: &str = "github";

pub trait Repository {
    const LABEL: String;

    async fn search(keyword: &str, page: usize, sizes: usize, filter: Filter);
    async fn resolve(namespace: Option<&str>, identity: &str, version: Option<&str>, filter: Filter);
}
```

解析的时候是 `Vec<Box<dyn Repository>>` 依次用 `LABEL` 去匹配，解析出后续字段调用 `repo.resolve()`