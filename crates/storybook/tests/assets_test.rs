// source: packages/storybook — parity: manifest files exist under assets/, total count matches 28
#![allow(clippy::all)]
use storybook::{ASSET_COUNT, KEY_FILES, MOCK_FILES};

fn count_assets() -> usize {
    let mut walk = vec![format!("{}/assets", env!("CARGO_MANIFEST_DIR"))];
    let mut n = 0usize;
    while let Some(dir) = walk.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                walk.push(path.display().to_string());
            } else {
                n += 1;
            }
        }
    }
    n
}

#[test]
fn key_files_present() {
    for rel in KEY_FILES.iter().chain(MOCK_FILES.iter()) {
        let path = format!("{}/assets/{rel}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("missing {rel}: {e}"));
        assert!(!bytes.is_empty(), "{rel} empty");
    }
}

#[test]
fn asset_count_matches_source() {
    assert_eq!(count_assets(), ASSET_COUNT);
}
