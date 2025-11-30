use std::path::PathBuf;

use anyhow::Result;
use tokio::fs;
use which::which;

const TAILWIND: &str = "tailwindcss";
const TAILWIND_VSN: &str = "v4.1.17";
const TAILWIND_FILE: &str = "tailwindcss";
const TAILWIND_DWNLD_BASE: &str = "https://github.com/tailwindlabs/tailwindcss/releases/download/";

#[cfg(all(target_os = "linux"))]
const TAILWIND_PLAT: &str = "linux";

#[cfg(all(target_os = "windows"))]
const TAILWIND_PLAT: &str = "windows";

#[cfg(all(target_os = "macos"))]
const TAILWIND_PLAT: &str = "macos";

#[cfg(all(target_arch = "x86_64", not(target_os = "windows")))]
const TAILWIND_ARCH: &str = "x64";

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
const TAILWIND_ARCH: &str = "x64.exe";

#[cfg(target_arch = "aarch64")]
const TAILWIND_ARCH: &str = "arm64";

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
compile_error!("Unsupported OS for Tailwind CSS CLI");
#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!("Unsupported Arch for Tailwind CSS CLI");

fn download_path() -> String {
    format!(
        "{}/{}/{}-{}-{}",
        TAILWIND_DWNLD_BASE, TAILWIND_VSN, TAILWIND_FILE, TAILWIND_PLAT, TAILWIND_ARCH
    )
}

fn install_path() -> PathBuf {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let mut path = PathBuf::from(out_dir);
    path.push(TAILWIND);
    path
}

#[cfg(unix)]
async fn patch_file(path: &PathBuf) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path).await?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).await?;

    Ok(())
}

async fn download_tw() -> Result<PathBuf> {
    let url = download_path();
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    let install_path = install_path();
    fs::write(&install_path, &bytes).await?;

    #[cfg(unix)]
    patch_file(&install_path).await?;

    Ok(install_path)
}

async fn locate_tw() -> Result<PathBuf> {
    let install = install_path();
    if install.exists() {
        return Ok(install);
    }

    match which(TAILWIND) {
        Ok(path) => Ok(path),
        Err(_) => download_tw().await,
    }
}

#[tokio::main]
async fn main() {
    let prog_path = locate_tw()
        .await
        .expect("Failed to locate or download Tailwind CSS CLI");

    println!("cargo:warning=Tailwind CSS CLI found at {:?}", prog_path);
}
