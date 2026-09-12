use serde::Deserialize;

/// Derive `Deserialize` allows Serde to automatically map TOML keys
/// directly into this Rust struct!
#[derive(Debug, Deserialize)]
pub struct Recipe {
    pub package: PackageMeta,
    pub source: SourceMeta,
}

#[derive(Debug, Deserialize)]
pub struct PackageMeta {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct SourceMeta {
    pub url: String,
    pub binary_name: String,
}

impl Recipe {
    /// Parses a raw TOML string into a Recipe struct.
    /// Returns `Result<Self, toml::de::Error>` to handle bad syntax gracefully.
    pub fn parse(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }
}