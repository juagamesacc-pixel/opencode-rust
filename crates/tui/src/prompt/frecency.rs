// source: packages/tui/src/prompt/frecency.tsx (80 lines, v1.18.30)
// 1:1 port — JSONL frecency store verbatim (parse cap 1000, decay scoring,
// append-then-rewrite persistence, cwd-relative resolution).

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Entry cap verbatim.
pub const MAX_FRECENCY_ENTRIES: usize = 1000;
/// Filename verbatim.
pub const FRECENCY_FILENAME: &str = "frecency.jsonl";

/// Mirrors `FrecencyEntry`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrecencyEntry {
    pub path: String,
    pub frequency: f64,
    pub last_open: i64,
}

/// Mirrors `parseFrecency` — last write per path wins, newest first,
// capped at 1000.
pub fn parse_frecency(text: &str) -> Vec<FrecencyEntry> {
    let mut latest: HashMap<String, FrecencyEntry> = HashMap::new();
    for line in text.lines().filter(|line| !line.is_empty()) {
        if let Ok(entry) = serde_json::from_str::<FrecencyEntry>(line) {
            latest.insert(entry.path.clone(), entry);
        }
    }
    let mut entries: Vec<FrecencyEntry> = latest.into_values().collect();
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.last_open));
    entries.truncate(MAX_FRECENCY_ENTRIES);
    entries
}

/// Mirrors `calculateFrecency` (frequency over days-since-open decay).
pub fn calculate_frecency(frequency: f64, last_open: i64, now_ms: i64) -> f64 {
    frequency / (1.0 + (now_ms - last_open) as f64 / 86_400_000.0)
}

/// Frecency store (mirrors the context value).
pub struct FrecencyStore {
    file: PathBuf,
    cwd: String,
    data: HashMap<String, (f64, i64)>,
}

impl FrecencyStore {
    pub fn new(state_dir: &str, cwd: &str) -> Self {
        Self {
            file: PathBuf::from(state_dir).join(FRECENCY_FILENAME),
            cwd: cwd.to_string(),
            data: HashMap::new(),
        }
    }

    /// Mount load + compaction rewrite (mirrors `onMount`).
    pub async fn load(&mut self) {
        let text = tokio::fs::read_to_string(&self.file)
            .await
            .unwrap_or_default();
        let lines = parse_frecency(&text);
        self.data = lines
            .into_iter()
            .map(|entry| (entry.path, (entry.frequency, entry.last_open)))
            .collect();
        if !self.data.is_empty() {
            let mut entries: Vec<FrecencyEntry> = self
                .data
                .iter()
                .map(|(path, (frequency, last_open))| FrecencyEntry {
                    path: path.clone(),
                    frequency: *frequency,
                    last_open: *last_open,
                })
                .collect();
            entries.sort_by_key(|entry| std::cmp::Reverse(entry.last_open));
            let payload = entries
                .iter()
                .map(|entry| serde_json::to_string(entry).unwrap_or_default())
                .collect::<Vec<_>>()
                .join("\n")
                + "\n";
            let _ = crate::util::persistence::write_text(&self.file, &payload).await;
        }
    }

    fn resolve(&self, file_path: &str) -> String {
        let path = std::path::Path::new(file_path);
        if path.is_absolute() {
            return file_path.to_string();
        }
        format!("{}/{}", self.cwd.trim_end_matches('/'), file_path)
    }

    /// Mirrors `getFrecency`.
    pub fn get_frecency(&self, file_path: &str, now_ms: i64) -> f64 {
        match self.data.get(&self.resolve(file_path)) {
            Some((frequency, last_open)) => calculate_frecency(*frequency, *last_open, now_ms),
            None => 0.0,
        }
    }

    /// Mirrors `updateFrecency` (append + cap rewrite at 1000).
    pub async fn update_frecency(&mut self, file_path: &str, now_ms: i64) {
        let absolute = self.resolve(file_path);
        let frequency = self
            .data
            .get(&absolute)
            .map(|(frequency, _)| *frequency)
            .unwrap_or(0.0)
            + 1.0;
        self.data.insert(absolute.clone(), (frequency, now_ms));
        let line =
            serde_json::json!({ "path": absolute, "frequency": frequency, "lastOpen": now_ms });
        let _ = crate::util::persistence::append_text(
            &self.file,
            &(serde_json::to_string(&line).unwrap_or_default() + "\n"),
        )
        .await;
        if self.data.len() <= MAX_FRECENCY_ENTRIES {
            return;
        }
        let mut sorted: Vec<(String, (f64, i64))> =
            self.data.iter().map(|(k, v)| (k.clone(), *v)).collect();
        sorted.sort_by_key(|(_, (_, last_open))| std::cmp::Reverse(*last_open));
        sorted.truncate(MAX_FRECENCY_ENTRIES);
        self.data = sorted.into_iter().collect();
        let payload = self
            .data
            .iter()
            .map(|(path, (frequency, last_open))| {
                serde_json::to_string(&serde_json::json!({ "path": path, "frequency": frequency, "lastOpen": last_open })).unwrap_or_default()
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        let _ = crate::util::persistence::write_text(&self.file, &payload).await;
    }

    pub fn snapshot(&self) -> HashMap<String, (f64, i64)> {
        self.data.clone()
    }
}
