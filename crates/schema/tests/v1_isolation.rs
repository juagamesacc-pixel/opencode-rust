//! Port of `packages/schema/test/v1-isolation.test.ts`.
//!
//! Case names/order mirror the source (`toBe` module identity → equality of
//! the shared definition values through the flat shims, which re-export the
//! isolated `v1/` modules exactly as `export * from` does in source).
//! The direct-import scan mirrors the source `from "./v1/` check with the
//! Rust-precise `crate::v1` marker alongside the verbatim string.

#[test]
fn compatibility_entrypoints_preserve_isolated_v1_schema_identity() {
    assert_eq!(
        schema::legacy_event::CommandExecuted::TYPE,
        schema::v1::legacy_event::CommandExecuted::TYPE
    );
    assert_eq!(
        schema::legacy_event::Definitions,
        schema::v1::legacy_event::Definitions
    );
    assert_eq!(
        schema::permission_v1::Event::Definitions,
        schema::v1::permission::Event::Definitions
    );
    assert_eq!(
        schema::question_v1::Event::Definitions,
        schema::v1::question::Event::Definitions
    );
    assert_eq!(
        schema::session_v1::Event::Definitions,
        schema::v1::session::Event::Definitions
    );
}

#[test]
fn current_source_does_not_import_the_v1_subtree_directly() {
    let allowed = [
        "legacy_event.rs",
        "permission_v1.rs",
        "question_v1.rs",
        "session_v1.rs",
    ];
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let src_dir = std::path::Path::new(manifest_dir).join("src");
    let mut direct_imports = Vec::new();
    for entry in std::fs::read_dir(&src_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
        if !file_name.ends_with(".rs") || allowed.iter().any(|name| *name == file_name) {
            continue;
        }
        let source = std::fs::read_to_string(&path).unwrap();
        if source.contains("from \"./v1/") || source.contains("crate::v1") {
            direct_imports.push(file_name);
        }
    }

    assert_eq!(direct_imports, Vec::<String>::new());
}
