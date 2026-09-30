// Parity tests for `astro.config.mjs`.
use web::starlight;

#[test]
#[allow(clippy::assertions_on_constants)]
fn site_wiring_matches_source() {
    // The bool constants below are config values pinned against
    // `astro.config.mjs`; asserting them is the point of this test.
    assert_eq!(starlight::BASE, "/docs");
    assert_eq!(starlight::OUTPUT, "server");
    assert_eq!(starlight::TITLE, "OpenCode");
    assert_eq!(starlight::DEFAULT_LOCALE, "root");
    assert_eq!(starlight::FAVICON, "/favicon-v3.svg");
    assert!(starlight::LAST_UPDATED);
    assert!(!starlight::HEADING_LINKS);
    assert_eq!(starlight::CUSTOM_CSS, "./src/styles/custom.css");
    assert_eq!(
        starlight::EXPRESSIVE_CODE_THEMES,
        ["github-light", "github-dark"]
    );
    assert_eq!(
        starlight::REHYPE_PLUGINS,
        ["rehypeHeadingIds", "rehype-autolink-headings"]
    );
    assert_eq!(starlight::AUTOLINK_HEADINGS_BEHAVIOR, "wrap");
    assert_eq!(
        starlight::INTEGRATIONS,
        [
            "configSchema",
            "solidJs",
            "starlight",
            "toolbeam-docs-theme"
        ]
    );
    assert_eq!(starlight::IMAGE_SERVICE, "passthrough");
    assert_eq!(starlight::SERVER_HOST, "0.0.0.0");
    assert!(!starlight::DEV_TOOLBAR_ENABLED);
    assert_eq!(starlight::LOGO_LIGHT, "./src/assets/logo-light.svg");
    assert_eq!(starlight::LOGO_DARK, "./src/assets/logo-dark.svg");
    assert!(starlight::LOGO_REPLACES_TITLE);
    assert_eq!(
        starlight::SOCIAL,
        [("github", "GitHub"), ("discord", "Discord")]
    );
    assert_eq!(starlight::HEAD_ICONS.len(), 3);
}

#[test]
fn starlight_locales_cover_docs_and_bundles() {
    assert_eq!(starlight::STARLIGHT_LOCALES.len(), 18);
    let root = starlight::starlight_locale("root").expect("root locale");
    assert_eq!(root.lang, "en");
    assert_eq!(root.label, "English");
    let arabic = starlight::starlight_locale("ar").expect("ar locale");
    assert_eq!(arabic.dir, "rtl");
    for entry in starlight::STARLIGHT_LOCALES {
        if entry.key != "ar" {
            assert_eq!(entry.dir, "ltr", "dir of {}", entry.key);
        }
        assert!(
            web::locales::is_known_locale(entry.key),
            "known {}",
            entry.key
        );
    }
    assert!(starlight::starlight_locale("en").is_none());
    assert!(starlight::starlight_locale("uk").is_none());
}

#[test]
fn sidebar_covers_every_default_locale_slug() {
    assert_eq!(starlight::SIDEBAR.len(), 10);
    for page in web::manifest::pages_for_locale("root") {
        let slug = if page.slug == "index" { "" } else { page.slug };
        assert!(
            starlight::sidebar_contains(slug),
            "sidebar has {}",
            page.slug
        );
    }
    assert!(starlight::sidebar_contains("windows-wsl"));
    assert!(!starlight::sidebar_contains("missing"));
}
