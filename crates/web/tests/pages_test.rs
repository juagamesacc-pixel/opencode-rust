// Parity tests for `src/pages/[...slug].md.ts` and `src/pages/s/[id].astro`.
use web::pages;

#[test]
#[allow(clippy::assertions_on_constants)]
fn route_descriptors_match_source() {
    // `SHARE_PAGEFIND` is a route constant pinned against the source; asserting
    // it is the point of this test.
    assert_eq!(pages::MARKDOWN_ROUTE, "[...slug].md");
    assert_eq!(pages::MARKDOWN_METHOD, "GET");
    assert_eq!(pages::MARKDOWN_CONTENT_TYPE, "text/plain; charset=utf-8");
    assert_eq!(pages::NOT_FOUND_KEY, "share.not_found");
    assert_eq!(pages::INDEX_SLUG, "index");
    assert_eq!(pages::SHARE_ROUTE, "s/[id]");
    assert_eq!(pages::SHARE_TEMPLATE, "splash");
    assert!(!pages::SHARE_PAGEFIND);
    assert_eq!(
        pages::SHARE_ROBOTS,
        "noindex, nofollow, noarchive, nosnippet"
    );
    assert_eq!(pages::SHARE_DATA_PATH, "/share_data");
    assert_eq!(pages::FALLBACK_VERSION, "v0.0.1");
    assert_eq!(pages::TITLE_TRUNCATE_AT, 700);
    assert_eq!(pages::DESCRIPTION_TRUNCATE_AT, 400);
    assert_eq!(pages::SHARE_MESSAGE_KEYS.len(), 46);
    assert_eq!(pages::SHARE_MESSAGE_KEYS[0], "locale");
}

#[test]
fn share_message_keys_resolve_in_i18n_bundles() {
    for key in pages::SHARE_MESSAGE_KEYS.iter().skip(1) {
        assert!(
            web::i18n::is_known_key(&format!("share.{}", key)),
            "share.{} is a bundle key",
            key
        );
    }
    assert!(web::i18n::is_known_key(pages::NOT_FOUND_KEY));
}

#[test]
fn not_found_text_matches_source() {
    assert_eq!(pages::not_found_text(Some("Introuvable")), "Introuvable");
    assert_eq!(pages::not_found_text(Some("")), "share.not_found");
    assert_eq!(pages::not_found_text(None), "share.not_found");
}

#[test]
fn markdown_body_serves_collection_entries() {
    let page = pages::markdown_body("root", "config").expect("root/config");
    assert_eq!(page.slug, "config");
    assert!(pages::markdown_body("root", "missing").is_none());
    assert!(pages::markdown_body("uk", "index").is_none());
}

#[test]
fn version_label_matches_source() {
    assert_eq!(pages::version_label(Some("1.18.30")), "v1.18.30");
    assert_eq!(pages::version_label(Some("")), "v0.0.1");
    assert_eq!(pages::version_label(None), "v0.0.1");
}

#[test]
fn model_param_matches_source() {
    assert_eq!(pages::model_param(&[]), "");
    assert_eq!(pages::model_param(&["gpt-5"]), "gpt-5");
    assert_eq!(
        pages::model_param(&["a", "b"]),
        "a%20%26%20b",
        "two models are joined with ' & ' then encoded"
    );
    assert_eq!(
        pages::model_param(&["a", "b", "c"]),
        "a%20%26%202%20others",
        "three or more models collapse to a count"
    );
}

#[test]
fn encoders_match_web_vectors() {
    assert_eq!(pages::base64_encode(b""), "");
    assert_eq!(pages::base64_encode(b"f"), "Zg==");
    assert_eq!(pages::base64_encode(b"fo"), "Zm8=");
    assert_eq!(pages::base64_encode(b"foo"), "Zm9v");
    assert_eq!(pages::base64_encode(b"OpenCode"), "T3BlbkNvZGU=");
    assert_eq!(pages::encode_uri_component("a & b"), "a%20%26%20b");
    assert_eq!(pages::encode_uri_component("config"), "config");
    assert_eq!(pages::encode_share_title("Hi"), "SGk%3D");
}

#[test]
fn truncation_matches_source_limits() {
    assert_eq!(
        pages::encode_share_title(&"x".repeat(800)),
        pages::encode_share_title(&"x".repeat(700)),
        "titles truncate to 700 characters"
    );
    let short = pages::encode_docs_description("Get started with OpenCode.");
    assert_eq!(short, "Get%20started%20with%20OpenCode.");
    assert_eq!(pages::encode_docs_description(&"y".repeat(500)).len(), 400);
}
