#![allow(non_snake_case, dead_code)] // 1:1 source names/fields preserved, unused mirrors source exports
#![allow(non_upper_case_globals, non_camel_case_types)] // 1:1 source names like monoDefault, popularProviders_RAW, RUN_ENTRY_NONE stay verbatim
#![allow(clippy::invisible_characters)] // i18n strings byte-identical to source translations
#![allow(clippy::doc_lazy_continuation)] // doc comments mirror source text
#![allow(clippy::module_inception)] // mod.rs structure mirrors source dirs 1:1
#![allow(clippy::tabs_in_doc_comments)] // doc bytes preserved
pub mod addons;
pub mod app;
pub mod assets;
pub mod components;
pub mod constants;
pub mod context;
pub mod desktop_menu;
pub mod entry;
pub mod hooks;
pub mod i18n;
pub mod index;
pub mod pages;
pub mod updater;
pub mod utils;
pub mod wsl;
