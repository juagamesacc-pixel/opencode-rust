// Parity tests for `src/assets`, `src/styles` and `public`.
use std::fs;
use std::path::Path;
use web::assets;

fn crate_file(path: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
}

#[test]
fn asset_tables_match_source_tree() {
    assert_eq!(assets::ASSETS.len(), 13);
    assert_eq!(assets::PUBLIC.len(), 15);
    assert_eq!(assets::STYLES.len(), 1);
    assert_eq!(assets::TOTAL_FILES, 29);
    assert_eq!(
        assets::TOTAL_FILES,
        assets::ASSETS.len() + assets::PUBLIC.len() + assets::STYLES.len()
    );

    let logo = assets::find("assets/logo-light.svg").expect("light logo");
    assert_eq!(logo.rel, "logo-light.svg");
    assert!(logo.bytes > 0);
    assert!(assets::find("public/robots.txt").is_some());
    assert!(assets::find("styles/custom.css").is_some());
    assert!(assets::find("assets/missing.svg").is_none());
}

#[test]
fn every_listed_asset_is_copied_byte_identical() {
    for asset in assets::ASSETS
        .iter()
        .chain(assets::PUBLIC.iter())
        .chain(assets::STYLES.iter())
    {
        let full = crate_file(asset.path);
        assert!(full.is_file(), "copied {}", asset.path);
        let bytes = fs::read(&full).expect("readable asset");
        assert_eq!(bytes.len(), asset.bytes, "size of {}", asset.path);
    }
}

#[test]
fn vercel_style_static_files_cover_head_and_manifest() {
    // `astro.config.mjs` head links and the webmanifest reference these public files.
    for rel in [
        "public/favicon-v3.svg",
        "public/favicon-v3.ico",
        "public/favicon-96x96-v3.png",
        "public/apple-touch-icon-v3.png",
        "public/site.webmanifest",
    ] {
        assert!(assets::find(rel).is_some(), "public file {}", rel);
        assert!(crate_file(rel).is_file(), "copied {}", rel);
    }
}
