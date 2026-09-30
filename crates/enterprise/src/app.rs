// source: packages/enterprise/src/app.tsx — locale detection + provider stack verbatim
//! 1:1 port — `resolveTemplate` (`{{ key }}` substitution), accept-language detection
//! (`zh`/`en` prefix rules + fallthrough order), and provider nesting preserved verbatim.
//! Solid reactivity/JSX is PROVISIONAL — modeled as descriptors.

/// source: `{{ key }}` template substitution — missing params render `""` verbatim
pub fn resolve_template(text: &str, params: Option<&[(&str, &str)]>) -> String {
    let Some(params) = params else {
        return text.to_string();
    };
    let mut out = String::new();
    let mut rest = text;
    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        match after.find("}}") {
            None => {
                out.push_str(&rest[start..]);
                return out;
            }
            Some(end) => {
                let key = after[..end].trim();
                let value = params
                    .iter()
                    .find(|(k, _)| *k == key)
                    .map(|(_, v)| *v)
                    .unwrap_or("");
                out.push_str(value);
                rest = &after[end + 2..];
            }
        }
    }
    out.push_str(rest);
    out
}

/// source: `detectLocaleFromHeader` — first `zh*` → `"zh"`, first `en*` → `"en"` verbatim
pub fn detect_locale_from_header(header: Option<&str>) -> Option<&'static str> {
    let header = header?;
    for item in header.split(',') {
        let value = item.trim().split(';').next()?.trim().to_lowercase();
        if value.is_empty() {
            continue;
        }
        if value.starts_with("zh") {
            return Some("zh");
        }
        if value.starts_with("en") {
            return Some("en");
        }
    }
    None
}

/// source: `detectLocale` fallthrough — header → `document.documentElement.lang` → navigator
/// languages (zh-only check) → `"en"` verbatim
pub fn detect_locale(
    header: Option<&str>,
    document_lang: Option<&str>,
    navigator_languages: &[String],
) -> &'static str {
    if let Some(locale) = detect_locale_from_header(header) {
        return locale;
    }
    if let Some(lang) = document_lang {
        let lower = lang.to_lowercase();
        if lower.starts_with("zh") {
            return "zh";
        }
        if lower.starts_with("en") {
            return "en";
        }
    }
    for language in navigator_languages {
        if language.to_lowercase().starts_with("zh") {
            return "zh";
        }
    }
    "en"
}

/// source: provider nesting `Meta > Dialog > Marked > Favicon + Font > UiI18nBridge > Suspense` verbatim
pub const PROVIDER_STACK: &[&str] = &[
    "Meta",
    "Dialog",
    "Marked",
    "Favicon",
    "Font",
    "UiI18nBridge",
    "Suspense",
];
/// source: supported locales verbatim
pub const SUPPORTED_LOCALES: &[&str] = &["en", "zh"];
