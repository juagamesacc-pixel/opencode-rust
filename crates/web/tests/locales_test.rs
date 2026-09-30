// Parity tests for `src/i18n/locales.ts`.
use web::locales;

#[test]
fn verbatim_tables_keep_source_sizes() {
    assert_eq!(locales::DOCS_LOCALE.len(), 18);
    assert_eq!(locales::LOCALE.len(), 19);
    assert_eq!(locales::LOCALE_ALIAS.len(), 26);
    assert_eq!(locales::STARTS.len(), 15);
    assert_eq!(locales::DEFAULT_LOCALE, "root");
    assert_eq!(locales::LOCALE_COOKIE, "oc_locale");
    assert_eq!(locales::LOCALE[0], "root");
    assert_eq!(&locales::LOCALE[1..], &locales::DOCS_LOCALE);
}

#[test]
fn declared_uk_locale_has_no_docs_directory() {
    // `docsLocale` declares `uk`, but the source tree ships no `docs/uk/` directory.
    assert!(locales::DOCS_LOCALE.contains(&"uk"));
    assert!(!web::manifest::LOCALES.contains(&"uk"));
    assert!(!locales::has_bundle("uk"));
    assert!(locales::is_docs_locale("uk"));
}

#[test]
fn exact_locale_matches_source() {
    assert_eq!(locales::exact_locale("de"), Some("de"));
    assert_eq!(locales::exact_locale("pt-BR"), Some("pt-br"));
    assert_eq!(locales::exact_locale("EN"), Some("root"));
    assert_eq!(locales::exact_locale("root"), Some("root"));
    assert_eq!(locales::exact_locale("zh"), Some("zh-cn"));
    assert_eq!(locales::exact_locale("  fr  "), Some("fr"));
    assert_eq!(locales::exact_locale("de-AT"), None);
    assert_eq!(locales::exact_locale(""), None);
    assert_eq!(locales::exact_locale("%E0%A4%A"), None);
}

#[test]
fn match_locale_matches_source() {
    assert_eq!(locales::match_locale("de-AT"), Some("de"));
    assert_eq!(locales::match_locale("pt-PT"), Some("pt-br"));
    assert_eq!(locales::match_locale("nb-NO"), Some("nb"));
    assert_eq!(locales::match_locale("nn"), Some("nb"));
    assert_eq!(locales::match_locale("no"), Some("nb"));
    assert_eq!(locales::match_locale("en-US"), Some("root"));
    assert_eq!(locales::match_locale("zh-Hant-TW"), Some("zh-tw"));
    assert_eq!(locales::match_locale("zh-HK"), Some("zh-tw"));
    assert_eq!(locales::match_locale("zh-MO"), Some("zh-tw"));
    assert_eq!(locales::match_locale("zh-Hans-CN"), Some("zh-cn"));
    assert_eq!(locales::match_locale("ko-KR"), Some("ko"));
    assert_eq!(locales::match_locale("uk-UA"), Some("uk"));
    assert_eq!(locales::match_locale("xx"), None);
    assert_eq!(locales::match_locale("*"), None);
    assert_eq!(locales::match_locale(""), None);
}

#[test]
fn every_alias_target_is_a_known_locale() {
    for (_, target) in locales::LOCALE_ALIAS {
        assert!(locales::is_known_locale(target), "alias target {}", target);
    }
    for (_, target) in locales::STARTS {
        assert!(locales::is_known_locale(target), "starts target {}", target);
    }
}

#[test]
fn bundles_exist_for_every_docs_locale() {
    for locale in web::manifest::LOCALES {
        assert!(locales::has_bundle(locale), "bundle for {}", locale);
    }
    assert!(locales::has_bundle("pt-br"));
    assert!(!locales::has_bundle("xx"));
}
