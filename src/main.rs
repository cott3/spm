mod cli;
mod fetcher;
mod profile;
mod recipe;
mod repo;
mod store;

use clap::Parser;
use cli::{Cli, Commands};
use profile::ProfileManager;
use recipe::Recipe;
use repo::Repository;
use store::PackageStore;
use std::fs;
use std::path::Path;

// Default repository endpoint (can be overridden by SPM_REPO_URL)
const DEFAULT_REPO_URL: &str = "https://raw.githubusercontent.com/lavaos/spm-recipes/main";

fn main() {
    let args = Cli::parse();

    let store_dir = std::env::var("SPM_STORE_DIR").unwrap_or_else(|_| "/store".to_string());
    let sys_dir = std::env::var("SPM_SYS_DIR").unwrap_or_else(|_| "/sys/current".to_string());
    let repo_url = std::env::var("SPM_REPO_URL").unwrap_or_else(|_| DEFAULT_REPO_URL.to_string());

    let store = PackageStore::new(&store_dir);
    let profile = ProfileManager::new(&sys_dir);
    let repository = Repository::new(&repo_url);

    match args.command {
        Commands::Install { name, version: _, disabled } => {
            println!("[spm] Resolving package '{}'...", name);

            // 1. First try loading local recipe if it exists; otherwise fetch from remote repo
            let recipe_path = format!("recipes/{}.toml", name);
            let recipe = if Path::new(&recipe_path).exists() {
                println!("  ℹ Using local recipe: {}", recipe_path);
                let content = fs::read_to_string(&recipe_path).expect("Failed to read local recipe");
                Recipe::parse(&content).expect("Failed to parse local TOML recipe")
            } else {
                match repository.fetch_recipe(&name) {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("[spm] Error: {}", e);
                        return;
                    }
                }
            };

            println!("[spm] Building {} (v{})...", recipe.package.name, recipe.package.version);

            // 2. Derive hash-addressed store directory
            let pkg_dir = store.get_pkg_path(&recipe.package.name, &recipe.package.version);

            // 3. Download & unpack binary cleanly
            if let Err(e) = fetcher::fetch_and_unpack(&recipe.source.url, &pkg_dir, &recipe.source.binary_name) {
                eprintln!("[spm] Failed to install package: {}", e);
                return;
            }

            // 4. Link binary to active profile
            if !disabled {
                profile.init_profile().unwrap();
                let store_bin = pkg_dir.join("bin").join(&recipe.source.binary_name);
                profile.link_binary(&recipe.source.binary_name, &store_bin).unwrap();
                println!("[spm] Successfully installed and linked {}!", recipe.package.name);
            }
        }

        Commands::Remove { name, purge } => {
            println!("[spm] Unlinking {} from active profile...", name);
            let unlinked = profile.unlink_binary(&name).unwrap_or(false);

            if !unlinked {
                println!("[spm] Binary '{}' was not active in profile.", name);
            }

            if purge {
                println!("[spm] Purging matching store packages for '{}'...", name);
                if let Ok(packages) = store.list_packages() {
                    for pkg in packages {
                        if pkg.starts_with(&name) {
                            let path = Path::new(&store_dir).join(&pkg);
                            if let Err(e) = fs::remove_dir_all(&path) {
                                eprintln!("[spm] Failed to purge {}: {}", pkg, e);
                            } else {
                                println!("  ✓ Removed store directory: {}", path.display());
                            }
                        }
                    }
                }
            }
        }

        Commands::List => {
            println!("=== Active System Profile ===");
            if let Ok(links) = profile.get_active_links() {
                if links.is_empty() {
                    println!("  (No active binaries linked)");
                } else {
                    for link in links {
                        println!("  • {}", link.display());
                    }
                }
            }

            println!("\n=== Store Inventory ===");
            if let Ok(pkgs) = store.list_packages() {
                if pkgs.is_empty() {
                    println!("  (Store is empty)");
                } else {
                    for pkg in pkgs {
                        println!("  📦 {}", pkg);
                    }
                }
            }
        }

        Commands::Gc => {
            println!("[spm] Running Garbage Collector...");
            let active_links = profile.get_active_links().unwrap_or_default();
            let store_pkgs = store.list_packages().unwrap_or_default();

            let mut deleted_count = 0;

            for pkg in store_pkgs {
                let pkg_path = Path::new(&store_dir).join(&pkg);
                let is_referenced = active_links.iter().any(|target| target.starts_with(&pkg_path));

                if !is_referenced {
                    println!("  🗑  Pruning orphaned store package: {}", pkg);
                    if let Err(e) = fs::remove_dir_all(&pkg_path) {
                        eprintln!("    Failed to remove {}: {}", pkg, e);
                    } else {
                        deleted_count += 1;
                    }
                }
            }

            println!("[spm] GC Complete. Swept {} orphaned package(s).", deleted_count);
        }

        Commands::Info => { 
            println!("Store Path:   {}", store_dir);
            println!("Active Profile: {}", sys_dir);
            println!("Repository:   {}", repo_url);
        }
    }
}