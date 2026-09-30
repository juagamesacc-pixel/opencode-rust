// Parity tests for `src/content/i18n` and `src/content.config.ts` schema input.
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use web::i18n;

fn bundle_path(file: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(i18n::bundle_path(file))
}

#[test]
fn bundle_table_matches_source_tree() {
    assert_eq!(i18n::BUNDLES.len(), 18);
    assert!(i18n::BUNDLES.contains(&"en.json"));
    assert!(i18n::BUNDLES.contains(&"pt-BR.json"));
    assert!(i18n::BUNDLES.contains(&"zh-CN.json"));
    assert!(i18n::BUNDLES.contains(&"zh-TW.json"));
    for (file, count) in i18n::BUNDLE_KEY_COUNTS {
        assert_eq!(count, 73, "key count of {}", file);
        assert!(i18n::BUNDLES.contains(&file));
    }
    assert_eq!(i18n::EN_KEY_COUNT, 73);
    assert_eq!(i18n::EN_KEYS.len(), 73);
    assert_eq!(i18n::key_count("en.json"), Some(73));
    assert_eq!(i18n::key_count("missing.json"), None);
}

#[test]
fn english_key_list_matches_en_json() {
    let text = fs::read_to_string(bundle_path("en.json")).expect("en.json is copied");
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("en.json parses");
    let object = parsed.as_object().expect("en.json is an object");
    let keys: HashSet<&str> = object.keys().map(String::as_str).collect();
    let listed: HashSet<&str> = i18n::EN_KEYS.iter().copied().collect();
    assert_eq!(keys, listed);
    for key in i18n::EN_KEYS {
        assert!(i18n::is_known_key(key));
    }
    assert!(!i18n::is_known_key("missing.key"));
}

#[test]
fn every_bundle_parses_with_the_english_key_set() {
    let expected: HashSet<String> = i18n::EN_KEYS.iter().map(|key| key.to_string()).collect();
    for file in i18n::BUNDLES {
        let text = fs::read_to_string(bundle_path(file)).expect("bundle is copied");
        let parsed: serde_json::Value = serde_json::from_str(&text).expect("bundle parses");
        let object = parsed.as_object().expect("bundle is an object");
        let keys: HashSet<String> = object.keys().cloned().collect();
        assert_eq!(keys, expected, "key set of {}", file);
        for value in object.values() {
            assert!(value.is_string(), "string values in {}", file);
        }
    }
}
