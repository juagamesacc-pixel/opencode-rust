// source: packages/web/src/content.config.ts
//
// Descriptor of the Astro content collections. Collection loading and zod validation need the
// Astro toolchain, so this module records the collection wiring verbatim:
// PROVISIONAL: `defineCollection`, Starlight loaders and zod schemas need the Astro runtime.

/// Name of the docs collection.
pub const DOCS_COLLECTION: &str = "docs";

/// Loader used by the docs collection.
pub const DOCS_LOADER: &str = "docsLoader";

/// Schema used by the docs collection.
pub const DOCS_SCHEMA: &str = "docsSchema";

/// Name of the internationalization collection.
pub const I18N_COLLECTION: &str = "i18n";

/// Loader used by the `i18n` collection.
pub const I18N_LOADER: &str = "i18nLoader";

/// Base schema extended by the `i18n` collection.
pub const I18N_SCHEMA: &str = "i18nSchema";

/// The `i18n` schema is built from the English bundle: `z.object(custom).catchall(z.string())`.
pub const I18N_SCHEMA_SOURCE: &str = "en.json";

/// `true` when `key` is declared by the `i18n` collection schema.
pub fn i18n_schema_has_key(key: &str) -> bool {
    crate::i18n::is_known_key(key)
}

/// Number of keys the `i18n` collection schema declares.
pub fn i18n_schema_key_count() -> usize {
    crate::i18n::EN_KEY_COUNT
}
