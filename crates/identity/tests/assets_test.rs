// source: packages/identity — parity: every manifest asset is present and non-empty under assets/
#![allow(clippy::all)]
use identity::ASSETS;

#[test]
fn all_assets_present_and_nonempty() {
    for name in ASSETS {
        let bytes = std::fs::read(format!("{}/assets/{name}", env!("CARGO_MANIFEST_DIR")))
            .unwrap_or_else(|e| panic!("missing asset {name}: {e}"));
        assert!(!bytes.is_empty(), "asset {name} is empty");
    }
}

#[test]
fn asset_count_matches() {
    let dir = std::fs::read_dir(format!("{}/assets", env!("CARGO_MANIFEST_DIR"))).unwrap();
    assert_eq!(dir.count(), ASSETS.len());
}

#[test]
fn svg_assets_mark_verbatim() {
    let mark =
        std::fs::read_to_string(format!("{}/assets/mark.svg", env!("CARGO_MANIFEST_DIR"))).unwrap();
    assert_eq!(&mark[..1], "<", "mark.svg must start with '<' (SVG)");
}
