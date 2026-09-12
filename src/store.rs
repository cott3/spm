use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

pub struct PackageStore {
    pub store_path: PathBuf,
}

impl PackageStore {
    pub fn new(store_path: &str) -> Self {
        Self {
            store_path: PathBuf::from(store_path),
        }
    }

    pub fn compute_hash(name: &str, version: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}-{}", name, version));
        let result = hasher.finalize();
        hex::encode(result)[..8].to_string()
    }

    pub fn get_pkg_path(&self, name: &str, version: &str) -> PathBuf {
        let hash = Self::compute_hash(name, version);
        self.store_path.join(format!("{}-{}-{}", name, version, hash))
    }

    /// List all packages currently residing in /store/
    pub fn list_packages(&self) -> std::io::Result<Vec<String>> {
        let mut packages = Vec::new();
        if self.store_path.exists() {
            for entry in fs::read_dir(&self.store_path)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    if let Some(name) = entry.file_name().to_str() {
                        packages.push(name.to_string());
                    }
                }
            }
        }
        Ok(packages)
    }
}