//! Port of `packages/schema/test/compatibility.test.ts`.
//!
//! Case names/order mirror the source (`describe` → module docs, `test` →
//! `#[test]` in order). The moved `FileSystem.FindInput` class schema remains
//! constructible with `query` preserved.

use schema::filesystem::FindInput;
use serde_json::json;

#[test]
fn moved_class_schemas_remain_constructible() {
    let input = FindInput {
        query: "src".to_string(),
        r#type: None,
        limit: None,
    };
    let value = serde_json::to_value(&input).unwrap();
    assert_eq!(input.query, "src");
    assert_eq!(value.get("query"), Some(&json!("src")));
}
