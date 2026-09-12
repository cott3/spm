# spm - Stored Package Manager

`spm` is a lightweight, standalone, hash-adressed store-based package manager built for Linux systems.
Inspired by functional package management concepts, `spm` isolates installed software inside unique, immutable `/store/` directories and atomically manages active binary versions using profile symlinks.

## features

- **Store Isolation:** Every package is placed into its own hashed directory (`store/pkg-ber-hash`).
- **Atomic Profiles:** Binaries are linked into `/sys/current/bin/`via symlinks, making installs, removals and upgrades instant and clean.

---

## installation

### prerequisites

Ensure you have C build tools installed:
- **Ubuntu / Debian:** `sudo apt install build-essential`
- **Arch Linux:** `sudo pacman -S base-devel`
- **Fedora:** `sudo dnf install gcc`

### building from source

```bash
# Clone the repository
git clone https://github.com/cott3/spm.git
cd spm

# Compile and install to ~/.cargo/bin
cargo install --path .
```
---

## quickstart and usage

## 1. Set up environment variables (sandbox testing)
By default, `spm`operates on root paths (`/store`and `/sys/current/`). For testing without root, set up a local sandbox in your shell:

```bash
export SPM_STORE_DIR="$(pwd)/sandbox/store"
export SPM_SYS_DIR="$(pwd)/sandbox/sys/current"
export PATH="$SPM_SYS_DIR/bin:$PATH"

# Set remote recipe index repository
export SPM_REPO_URL="https://github.com/cott3/spm-recipes/main"
```

## 2. Basic commands

```bash
# Display system profile, store paths, and active repository
spm info

# Fetch recipe over HTTPS, unpack binary to store, and symlink to profile
spm install ripgrep 14.1.0

# List active symlinked binaries and store contents
spm list

# Unlink a binary from active profile without deleting its store directory
spm remove rg

# Purge orphaned store directories not linked to any active profile
spm gc
```

---
## Recipe format
Package recipes are defined using simple TOML files hosted in a `spm-recipes` repository under `recipes/(pkg).toml`

```toml
[package]
name = "ripgrep"
version = "14.1.0"
description = "Fast line-oriented search tool"

[source]
url = "[https://github.com/BurntSushi/ripgrep/releases/download/14.1.0/ripgrep-14.1.0-x86_64-unknown-linux-musl.tar.gz](https://github.com/BurntSushi/ripgrep/releases/download/14.1.0/ripgrep-14.1.0-x86_64-unknown-linux-musl.tar.gz)"
binary_name = "rg"
```