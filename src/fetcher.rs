use std::fs;
use std::io::{self, Cursor};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn fetch_and_unpack(url: &str, target_dir: &Path, binary_name: &str) -> io::Result<()> {
    println!("  ↓ Downloading archive from: {}", url);

    let response = reqwest::blocking::get(url)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let bytes = response.bytes()
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    println!("  📦 Inspecting and extracting binary to {}", target_dir.display());

    let tar_gz = GzDecoder::new(Cursor::new(bytes));
    let mut archive = Archive::new(tar_gz);

    let bin_dir = target_dir.join("bin");
    fs::create_dir_all(&bin_dir)?;

    let mut found = false;

    for entry_result in archive.entries()? {
        let mut entry = entry_result?;
        let path = entry.path()?;

        // Dynamically match any file inside the tarball that matches binary_name
        if let Some(file_name) = path.file_name() {
            if file_name.to_str() == Some(binary_name) {
                let dest_path = bin_dir.join(binary_name);
                
                // Extract directly to store bin path
                entry.unpack(&dest_path)?;

                // Ensure execution permissions on Unix
                let mut perms = fs::metadata(&dest_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&dest_path, perms)?;

                println!("  ✓ Extracted and set executable: {}", dest_path.display());
                found = true;
                break;
            }
        }
    }

    if found {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Binary '{}' was not found anywhere inside the downloaded archive!", binary_name),
        ))
    }
}