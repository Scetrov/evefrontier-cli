use anyhow::{anyhow, Context, Result};
use dirs::cache_dir;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{fs, io::{Read, Write}, path::PathBuf};
use zip::ZipArchive;

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

/// Ensure we have a local c3e6.db from the latest evefrontier_datasets release.
/// Returns the path to the extracted c3e6.db file.
pub fn ensure_c3e6_dataset() -> Result<PathBuf> {
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

    let cache_root = cache_dir()
        .ok_or_else(|| anyhow!("Could not determine user cache directory"))?
        .join("evefrontier_datasets");

    fs::create_dir_all(&cache_root)
        .with_context(|| format!("Failed to create cache directory {}", cache_root.display()))?;

    // If asset is already cached, we may still need to extract c3e6.db from it.
    let cached_asset_path = cache_root.join(&asset.name);
    if !cached_asset_path.exists() {
        download_asset(&client, &asset.browser_download_url, &cached_asset_path)?;
    } else {
        eprintln!("Using cached asset: {}", cached_asset_path.display());
    }

    if cached_asset_path.extension().and_then(|e| e.to_str()) == Some("db") {
        // Assume this is directly the c3e6 database or a superset.
        return Ok(cached_asset_path);
    }

    // Otherwise assume it's a .zip and extract c3e6.db (or the first .db) into cache_root.
    extract_c3e6_from_zip(&cached_asset_path, &cache_root)
}

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

fn extract_c3e6_from_zip(zip_path: &PathBuf, out_dir: &PathBuf) -> Result<PathBuf> {
    let file = fs::File::open(zip_path)
        .with_context(|| format!("failed to open zip file {}", zip_path.display()))?;
    let mut archive = ZipArchive::new(file)
        .with_context(|| format!("failed to read zip archive {}", zip_path.display()))?;

    // Prefer a file that looks like c3e6*.db, otherwise take the first .db.
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
