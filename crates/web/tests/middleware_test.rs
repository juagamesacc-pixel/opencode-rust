// Parity tests for `src/middleware.ts`.
use web::middleware::{self, DocsAlias, MiddlewareAction};

#[test]
fn verbatim_constants_match_source() {
    assert_eq!(middleware::DOCS_PREFIX, "/docs/");
    assert_eq!(middleware::REDIRECT_STATUS, 302);
    assert_eq!(middleware::COOKIE_MAX_AGE, 31536000);
}

#[test]
fn docs_alias_rewrites_locale_prefixes() {
    assert_eq!(
        DocsAlias::resolve("/docs/en/config"),
        Some(DocsAlias {
            path: "/docs/config".to_string(),
            locale: "root",
        })
    );
    assert_eq!(
        DocsAlias::resolve("/docs/pt/tui"),
        Some(DocsAlias {
            path: "/docs/pt-br/tui".to_string(),
            locale: "pt-br",
        })
    );
    // `exactLocale` lowercases, so an uppercase alias rewrites to the
    // canonical path (a lowercase `/docs/de` is already canonical → None).
    assert_eq!(
        DocsAlias::resolve("/docs/DE"),
        Some(DocsAlias {
            path: "/docs/de".to_string(),
            locale: "de",
        })
    );
    // Canonical paths are left alone.
    assert_eq!(DocsAlias::resolve("/docs/de/config"), None);
    assert_eq!(DocsAlias::resolve("/docs/config"), None);
    assert_eq!(DocsAlias::resolve("/docs/"), None);
    assert_eq!(DocsAlias::resolve("/docs"), None);
    assert_eq!(DocsAlias::resolve("/s/abc"), None);
    assert_eq!(DocsAlias::resolve("/"), None);
}

#[test]
fn locale_cookie_matches_source_format() {
    assert_eq!(
        middleware::locale_cookie("root"),
        "oc_locale=en; Path=/; Max-Age=31536000; SameSite=Lax"
    );
    assert_eq!(
        middleware::locale_cookie("pt-br"),
        "oc_locale=pt-br; Path=/; Max-Age=31536000; SameSite=Lax"
    );
}

#[test]
fn locale_from_cookie_matches_source() {
    assert_eq!(middleware::locale_from_cookie(None), None);
    assert_eq!(middleware::locale_from_cookie(Some("")), None);
    assert_eq!(
        middleware::locale_from_cookie(Some("a=1; oc_locale=de; b=2")),
        Some("de")
    );
    assert_eq!(
        middleware::locale_from_cookie(Some("oc_locale=pt-PT")),
        Some("pt-br")
    );
    assert_eq!(middleware::locale_from_cookie(Some("oc_locale=")), None);
    assert_eq!(middleware::locale_from_cookie(Some("other=1")), None);
}

#[test]
fn locale_from_accept_language_matches_source() {
    assert_eq!(middleware::locale_from_accept_language(None), "root");
    assert_eq!(middleware::locale_from_accept_language(Some("")), "root");
    assert_eq!(
        middleware::locale_from_accept_language(Some("de-DE,de;q=0.9")),
        "de"
    );
    assert_eq!(
        middleware::locale_from_accept_language(Some("fr-CA;q=0.5, ja;q=0.9")),
        "ja"
    );
    assert_eq!(middleware::locale_from_accept_language(Some("*")), "root");
    assert_eq!(
        middleware::locale_from_accept_language(Some("xx, yy;q=0.1")),
        "root"
    );
}

#[test]
fn handle_matches_on_request() {
    assert!(middleware::handle("/docs/config", None, None).is_continue());
    assert!(middleware::handle("/s/abc", None, None).is_continue());

    assert_eq!(
        middleware::handle("/docs/en/config", None, None),
        MiddlewareAction::Redirect {
            path: "/docs/config".to_string(),
            set_cookie: Some("oc_locale=en; Path=/; Max-Age=31536000; SameSite=Lax".to_string()),
        }
    );
    assert_eq!(
        middleware::handle("/docs", Some("oc_locale=ja"), None),
        MiddlewareAction::Redirect {
            path: "/docs/ja/".to_string(),
            set_cookie: None,
        }
    );
    assert_eq!(
        middleware::handle("/docs/", None, Some("fr-FR,fr;q=0.9")),
        MiddlewareAction::Redirect {
            path: "/docs/fr/".to_string(),
            set_cookie: None,
        }
    );
    assert!(middleware::handle("/docs/", None, None).is_continue());
    assert!(middleware::handle("/docs/", Some("oc_locale=en"), None).is_continue());
}
