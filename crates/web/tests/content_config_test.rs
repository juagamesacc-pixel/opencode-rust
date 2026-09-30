// Parity tests for `src/content.config.ts`.
use web::content_config;

#[test]
fn collection_wiring_matches_source() {
    assert_eq!(content_config::DOCS_COLLECTION, "docs");
    assert_eq!(content_config::DOCS_LOADER, "docsLoader");
    assert_eq!(content_config::DOCS_SCHEMA, "docsSchema");
    assert_eq!(content_config::I18N_COLLECTION, "i18n");
    assert_eq!(content_config::I18N_LOADER, "i18nLoader");
    assert_eq!(content_config::I18N_SCHEMA, "i18nSchema");
    assert_eq!(content_config::I18N_SCHEMA_SOURCE, "en.json");
}

#[test]
fn i18n_schema_covers_the_english_bundle() {
    assert_eq!(content_config::i18n_schema_key_count(), 73);
    assert_eq!(
        content_config::i18n_schema_key_count(),
        web::i18n::EN_KEY_COUNT
    );
    assert!(content_config::i18n_schema_has_key("share.not_found"));
    assert!(content_config::i18n_schema_has_key("app.header.docs"));
    assert!(!content_config::i18n_schema_has_key("missing.key"));
}
