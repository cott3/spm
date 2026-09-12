use crate::recipe::Recipe;
use std::io;

pub struct Repository {
    pub base_url: String,
}

impl Repository {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Fetches a package recipe directly from the remote repository
    pub fn fetch_recipe(&self, name: &str) -> io::Result<Recipe> {
        let recipe_url = format!("{}/recipes/{}.toml", self.base_url, name);
        println!("  ↓ Fetching recipe for '{}' from remote repository...", name);

        let response = reqwest::blocking::get(&recipe_url)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if !response.status().is_success() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Package '{}' not found in remote repository (HTTP {}).", name, response.status()),
            ));
        }

        let recipe_str = response.text()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Recipe::parse(&recipe_str)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to parse recipe TOML: {}", e)))
    }
}