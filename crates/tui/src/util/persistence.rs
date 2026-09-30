// source: packages/tui/src/util/persistence.ts (33 lines, v1.18.30)
// 1:1 port — Bun file APIs become tokio::fs; the atomic-write temp name
// keeps pid + uniqueness verbatim (`pid.uuid.tmp` → pid + nanos tmp).

#![allow(dead_code)]

use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// Mirrors `readText`.
pub async fn read_text(file_path: &Path) -> Result<String, String> {
    tokio::fs::read_to_string(file_path)
        .await
        .map_err(|e| e.to_string())
}

/// Mirrors `readJson`.
pub async fn read_json<T: DeserializeOwned>(file_path: &Path) -> Result<T, String> {
    let text = read_text(file_path).await?;
    serde_json::from_str(&text).map_err(|e| e.to_string())
}

/// Mirrors `writeText` (creates parent dirs).
pub async fn write_text(file_path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }
    tokio::fs::write(file_path, content)
        .await
        .map_err(|e| e.to_string())
}

/// Mirrors `appendText` (creates parent dirs).
pub async fn append_text(file_path: &Path, content: &str) -> Result<(), String> {
    use tokio::io::AsyncWriteExt;
    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }
    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .await
        .map_err(|e| e.to_string())?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| e.to_string())
}

fn temp_path(file_path: &Path) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
    let name = format!("{}.{}.{}.tmp", std::process::id(), nanos, unique);
    let base = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("tmp");
    file_path.with_file_name(format!("{base}.{name}"))
}

/// Mirrors `writeJsonAtomic` — write temp, clean it up on either failure,
// rename over the target.
pub async fn write_json_atomic(file_path: &Path, value: &serde_json::Value) -> Result<(), String> {
    if let Some(parent) = file_path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }
    let temporary = temp_path(file_path);
    let payload = serde_json::to_string(value).map_err(|e| e.to_string())?;
    if let Err(error) = tokio::fs::write(&temporary, payload).await {
        let _ = tokio::fs::remove_file(&temporary).await;
        return Err(error.to_string());
    }
    if let Err(error) = tokio::fs::rename(&temporary, file_path).await {
        let _ = tokio::fs::remove_file(&temporary).await;
        return Err(error.to_string());
    }
    Ok(())
}
