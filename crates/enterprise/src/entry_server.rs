// source: packages/enterprise/src/entry-server.tsx — SSR document shell verbatim
//! PROVISIONAL: SolidStart server runtime — modeled as descriptor with verbatim strings.

/// source: `<title>OpenCode</title>` verbatim
pub const DOC_TITLE: &str = "OpenCode";
/// source: `theme-color` `#F8F7F7` verbatim
pub const THEME_COLOR: &str = "#F8F7F7";
/// source: body classes verbatim
pub const BODY_CLASS: &str = "antialiased overscroll-none text-12-regular";
/// source: mount node id verbatim
pub const APP_DIV_ID: &str = "app";
/// source: default lang `"en"` verbatim (same header rules as `app::detect_locale_from_header`)
pub const DEFAULT_LANG: &str = "en";

/// source: document lang resolution — header rules, default `"en"` verbatim
pub fn document_lang(header: Option<&str>) -> &'static str {
    crate::app::detect_locale_from_header(header).unwrap_or(DEFAULT_LANG)
}
