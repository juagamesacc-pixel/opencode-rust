// source: packages/web/src/content/docs, packages/web/src/content/i18n,
// packages/web/src/assets, packages/web/src/styles, packages/web/public (Astro docs site)
//
// Rust port of the `@opencode-ai/web` docs site. Documentation pages, message bundles, assets
// and styles are copied byte-for-byte next to this crate; the Astro/Starlight configuration,
// routes, middleware and components are captured as descriptor modules.

pub mod assets;
pub mod components;
pub mod config;
pub mod content_config;
pub mod i18n;
pub mod locales;
pub mod manifest;
pub mod middleware;
pub mod pages;
pub mod starlight;
