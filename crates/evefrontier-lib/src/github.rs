use anyhow::{anyhow, Context, Result};
use dirs::cache_dir;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{fs, io::{Read, Write}, path::PathBuf};
use zip::ZipArchive;
use std::path::Path;

#[derive(Deserialize)]
struct Release {
    #[serde(rename = "tag_name")]
    _tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    browser_download_url: String,
    name: String,
}

// (moved to the Option-taking implementation further down)

fn download_asset(client: &Client, url: &str, dest: &PathBuf) -> Result<()> {
    eprintln!("Downloading {}", url);
    let mut resp = client
        .get(url)
        .header("User-Agent", "evefrontier-pathfinder (github.com/Scetrov)")
        .send()
        .context("failed to start download")?
        .error_for_status()
        .context("download request returned error")?;

    let total_size = resp
        .content_length()
        .unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("=>-"),
    );

    let mut file = fs::File::create(dest)
        .with_context(|| format!("failed to create file {}", dest.display()))?;
    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 8 * 1024];
    loop {
        let n = resp.read(&mut buffer).context("download read error")?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }
    pb.finish_with_message("download complete");

    Ok(())
}

/// Ensure we have a dataset available at `target_db`.
///
/// If `target_db` is `Some(path)`, this function will ensure the file exists at that
/// path (creating parent directories as needed). If it's `None`, it falls back to
/// the user's cache directory (same behavior as the previous implementation) and
/// returns the path to the DB file.
pub fn ensure_c3e6_dataset(target_db: Option<&Path>) -> Result<PathBuf> {
    let client = Client::new();
    let api = "https://api.github.com/repos/Scetrov/evefrontier_datasets/releases/latest";

    let release: Release = client
        .get(api)
        .header("User-Agent", "evefrontier-pathfinder (github.com/Scetrov)")
        .send()
        .context("failed to query GitHub releases API")?
        .error_for_status()
        .context("GitHub releases API returned error")?
        .json()
        .context("failed to parse GitHub releases JSON")?;

    let asset = release
        .assets
        .iter()
        .find(|a| a.name.ends_with(".zip") || a.name.ends_with(".db"))
        .ok_or_else(|| anyhow!("No suitable asset (.zip or .db) found in latest release"))?;

    // Where will we place the final DB file?
    let target_path = if let Some(p) = target_db {
        p.to_path_buf()
    } else {
        cache_dir()
            .ok_or_else(|| anyhow!("Could not determine user cache directory"))?
            .join("evefrontier_datasets")
            .join(&asset.name)
    };

    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create data directory {}", parent.display()))?;
    }

    if target_path.exists() {
        eprintln!("Using existing DB: {}", target_path.display());
        return Ok(target_path);
    }

    // Download to a temporary file then atomically rename to target_path
    let cache_root = cache_dir()
        .ok_or_else(|| anyhow!("Could not determine user cache directory"))?
        .join("evefrontier_datasets");
    fs::create_dir_all(&cache_root).ok();
    let cached_asset_path = cache_root.join(&asset.name);
    if !cached_asset_path.exists() {
        download_asset(&client, &asset.browser_download_url, &cached_asset_path)?;
    } else {
        eprintln!("Using cached asset: {}", cached_asset_path.display());
    }

    // If the cached asset is a DB, copy/rename it to target_path.
    if cached_asset_path.extension().and_then(|e| e.to_str()) == Some("db") {
        // copy to temp and rename
        let tmp = target_path.with_extension("db.tmp");
        fs::copy(&cached_asset_path, &tmp)
            .with_context(|| format!("failed to copy DB to {}", tmp.display()))?;
        fs::rename(&tmp, &target_path)
            .with_context(|| format!("failed to move DB to {}", target_path.display()))?;
        return Ok(target_path);
    }

    // Otherwise assume zip and extract
    let extracted = extract_c3e6_from_zip(&cached_asset_path, &cache_root)?;
    // Move extracted to target_path
    let tmp = target_path.with_extension("db.tmp");
    fs::copy(&extracted, &tmp)
        .with_context(|| format!("failed to copy extracted DB to {}", tmp.display()))?;
    fs::rename(&tmp, &target_path)
        .with_context(|| format!("failed to move DB to {}", target_path.display()))?;
    Ok(target_path)
}

// Compatibility shim: previous callers without args
pub fn ensure_c3e6_dataset_default() -> Result<PathBuf> {
    ensure_c3e6_dataset(None)
}

fn extract_c3e6_from_zip(zip_path: &PathBuf, out_dir: &PathBuf) -> Result<PathBuf> {
    let file = fs::File::open(zip_path)
        .with_context(|| format!("failed to open zip file {}", zip_path.display()))?;
    let mut archive = ZipArchive::new(file)
        .with_context(|| format!("failed to read zip archive {}", zip_path.display()))?;

    let mut candidate_index: Option<usize> = None;
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let name = file.name().to_string();
        if name.ends_with(".db") {
            if name.to_lowercase().contains("c3e6") {
                candidate_index = Some(i);
                break;
            }
            if candidate_index.is_none() {
                candidate_index = Some(i);
            }
        }
    }

    let idx = candidate_index.ok_or_else(|| anyhow!("No .db file found inside zip"))?;
    let mut db_file = archive.by_index(idx)?;
    let db_name = db_file.name().rsplit('/').next().unwrap_or("c3e6.db");
    let out_path = out_dir.join(db_name);

    if out_path.exists() {
        eprintln!("Using cached extracted DB: {}", out_path.display());
        return Ok(out_path);
    }

    eprintln!("Extracting {} to {}", db_file.name(), out_path.display());
    let mut out = fs::File::create(&out_path)?;
    std::io::copy(&mut db_file, &mut out)?;
    Ok(out_path)
}
