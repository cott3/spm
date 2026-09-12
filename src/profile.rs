use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::PathBuf;

pub struct ProfileManager {
    pub current_profile: PathBuf,
}

impl ProfileManager {
    pub fn new(sys_current_path: &str) -> Self {
        Self {
            current_profile: PathBuf::from(sys_current_path),
        }
    }

    pub fn init_profile(&self) -> std::io::Result<()> {
        let bin_dir = self.current_profile.join("bin");
        fs::create_dir_all(bin_dir)?;
        Ok(())
    }

    pub fn link_binary(&self, binary_name: &str, store_bin_path: &PathBuf) -> std::io::Result<()> {
        let sys_bin = self.current_profile.join("bin").join(binary_name);

        if sys_bin.exists() || sys_bin.is_symlink() {
            fs::remove_file(&sys_bin)?;
        }

        unix_fs::symlink(store_bin_path, &sys_bin)?;
        println!("  ✓ Linked {} -> {}", sys_bin.display(), store_bin_path.display());
        Ok(())
    }

    /// Unlink a binary from /sys/current/bin/
    pub fn unlink_binary(&self, binary_name: &str) -> std::io::Result<bool> {
        let sys_bin = self.current_profile.join("bin").join(binary_name);
        if sys_bin.exists() || sys_bin.is_symlink() {
            fs::remove_file(&sys_bin)?;
            println!("  ✓ Removed active link: {}", sys_bin.display());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get all active symlink targets in /sys/current/bin/
    pub fn get_active_links(&self) -> std::io::Result<Vec<PathBuf>> {
        let mut targets = Vec::new();
        let bin_dir = self.current_profile.join("bin");
        if bin_dir.exists() {
            for entry in fs::read_dir(bin_dir)? {
                let entry = entry?;
                if entry.path().is_symlink() {
                    if let Ok(target) = fs::read_link(entry.path()) {
                        targets.push(target);
                    }
                }
            }
        }
        Ok(targets)
    }
}