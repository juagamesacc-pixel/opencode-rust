// source: packages/web/src/i18n/locales.ts
//
// 1:1 port. The pure lookup helpers are implemented in Rust; the `docsLocale`, `locale`,
// `localeAlias` and `starts` tables are kept verbatim.

/// Starlight's default (untranslated) locale key, i.e. `docs/<slug>.mdx`.
pub const DEFAULT_LOCALE: &str = "root";

/// Cookie name written by the middleware and by the language selector.
pub const LOCALE_COOKIE: &str = "oc_locale";

/// `docsLocale` — locales that have a directory under `src/content/docs`.
pub const DOCS_LOCALE: [&str; 18] = [
    "ar", "bs", "da", "de", "es", "fr", "it", "ja", "ko", "nb", "pl", "pt-br", "ru", "th", "tr",
    "uk", "zh-cn", "zh-tw",
];

/// `locale` — `["root", ...docsLocale]`.
pub const LOCALE: [&str; 19] = [
    "root", "ar", "bs", "da", "de", "es", "fr", "it", "ja", "ko", "nb", "pl", "pt-br", "ru", "th",
    "tr", "uk", "zh-cn", "zh-tw",
];

/// `localeAlias` — exact-match alias table, verbatim.
pub const LOCALE_ALIAS: [(&str, &str); 26] = [
    ("ar", "ar"),
    ("br", "pt-br"),
    ("bs", "bs"),
    ("da", "da"),
    ("de", "de"),
    ("en", "root"),
    ("es", "es"),
    ("fr", "fr"),
    ("it", "it"),
    ("ja", "ja"),
    ("ko", "ko"),
    ("nb", "nb"),
    ("nn", "nb"),
    ("no", "nb"),
    ("pl", "pl"),
    ("pt", "pt-br"),
    ("pt-br", "pt-br"),
    ("root", "root"),
    ("ru", "ru"),
    ("th", "th"),
    ("tr", "tr"),
    ("uk", "uk"),
    ("zh", "zh-cn"),
    ("zh-cn", "zh-cn"),
    ("zht", "zh-tw"),
    ("zh-tw", "zh-tw"),
];

/// `starts` — prefix fallback table consulted by [`match_locale`], verbatim.
pub const STARTS: [(&str, &str); 15] = [
    ("ko", "ko"),
    ("bs", "bs"),
    ("de", "de"),
    ("es", "es"),
    ("fr", "fr"),
    ("it", "it"),
    ("da", "da"),
    ("ja", "ja"),
    ("pl", "pl"),
    ("ru", "ru"),
    ("uk", "uk"),
    ("ar", "ar"),
    ("th", "th"),
    ("tr", "tr"),
    ("en", "root"),
];

/// `parse` — `decodeURIComponent`, trim and lowercase; `None` on malformed input.
pub fn parse(input: &str) -> Option<String> {
    let decoded = percent_decode(input)?;
    let value = decoded.trim().to_lowercase();
    if value.is_empty() {
        return None;
    }
    Some(value)
}

/// `exactLocale` — alias lookup only.
pub fn exact_locale(input: &str) -> Option<&'static str> {
    let value = parse(input)?;
    LOCALE_ALIAS
        .iter()
        .find(|(alias, _)| *alias == value)
        .map(|(_, locale)| *locale)
}

/// `matchLocale` — alias lookup with `zh` / `pt` / `no` handling and a prefix fallback.
pub fn match_locale(input: &str) -> Option<&'static str> {
    let value = parse(input)?;

    if value.starts_with("zh") {
        if value.contains("hant")
            || value.contains("-tw")
            || value.contains("-hk")
            || value.contains("-mo")
        {
            return Some("zh-tw");
        }
        return Some("zh-cn");
    }

    if let Some((_, locale)) = LOCALE_ALIAS.iter().find(|(alias, _)| *alias == value) {
        return Some(locale);
    }

    if value.starts_with("pt") {
        return Some("pt-br");
    }

    if value.starts_with("no") || value.starts_with("nb") || value.starts_with("nn") {
        return Some("nb");
    }

    STARTS
        .iter()
        .find(|(prefix, _)| value.starts_with(prefix))
        .map(|(_, locale)| *locale)
}

/// `true` when `locale` is part of the `locale` tuple.
pub fn is_known_locale(locale: &str) -> bool {
    LOCALE.contains(&locale)
}

/// `true` when `locale` has a directory under `src/content/docs`.
///
/// Note: `docsLocale` also declares `uk`, but the source tree ships no `docs/uk/` directory.
pub fn is_docs_locale(locale: &str) -> bool {
    DOCS_LOCALE.contains(&locale)
}

/// `true` when the `i18n` collection ships a message bundle for `locale` (case-insensitive,
/// so `pt-BR.json` matches the `pt-br` docs directory).
///
/// The `root` docs locale is served the English bundle everywhere in the
/// source (`middleware.ts`: `locale === "root" ? "en" : locale`, same in
/// `components/share/common.tsx` and `pages/s/[id].astro`), so `root` resolves
/// to `en.json` here too.
pub fn has_bundle(locale: &str) -> bool {
    let bundle = if locale.eq_ignore_ascii_case("root") {
        "en"
    } else {
        locale
    };
    crate::i18n::BUNDLES
        .iter()
        .any(|file| file.eq_ignore_ascii_case(&format!("{}.json", bundle)))
}

/// `decodeURIComponent` — percent-decodes UTF-8, `None` on a malformed sequence.
fn percent_decode(input: &str) -> Option<String> {
    if !input.contains('%') {
        return Some(input.to_string());
    }

    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hex = input.get(index + 1..index + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            index += 3;
            continue;
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8(out).ok()
}
