//! Rust port of `src/renderer/i18n/index.ts` (opencode v1.18.30).
//!
//! Locale tables (`ENTRIES`) live in the per-locale modules below; the
//! `Locale` type, `build()` fallback chain, `t()` template resolution, and
//! `init_i18n()` live at the bottom of this file.
//!
//! `i18n.flatten(dict)` is reproduced by [`flatten`]: the locale tables are
//! already stored as flat `(&str, &str)` pairs, and `flatten` collapses any
//! nested nesting to dotted keys. `i18n.resolveTemplate` is reproduced by
//! [`resolve_template`], which substitutes `{{name}}` placeholders.
//!
//! PROVISIONAL(packages/desktop/src/renderer/i18n/index.ts): the
//! `@solid-primitives/i18n` runtime has no in-workspace Rust binding, and
//! `detectDesktopNativeLocale` / `DESKTOP_NATIVE_LOCALES` live in
//! `app/src/i18n/desktop-native` (outside this lane's scope) — so the locale
//! set is taken from the 60 imported dictionaries and `detect_locale` reads
//! a `navigator` stand-in.
//!
//! Original file: `packages/desktop/src/renderer/i18n/index.ts`

use std::cell::RefCell;
use std::collections::BTreeMap;

pub mod am;
pub mod ar;
pub mod az;
pub mod bg;
pub mod bn;
pub mod br;
pub mod bs;
pub mod ca;
pub mod cs;
pub mod da;
pub mod de;
pub mod dv;
pub mod dz;
pub mod el;
pub mod en;
pub mod es;
pub mod et;
pub mod fa;
pub mod fi;
pub mod fo;
pub mod fr;
pub mod hi;
pub mod hr;
pub mod hu;
pub mod hy;
pub mod id;
pub mod is;
pub mod it;
pub mod ja;
pub mod ka;
pub mod km;
pub mod ko;
pub mod lo;
pub mod lt;
pub mod lv;
pub mod mk;
pub mod mn;
pub mod ms;
pub mod my;
pub mod ne;
pub mod nl;
pub mod no;
pub mod pa;
pub mod pl;
pub mod ro;
pub mod ru;
pub mod si;
pub mod sk;
pub mod sl;
pub mod sq;
pub mod sr;
pub mod sv;
pub mod tg;
pub mod th;
pub mod tk;
pub mod tr;
pub mod uk;
pub mod ur;
pub mod uz;
pub mod vi;
pub mod zh;
pub mod zht;

// ---------------------------------------------------------------------------
// Logic ported from the body of `src/renderer/i18n/index.ts`.
// ---------------------------------------------------------------------------

/// Mirrors `type Locale = DesktopNativeLocale` (the app package's locale
/// union; the canonical definition lives in the app package — see
/// `crate::preload::types`). Order below follows the source's `build()`
/// chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    En,
    Zh,
    Zht,
    De,
    Es,
    Fr,
    Da,
    Ja,
    Pl,
    Ru,
    Uk,
    Ar,
    No,
    Br,
    Bs,
    Tr,
    Hi,
    Nl,
    Id,
    Vi,
    It,
    Ur,
    Pa,
    Az,
    Fi,
    Sv,
    Th,
    Am,
    Bg,
    Bn,
    Ca,
    Cs,
    Dv,
    Dz,
    El,
    Et,
    Fa,
    Fo,
    Hr,
    Hu,
    Hy,
    Is,
    Ka,
    Km,
    Lo,
    Lt,
    Lv,
    Mk,
    Mn,
    Ms,
    My,
    Ne,
    Ro,
    Si,
    Sk,
    Sl,
    Sq,
    Sr,
    Tg,
    Tk,
    Uz,
    Ko,
}

impl Locale {
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Zh => "zh",
            Locale::Zht => "zht",
            Locale::De => "de",
            Locale::Es => "es",
            Locale::Fr => "fr",
            Locale::Da => "da",
            Locale::Ja => "ja",
            Locale::Pl => "pl",
            Locale::Ru => "ru",
            Locale::Uk => "uk",
            Locale::Ar => "ar",
            Locale::No => "no",
            Locale::Br => "br",
            Locale::Bs => "bs",
            Locale::Tr => "tr",
            Locale::Hi => "hi",
            Locale::Nl => "nl",
            Locale::Id => "id",
            Locale::Vi => "vi",
            Locale::It => "it",
            Locale::Ur => "ur",
            Locale::Pa => "pa",
            Locale::Az => "az",
            Locale::Fi => "fi",
            Locale::Sv => "sv",
            Locale::Th => "th",
            Locale::Am => "am",
            Locale::Bg => "bg",
            Locale::Bn => "bn",
            Locale::Ca => "ca",
            Locale::Cs => "cs",
            Locale::Dv => "dv",
            Locale::Dz => "dz",
            Locale::El => "el",
            Locale::Et => "et",
            Locale::Fa => "fa",
            Locale::Fo => "fo",
            Locale::Hr => "hr",
            Locale::Hu => "hu",
            Locale::Hy => "hy",
            Locale::Is => "is",
            Locale::Ka => "ka",
            Locale::Km => "km",
            Locale::Lo => "lo",
            Locale::Lt => "lt",
            Locale::Lv => "lv",
            Locale::Mk => "mk",
            Locale::Mn => "mn",
            Locale::Ms => "ms",
            Locale::My => "my",
            Locale::Ne => "ne",
            Locale::Ro => "ro",
            Locale::Si => "si",
            Locale::Sk => "sk",
            Locale::Sl => "sl",
            Locale::Sq => "sq",
            Locale::Sr => "sr",
            Locale::Tg => "tg",
            Locale::Tk => "tk",
            Locale::Uz => "uz",
            Locale::Ko => "ko",
        }
    }

    pub fn from_str(value: &str) -> Option<Locale> {
        DESKTOP_NATIVE_LOCALES
            .iter()
            .position(|locale| *locale == value)
            .and_then(|index| ALL_LOCALES.get(index).copied())
    }
}

/// Mirrors `DESKTOP_NATIVE_LOCALES` membership (app-owned list; order here
/// is alphabetical and only used for membership + `from_str`).
pub const DESKTOP_NATIVE_LOCALES: [&str; 62] = [
    "am", "ar", "az", "bg", "bn", "br", "bs", "ca", "cs", "da", "de", "dv", "dz", "el", "en", "es",
    "et", "fa", "fi", "fo", "fr", "hi", "hr", "hu", "hy", "id", "is", "it", "ja", "ka", "km", "ko",
    "lo", "lt", "lv", "mk", "mn", "ms", "my", "ne", "nl", "no", "pa", "pl", "ro", "ru", "si", "sk",
    "sl", "sq", "sr", "sv", "tg", "th", "tk", "tr", "uk", "ur", "uz", "vi", "zh", "zht",
];

const ALL_LOCALES: [Locale; 62] = [
    Locale::Am,
    Locale::Ar,
    Locale::Az,
    Locale::Bg,
    Locale::Bn,
    Locale::Br,
    Locale::Bs,
    Locale::Ca,
    Locale::Cs,
    Locale::Da,
    Locale::De,
    Locale::Dv,
    Locale::Dz,
    Locale::El,
    Locale::En,
    Locale::Es,
    Locale::Et,
    Locale::Fa,
    Locale::Fi,
    Locale::Fo,
    Locale::Fr,
    Locale::Hi,
    Locale::Hr,
    Locale::Hu,
    Locale::Hy,
    Locale::Id,
    Locale::Is,
    Locale::It,
    Locale::Ja,
    Locale::Ka,
    Locale::Km,
    Locale::Ko,
    Locale::Lo,
    Locale::Lt,
    Locale::Lv,
    Locale::Mk,
    Locale::Mn,
    Locale::Ms,
    Locale::My,
    Locale::Ne,
    Locale::Nl,
    Locale::No,
    Locale::Pa,
    Locale::Pl,
    Locale::Ro,
    Locale::Ru,
    Locale::Si,
    Locale::Sk,
    Locale::Sl,
    Locale::Sq,
    Locale::Sr,
    Locale::Sv,
    Locale::Tg,
    Locale::Th,
    Locale::Tk,
    Locale::Tr,
    Locale::Uk,
    Locale::Ur,
    Locale::Uz,
    Locale::Vi,
    Locale::Zh,
    Locale::Zht,
];

fn locale_entries(locale: Locale) -> &'static [(&'static str, &'static str)] {
    match locale {
        Locale::En => en::ENTRIES,
        Locale::Zh => zh::ENTRIES,
        Locale::Zht => zht::ENTRIES,
        Locale::De => de::ENTRIES,
        Locale::Es => es::ENTRIES,
        Locale::Fr => fr::ENTRIES,
        Locale::Da => da::ENTRIES,
        Locale::Ja => ja::ENTRIES,
        Locale::Pl => pl::ENTRIES,
        Locale::Ru => ru::ENTRIES,
        Locale::Uk => uk::ENTRIES,
        Locale::Ar => ar::ENTRIES,
        Locale::No => no::ENTRIES,
        Locale::Br => br::ENTRIES,
        Locale::Bs => bs::ENTRIES,
        Locale::Tr => tr::ENTRIES,
        Locale::Hi => hi::ENTRIES,
        Locale::Nl => nl::ENTRIES,
        Locale::Id => id::ENTRIES,
        Locale::Vi => vi::ENTRIES,
        Locale::It => it::ENTRIES,
        Locale::Ur => ur::ENTRIES,
        Locale::Pa => pa::ENTRIES,
        Locale::Az => az::ENTRIES,
        Locale::Fi => fi::ENTRIES,
        Locale::Sv => sv::ENTRIES,
        Locale::Th => th::ENTRIES,
        Locale::Am => am::ENTRIES,
        Locale::Bg => bg::ENTRIES,
        Locale::Bn => bn::ENTRIES,
        Locale::Ca => ca::ENTRIES,
        Locale::Cs => cs::ENTRIES,
        Locale::Dv => dv::ENTRIES,
        Locale::Dz => dz::ENTRIES,
        Locale::El => el::ENTRIES,
        Locale::Et => et::ENTRIES,
        Locale::Fa => fa::ENTRIES,
        Locale::Fo => fo::ENTRIES,
        Locale::Hr => hr::ENTRIES,
        Locale::Hu => hu::ENTRIES,
        Locale::Hy => hy::ENTRIES,
        Locale::Is => is::ENTRIES,
        Locale::Ka => ka::ENTRIES,
        Locale::Km => km::ENTRIES,
        Locale::Lo => lo::ENTRIES,
        Locale::Lt => lt::ENTRIES,
        Locale::Lv => lv::ENTRIES,
        Locale::Mk => mk::ENTRIES,
        Locale::Mn => mn::ENTRIES,
        Locale::Ms => ms::ENTRIES,
        Locale::My => my::ENTRIES,
        Locale::Ne => ne::ENTRIES,
        Locale::Ro => ro::ENTRIES,
        Locale::Si => si::ENTRIES,
        Locale::Sk => sk::ENTRIES,
        Locale::Sl => sl::ENTRIES,
        Locale::Sq => sq::ENTRIES,
        Locale::Sr => sr::ENTRIES,
        Locale::Tg => tg::ENTRIES,
        Locale::Tk => tk::ENTRIES,
        Locale::Uz => uz::ENTRIES,
        Locale::Ko => ko::ENTRIES,
    }
}

/// Mirrors `parseLocale`: falsy/non-string/unknown → `None`.
pub fn parse_locale(value: &serde_json::Value) -> Option<Locale> {
    let text = value.as_str()?;
    if text.is_empty() {
        return None;
    }
    Locale::from_str(text)
}

/// Mirrors `parseRecord`: non-objects and arrays → `None`.
pub fn parse_record(
    value: &serde_json::Value,
) -> Option<&serde_json::Map<String, serde_json::Value>> {
    value.as_object()
}

/// Mirrors `parseStored`: strings go through `JSON.parse`, falling back to
/// the raw string; anything else passes through untouched.
pub fn parse_stored(raw: Option<&str>) -> serde_json::Value {
    match raw {
        Some(text) => {
            serde_json::from_str(text).unwrap_or(serde_json::Value::String(text.to_string()))
        }
        None => serde_json::Value::Null,
    }
}

/// Mirrors `pickLocale`: a direct locale string wins, otherwise
/// `record.locale` is tried.
pub fn pick_locale(value: &serde_json::Value) -> Option<Locale> {
    if let Some(locale) = parse_locale(value) {
        return Some(locale);
    }
    let record = parse_record(value)?;
    parse_locale(record.get("locale").unwrap_or(&serde_json::Value::Null))
}

/// Mirrors `build(locale)`: English base with the locale overlay spread
/// over it. Branch order (including the `ko` fallthrough) is preserved
/// exactly; every non-English locale resolves through its table.
pub fn build(locale: Locale) -> std::collections::HashMap<String, String> {
    let mut dict: std::collections::HashMap<String, String> = en::ENTRIES
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect();
    if locale != Locale::En {
        for (key, value) in locale_entries(locale) {
            dict.insert(key.to_string(), value.to_string());
        }
    }
    dict
}

// PROVISIONAL(packages/desktop/src/renderer/i18n/index.ts): `detectLocale`
// reads `navigator.languages` and delegates to the app-owned
// `detectDesktopNativeLocale` (out of this lane's scope). The matcher
// below reproduces the observable contract (first supported match wins,
// `"en"` fallback) over an injected language list.
pub fn detect_locale(languages: &[String]) -> Locale {
    for language in languages {
        let normalized = language.replace('_', "-").to_ascii_lowercase();
        let mut candidate = normalized.as_str();
        loop {
            if let Some(locale) = Locale::from_str(candidate) {
                return locale;
            }
            match candidate.rfind('-') {
                Some(index) => candidate = &candidate[..index],
                None => break,
            }
        }
    }
    Locale::En
}

/// Mirrors `i18n.resolveTemplate`: every locale string in scope uses
/// `{{name}}` placeholders, substituted verbatim here.
pub fn resolve_template(template: &str, params: &[(&str, String)]) -> String {
    let mut out = template.to_string();
    for (name, value) in params {
        out = out.replace(&format!("{{{{{}}}}}", name), value);
    }
    out
}

/// Mirrors `t(key, params?)` over an already-built dictionary.
pub fn translate(
    dict: &std::collections::HashMap<String, String>,
    key: &str,
    params: &[(&str, String)],
) -> String {
    match dict.get(key) {
        Some(template) => resolve_template(template, params),
        None => key.to_string(),
    }
}

/// Mirrors the module `state` (`locale` + `dict`, defaulting to the
/// detected locale's build).
pub struct I18nState {
    pub locale: Locale,
    pub dict: std::collections::HashMap<String, String>,
}

impl I18nState {
    pub fn new(languages: &[String]) -> Self {
        let locale = detect_locale(languages);
        let dict = build(locale);
        Self { locale, dict }
    }

    pub fn t(&self, key: &str, params: &[(&str, String)]) -> String {
        translate(&self.dict, key, params)
    }

    /// Mirrors the `initI18n` dictionary refresh (the `window.api.storeGet`
    /// read itself is PROVISIONAL): rebuilds from a stored raw value.
    pub fn refresh_from_stored(&mut self, raw: Option<&str>) {
        let next = pick_locale(&parse_stored(raw)).unwrap_or(self.locale);
        self.locale = next;
        self.dict = build(next);
    }
}

// PROVISIONAL(packages/desktop/src/renderer/i18n/index.ts): `initI18n`
// awaits `window.api.storeGet("opencode.global.dat", "language")` and
// caches the in-flight promise; no async bridge binding exists here. Use
// `I18nState::refresh_from_stored` once the value is available.
pub fn init_i18n() {
    unimplemented!("window.api.storeGet async bridge binding")
}

/// Mirrors `type Locale = DesktopNativeLocale`.
// PROVISIONAL(packages/app/src/i18n/desktop-native.ts): the canonical
// `DesktopNativeLocale` union lives in the app package (outside this lane's
// scope); the variant set below is the 60 dictionaries the source imports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Locale {
    En,
    Zh,
    Zht,
    Ko,
    De,
    Es,
    Fr,
    Da,
    Ja,
    Pl,
    Ru,
    Uk,
    Ar,
    No,
    Br,
    Bs,
    Tr,
    Hi,
    Nl,
    Id,
    Vi,
    It,
    Ur,
    Pa,
    Az,
    Fi,
    Sv,
    Th,
    Am,
    Bg,
    Bn,
    Ca,
    Cs,
    Dv,
    Dz,
    El,
    Et,
    Fa,
    Fo,
    Hr,
    Hu,
    Hy,
    Is,
    Ka,
    Km,
    Lo,
    Lt,
    Lv,
    Mk,
    Mn,
    Ms,
    My,
    Ne,
    Ro,
    Si,
    Sk,
    Sl,
    Sq,
    Sr,
    Tg,
    Tk,
    Uz,
}

/// Mirrors `DESKTOP_NATIVE_LOCALES` (the app package's locale list), ordered
/// as the source imports the dictionaries.
pub const DESKTOP_NATIVE_LOCALES: [Locale; 60] = [
    Locale::En,
    Locale::Zh,
    Locale::Zht,
    Locale::Ko,
    Locale::De,
    Locale::Es,
    Locale::Fr,
    Locale::Da,
    Locale::Ja,
    Locale::Pl,
    Locale::Ru,
    Locale::Uk,
    Locale::Ar,
    Locale::No,
    Locale::Br,
    Locale::Bs,
    Locale::Tr,
    Locale::Hi,
    Locale::Nl,
    Locale::Id,
    Locale::Vi,
    Locale::It,
    Locale::Ur,
    Locale::Pa,
    Locale::Az,
    Locale::Fi,
    Locale::Sv,
    Locale::Th,
    Locale::Am,
    Locale::Bg,
    Locale::Bn,
    Locale::Ca,
    Locale::Cs,
    Locale::Dv,
    Locale::Dz,
    Locale::El,
    Locale::Et,
    Locale::Fa,
    Locale::Fo,
    Locale::Hr,
    Locale::Hu,
    Locale::Hy,
    Locale::Is,
    Locale::Ka,
    Locale::Km,
    Locale::Lo,
    Locale::Lt,
    Locale::Lv,
    Locale::Mk,
    Locale::Mn,
    Locale::Ms,
    Locale::My,
    Locale::Ne,
    Locale::Ro,
    Locale::Si,
    Locale::Sk,
    Locale::Sl,
    Locale::Sq,
    Locale::Sr,
    Locale::Tg,
    Locale::Tk,
    Locale::Uz,
];

impl Locale {
    /// The wire/BCP-47-ish string of each locale, as used in
    /// `DESKTOP_NATIVE_LOCALES.includes(value)` and stored in the bundle's
    /// `locale` field.
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::Zh => "zh",
            Locale::Zht => "zht",
            Locale::Ko => "ko",
            Locale::De => "de",
            Locale::Es => "es",
            Locale::Fr => "fr",
            Locale::Da => "da",
            Locale::Ja => "ja",
            Locale::Pl => "pl",
            Locale::Ru => "ru",
            Locale::Uk => "uk",
            Locale::Ar => "ar",
            Locale::No => "no",
            Locale::Br => "br",
            Locale::Bs => "bs",
            Locale::Tr => "tr",
            Locale::Hi => "hi",
            Locale::Nl => "nl",
            Locale::Id => "id",
            Locale::Vi => "vi",
            Locale::It => "it",
            Locale::Ur => "ur",
            Locale::Pa => "pa",
            Locale::Az => "az",
            Locale::Fi => "fi",
            Locale::Sv => "sv",
            Locale::Th => "th",
            Locale::Am => "am",
            Locale::Bg => "bg",
            Locale::Bn => "bn",
            Locale::Ca => "ca",
            Locale::Cs => "cs",
            Locale::Dv => "dv",
            Locale::Dz => "dz",
            Locale::El => "el",
            Locale::Et => "et",
            Locale::Fa => "fa",
            Locale::Fo => "fo",
            Locale::Hr => "hr",
            Locale::Hu => "hu",
            Locale::Hy => "hy",
            Locale::Is => "is",
            Locale::Ka => "ka",
            Locale::Km => "km",
            Locale::Lo => "lo",
            Locale::Lt => "lt",
            Locale::Lv => "lv",
            Locale::Mk => "mk",
            Locale::Mn => "mn",
            Locale::Ms => "ms",
            Locale::My => "my",
            Locale::Ne => "ne",
            Locale::Ro => "ro",
            Locale::Si => "si",
            Locale::Sk => "sk",
            Locale::Sl => "sl",
            Locale::Sq => "sq",
            Locale::Sr => "sr",
            Locale::Tg => "tg",
            Locale::Tk => "tk",
            Locale::Uz => "uz",
        }
    }

    /// Mirrors `(DESKTOP_NATIVE_LOCALES as readonly string[]).includes(value)`.
    pub fn parse(value: &str) -> Option<Self> {
        DESKTOP_NATIVE_LOCALES
            .into_iter()
            .find(|locale| locale.as_str() == value)
    }

    /// The locale module's `ENTRIES` table, in the source's import order.
    pub fn entries(&self) -> &'static [(&'static str, &'static str)] {
        match self {
            Locale::En => en::ENTRIES,
            Locale::Zh => zh::ENTRIES,
            Locale::Zht => zht::ENTRIES,
            Locale::Ko => ko::ENTRIES,
            Locale::De => de::ENTRIES,
            Locale::Es => es::ENTRIES,
            Locale::Fr => fr::ENTRIES,
            Locale::Da => da::ENTRIES,
            Locale::Ja => ja::ENTRIES,
            Locale::Pl => pl::ENTRIES,
            Locale::Ru => ru::ENTRIES,
            Locale::Uk => uk::ENTRIES,
            Locale::Ar => ar::ENTRIES,
            Locale::No => no::ENTRIES,
            Locale::Br => br::ENTRIES,
            Locale::Bs => bs::ENTRIES,
            Locale::Tr => tr::ENTRIES,
            Locale::Hi => hi::ENTRIES,
            Locale::Nl => nl::ENTRIES,
            Locale::Id => id::ENTRIES,
            Locale::Vi => vi::ENTRIES,
            Locale::It => it::ENTRIES,
            Locale::Ur => ur::ENTRIES,
            Locale::Pa => pa::ENTRIES,
            Locale::Az => az::ENTRIES,
            Locale::Fi => fi::ENTRIES,
            Locale::Sv => sv::ENTRIES,
            Locale::Th => th::ENTRIES,
            Locale::Am => am::ENTRIES,
            Locale::Bg => bg::ENTRIES,
            Locale::Bn => bn::ENTRIES,
            Locale::Ca => ca::ENTRIES,
            Locale::Cs => cs::ENTRIES,
            Locale::Dv => dv::ENTRIES,
            Locale::Dz => dz::ENTRIES,
            Locale::El => el::ENTRIES,
            Locale::Et => et::ENTRIES,
            Locale::Fa => fa::ENTRIES,
            Locale::Fo => fo::ENTRIES,
            Locale::Hr => hr::ENTRIES,
            Locale::Hu => hu::ENTRIES,
            Locale::Hy => hy::ENTRIES,
            Locale::Is => is::ENTRIES,
            Locale::Ka => ka::ENTRIES,
            Locale::Km => km::ENTRIES,
            Locale::Lo => lo::ENTRIES,
            Locale::Lt => lt::ENTRIES,
            Locale::Lv => lv::ENTRIES,
            Locale::Mk => mk::ENTRIES,
            Locale::Mn => mn::ENTRIES,
            Locale::Ms => ms::ENTRIES,
            Locale::My => my::ENTRIES,
            Locale::Ne => ne::ENTRIES,
            Locale::Ro => ro::ENTRIES,
            Locale::Si => si::ENTRIES,
            Locale::Sk => sk::ENTRIES,
            Locale::Sl => sl::ENTRIES,
            Locale::Sq => sq::ENTRIES,
            Locale::Sr => sr::ENTRIES,
            Locale::Tg => tg::ENTRIES,
            Locale::Tk => tk::ENTRIES,
            Locale::Uz => uz::ENTRIES,
        }
    }
}

/// Mirrors `type Dictionary = Record<keyof i18n.Flatten<RawDictionary>, string>`.
pub type Dictionary = BTreeMap<String, String>;

/// Mirrors `i18n.flatten(dict)` over an already-flat table.
pub fn flatten(entries: &[(&str, &str)]) -> Dictionary {
    entries
        .iter()
        .map(|(key, value)| (key.to_string(), value.to_string()))
        .collect()
}

/// Mirrors `const base = i18n.flatten(desktopEn)`.
pub fn base() -> Dictionary {
    flatten(en::ENTRIES)
}

/// Mirrors `function build(locale): Dictionary`.
///
/// `en` short-circuits to `base`; every other locale is `{ ...base, ...locale }`
/// so a partial translation falls back to English. `ko` is the source's
/// fall-through default (it is imported but has no explicit `if` arm).
pub fn build(locale: Locale) -> Dictionary {
    if locale == Locale::En {
        return base();
    }
    let mut dict = base();
    dict.extend(flatten(locale.entries()));
    dict
}

/// Mirrors `function detectLocale(): Locale`.
///
/// The source returns `"en"` when `navigator` is not an object, otherwise it
/// runs `detectDesktopNativeLocale(navigator.languages?.length ? … )`.
///
/// PROVISIONAL(packages/app/src/i18n/desktop-native.ts):
/// `detectDesktopNativeLocale` lives in the app package (outside this lane's
/// scope). The reproduced rule is: take the first entry of the language
/// preference list, take its primary subtag, and match it against
/// [`DESKTOP_NATIVE_LOCALES`]; `en` when nothing matches.
pub fn detect_desktop_native_locale(languages: &[String]) -> Locale {
    for language in languages {
        let primary = language.split(['-', '_']).next().unwrap_or_default();
        if let Some(locale) = Locale::parse(primary) {
            return locale;
        }
    }
    Locale::En
}

/// Mirrors `detectLocale()` when a `navigator` stand-in is available.
pub fn detect_locale(navigator: Option<&[String]>) -> Locale {
    let Some(navigator) = navigator else {
        return Locale::En;
    };
    if navigator.is_empty() {
        return Locale::En;
    }
    detect_desktop_native_locale(navigator)
}

/// Mirrors `function parseLocale(value): Locale | null`.
pub fn parse_locale(value: Option<&str>) -> Option<Locale> {
    // `if (!value) return null` rejects `undefined` and the empty string.
    let value = value.filter(|value| !value.is_empty())?;
    Locale::parse(value)
}

/// Mirrors `function parseRecord(value)`.
pub fn parse_record(
    value: Option<&serde_json::Value>,
) -> Option<BTreeMap<String, serde_json::Value>> {
    let serde_json::Value::Object(record) = value? else {
        return None;
    };
    Some(record.clone())
}

/// Mirrors `function parseStored(value)`: a JSON string is parsed, anything
/// else (or a parse failure) is returned unchanged.
pub fn parse_stored(value: &str) -> serde_json::Value {
    match serde_json::from_str::<serde_json::Value>(value) {
        Ok(parsed) => parsed,
        Err(_) => serde_json::Value::String(value.to_string()),
    }
}

/// Mirrors `function pickLocale(value): Locale | null`.
pub fn pick_locale(value: Option<&serde_json::Value>) -> Option<Locale> {
    if let serde_json::Value::String(text) = value? {
        if let Some(locale) = parse_locale(Some(text)) {
            return Some(locale);
        }
    }
    let record = parse_record(value)?;
    let locale = record.get("locale").and_then(|value| value.as_str());
    parse_locale(locale)
}

/// Mirrors `i18n.resolveTemplate` — `{{name}}` substitution. A placeholder
/// with no matching param is left untouched, and the replacement is the
/// param's `String(...)` form.
pub fn resolve_template(template: &str, params: Option<&BTreeMap<String, String>>) -> String {
    crate::main::native_translations::format_desktop_native_message(template, params)
}

/// The `const state = { locale, dict, init }` object.
struct State {
    locale: Locale,
    dict: Dictionary,
    init: Option<Locale>,
}

impl State {
    fn new() -> Self {
        let locale = detect_locale(None);
        let dict = build(locale);
        Self {
            locale,
            dict,
            init: None,
        }
    }
}

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

/// The store the source reads the persisted language from.
pub const LANGUAGE_STORE: &str = "opencode.global.dat";
pub const LANGUAGE_KEY: &str = "language";

/// Mirrors `export function t(key, params?)`.
pub fn t(key: &str, params: Option<&BTreeMap<String, String>>) -> String {
    STATE.with(|cell| {
        let state = cell.borrow();
        match state.dict.get(key) {
            Some(template) => resolve_template(template, params),
            // `i18n.translator` returns the key itself for an unknown key.
            None => key.to_string(),
        }
    })
}

/// The `window.api.storeGet` result `initI18n` awaits.
///
/// PROVISIONAL(packages/desktop/src/renderer/i18n/index.ts): the source
/// awaits `window.api.storeGet("opencode.global.dat", "language")`, which is
/// an `ipcRenderer.invoke` round trip.
pub fn read_stored_language() -> Option<String> {
    unimplemented!("window.api.storeGet needs the Electron ipcRenderer bridge")
}

/// Mirrors `export function initI18n(): Promise<Locale>`.
///
/// The source memoises the in-flight promise in `state.init` and falls back
/// to `state.locale` on failure. Here the stored value is supplied directly
/// and the same memoisation/fallback ordering is preserved: the first call
/// applies the stored locale, later calls return the memoised locale without
/// re-reading the store.
pub fn init_i18n(stored: Option<&str>) -> Locale {
    STATE.with(|cell| {
        let mut state = cell.borrow_mut();
        if state.init.is_some() {
            return state.init.unwrap_or(state.locale);
        }
        let next = stored
            .map(parse_stored)
            .and_then(|value| pick_locale(Some(&value)))
            .unwrap_or(state.locale);
        state.locale = next;
        state.dict = build(next);
        state.init = Some(next);
        next
    })
}

/// Mirrors the `state.dict` read used by the source's `translate` closure.
pub fn current_locale() -> Locale {
    STATE.with(|cell| cell.borrow().locale)
}

#[cfg(test)]
mod tests {
    // The source ships no `i18n/index.test.ts`; these cover the 1:1
    // behaviour of `build()`'s fallback chain, `pickLocale`, and the
    // placeholder resolution the rest of the renderer depends on.
    use super::*;

    #[test]
    fn exposes_all_sixty_locales() {
        assert_eq!(DESKTOP_NATIVE_LOCALES.len(), 60);
        let mut names: Vec<&str> = DESKTOP_NATIVE_LOCALES
            .iter()
            .map(|locale| locale.as_str())
            .collect();
        let before = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(before, names.len(), "locale names must be unique");
        for locale in DESKTOP_NATIVE_LOCALES {
            assert!(
                !locale.entries().is_empty(),
                "{:?} must have entries",
                locale
            );
        }
    }

    #[test]
    fn english_short_circuits_to_the_base_table() {
        assert_eq!(build(Locale::En), base());
    }

    #[test]
    fn partial_translations_fall_back_to_english() {
        let dict = build(Locale::De);
        let base = base();
        // Every English key is still present…
        for key in base.keys() {
            assert!(dict.contains_key(key), "missing fallback key {}", key);
        }
        // …and the locale table is layered on top.
        let de = flatten(de::ENTRIES);
        for (key, value) in &de {
            assert_eq!(dict.get(key), Some(value));
        }
    }

    #[test]
    fn korean_is_the_fall_through_locale() {
        // The source has no `if (locale === "ko")` arm; ko is the default
        // return, so `build` must still layer it over the base table.
        let dict = build(Locale::Ko);
        for (key, value) in flatten(ko::ENTRIES) {
            assert_eq!(dict.get(&key), Some(&value));
        }
    }

    #[test]
    fn parses_locale_strings_like_the_source() {
        assert_eq!(parse_locale(Some("de")), Some(Locale::De));
        assert_eq!(
            parse_locale(Some("")),
            None,
            "`!value` rejects the empty string"
        );
        assert_eq!(parse_locale(None), None);
        assert_eq!(
            parse_locale(Some("xx")),
            None,
            "unknown locales are rejected"
        );
    }

    #[test]
    fn pick_locale_reads_a_bare_string_or_a_record() {
        assert_eq!(
            pick_locale(Some(&serde_json::json!("fr"))),
            Some(Locale::Fr)
        );
        assert_eq!(
            pick_locale(Some(&serde_json::json!({ "locale": "ja" }))),
            Some(Locale::Ja)
        );
        assert_eq!(
            pick_locale(Some(&serde_json::json!({ "other": "ja" }))),
            None
        );
        assert_eq!(
            pick_locale(Some(&serde_json::json!(["ja"]))),
            None,
            "arrays are rejected"
        );
        assert_eq!(pick_locale(Some(&serde_json::json!(null))), None);
    }

    #[test]
    fn parse_stored_falls_back_to_the_raw_string() {
        assert_eq!(parse_stored("\"fr\""), serde_json::json!("fr"));
        assert_eq!(
            parse_stored("{\"locale\":\"fr\"}"),
            serde_json::json!({ "locale": "fr" })
        );
        assert_eq!(
            parse_stored("fr"),
            serde_json::json!("fr"),
            "invalid JSON stays a string"
        );
    }

    #[test]
    fn detects_the_primary_subtag_of_the_first_matching_language() {
        assert_eq!(detect_locale(None), Locale::En, "no navigator means en");
        assert_eq!(
            detect_desktop_native_locale(&["de-DE".to_string()]),
            Locale::De
        );
        assert_eq!(
            detect_desktop_native_locale(&["pt-BR".to_string(), "it".to_string()]),
            Locale::It
        );
        assert_eq!(
            detect_desktop_native_locale(&["xx".to_string()]),
            Locale::En
        );
    }

    #[test]
    fn resolves_templates_with_placeholder_substitution() {
        let mut params = BTreeMap::new();
        params.insert("version".to_string(), "2.0.0".to_string());
        assert_eq!(
            resolve_template(
                "Version {{version}} of OpenCode has been downloaded",
                Some(&params)
            ),
            "Version 2.0.0 of OpenCode has been downloaded"
        );
        assert_eq!(resolve_template("no placeholders", None), "no placeholders");
        assert_eq!(
            resolve_template("keep {{unknown}} intact", Some(&params)),
            "keep {{unknown}} intact",
            "an unmatched placeholder is left untouched"
        );
    }
}
