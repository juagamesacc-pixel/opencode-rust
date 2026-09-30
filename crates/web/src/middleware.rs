// source: packages/web/src/middleware.ts
//
// 1:1 port. The locale detection, cookie and redirect-target logic is pure and runs here; only
// the Astro request plumbing is out of scope:
// PROVISIONAL: `defineMiddleware` request/response wiring needs the Astro runtime.

use std::cmp::Ordering;

/// Docs route handled by the alias redirect, verbatim from the middleware regex.
pub const DOCS_PREFIX: &str = "/docs/";

/// Redirect status used for both alias and locale redirects.
pub const REDIRECT_STATUS: u16 = 302;

/// Cookie `Max-Age` in seconds (one year), verbatim.
pub const COOKIE_MAX_AGE: u32 = 31536000;

/// Decision returned by [`handle`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MiddlewareAction {
    /// Continue to the next middleware / route.
    Continue,
    /// Issue a redirect to `path`, optionally setting the locale cookie.
    Redirect {
        /// Target path (the request origin is preserved by the caller).
        path: String,
        /// Value for the `Set-Cookie` header, if any.
        set_cookie: Option<String>,
    },
}

impl MiddlewareAction {
    /// `true` when the request continues without a redirect.
    pub fn is_continue(&self) -> bool {
        matches!(self, MiddlewareAction::Continue)
    }
}

/// The resolved `/docs/<locale>` alias target: `docsAlias` in the source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsAlias {
    /// Redirect target path.
    pub path: String,
    /// Matched locale key.
    pub locale: &'static str,
}

impl DocsAlias {
    /// Resolve `path` like the source: `/docs/<alias>(/...)` becomes `/docs[/<locale>](/...)`.
    pub fn resolve(pathname: &str) -> Option<Self> {
        let rest = pathname.strip_prefix(DOCS_PREFIX)?;
        let (value, tail) = match rest.find('/') {
            Some(index) => (&rest[..index], &rest[index..]),
            None => (rest, ""),
        };
        let locale = crate::locales::exact_locale(value)?;
        let path = if locale == crate::locales::DEFAULT_LOCALE {
            format!("/docs{}", tail)
        } else {
            format!("/docs/{}{}", locale, tail)
        };
        if path == pathname {
            return None;
        }
        Some(DocsAlias { path, locale })
    }
}

/// Build the `oc_locale` cookie value: `cookie` in the source.
pub fn locale_cookie(locale: &str) -> String {
    let value = if locale == crate::locales::DEFAULT_LOCALE {
        "en"
    } else {
        locale
    };
    format!(
        "{}={}; Path=/; Max-Age={}; SameSite=Lax",
        crate::locales::LOCALE_COOKIE,
        encode_uri_component(value),
        COOKIE_MAX_AGE
    )
}

/// Read the locale from the `Cookie` header: `localeFromCookie` in the source.
pub fn locale_from_cookie(header: Option<&str>) -> Option<&'static str> {
    let header = header?;
    let raw = header
        .split(';')
        .map(str::trim)
        .find(|part| part.starts_with("oc_locale="))?
        .strip_prefix("oc_locale=")?;
    if raw.is_empty() {
        return None;
    }
    crate::locales::match_locale(raw)
}

/// Read the locale from the `Accept-Language` header: `localeFromAcceptLanguage` in the source.
pub fn locale_from_accept_language(header: Option<&str>) -> &'static str {
    let header = match header {
        None => return crate::locales::DEFAULT_LOCALE,
        Some(header) => header,
    };

    let mut items: Vec<(&str, f64)> = header
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut pieces = part.split(';').map(str::trim);
            let lang = pieces.next().unwrap_or("");
            let quality = pieces
                .find(|piece| piece.starts_with("q="))
                .map(|piece| piece[2..].parse::<f64>().unwrap_or(f64::NAN))
                .unwrap_or(1.0);
            (lang, quality)
        })
        .collect();
    items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

    for (lang, _) in items {
        if lang.is_empty() || lang == "*" {
            continue;
        }
        if let Some(locale) = crate::locales::match_locale(lang) {
            return locale;
        }
    }
    crate::locales::DEFAULT_LOCALE
}

/// Run the middleware decision for a request: `onRequest` in the source.
///
/// `pathname` is the request path; `cookie` and `accept_language` are the raw header values.
pub fn handle(
    pathname: &str,
    cookie: Option<&str>,
    accept_language: Option<&str>,
) -> MiddlewareAction {
    if let Some(alias) = DocsAlias::resolve(pathname) {
        return MiddlewareAction::Redirect {
            path: alias.path,
            set_cookie: Some(locale_cookie(alias.locale)),
        };
    }

    if pathname != "/docs" && pathname != "/docs/" {
        return MiddlewareAction::Continue;
    }

    let locale =
        locale_from_cookie(cookie).unwrap_or_else(|| locale_from_accept_language(accept_language));
    if locale == crate::locales::DEFAULT_LOCALE {
        return MiddlewareAction::Continue;
    }

    MiddlewareAction::Redirect {
        path: format!("/docs/{}/", locale),
        set_cookie: None,
    }
}

/// `encodeURIComponent` for the ASCII inputs this crate passes (locale names, path parts).
pub fn encode_uri_component(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        if byte.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&byte) {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{:02X}", byte));
        }
    }
    out
}
