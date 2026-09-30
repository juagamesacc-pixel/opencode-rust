//! Rust port of `packages/ui/src/i18n/*` (opencode v1.18.30).
//!
//! 1:1 — every locale file in the source package has a verbatim string-table module.
//! Locale files ported: 62.

#![allow(dead_code)]

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

/// All locale modules, keyed by locale code (source file order preserved).
pub const LOCALES: &[&str] = &[
    "am", "ar", "az", "bg", "bn", "br", "bs", "ca", "cs", "da", "de", "dv", "dz", "el", "en", "es",
    "et", "fa", "fi", "fo", "fr", "hi", "hr", "hu", "hy", "id", "is", "it", "ja", "ka", "km", "ko",
    "lo", "lt", "lv", "mk", "mn", "ms", "my", "ne", "nl", "no", "pa", "pl", "ro", "ru", "si", "sk",
    "sl", "sq", "sr", "sv", "tg", "th", "tk", "tr", "uk", "ur", "uz", "vi", "zh", "zht",
];

/// Entry count per locale code (source file order preserved).
pub const LOCALE_ENTRY_COUNTS: &[(&str, usize)] = &[
    ("am", 193),
    ("ar", 209),
    ("az", 193),
    ("bg", 193),
    ("bn", 193),
    ("br", 197),
    ("bs", 197),
    ("ca", 197),
    ("cs", 201),
    ("da", 193),
    ("de", 193),
    ("dv", 193),
    ("dz", 193),
    ("el", 193),
    ("en", 193),
    ("es", 197),
    ("et", 193),
    ("fa", 193),
    ("fi", 193),
    ("fo", 193),
    ("fr", 197),
    ("hi", 193),
    ("hr", 197),
    ("hu", 193),
    ("hy", 193),
    ("id", 193),
    ("is", 193),
    ("it", 197),
    ("ja", 193),
    ("ka", 193),
    ("km", 193),
    ("ko", 193),
    ("lo", 193),
    ("lt", 201),
    ("lv", 197),
    ("mk", 193),
    ("mn", 193),
    ("ms", 193),
    ("my", 193),
    ("ne", 193),
    ("nl", 193),
    ("no", 193),
    ("pa", 193),
    ("pl", 201),
    ("ro", 197),
    ("ru", 201),
    ("si", 193),
    ("sk", 201),
    ("sl", 201),
    ("sq", 193),
    ("sr", 197),
    ("sv", 193),
    ("tg", 193),
    ("th", 193),
    ("tk", 193),
    ("tr", 193),
    ("uk", 201),
    ("ur", 193),
    ("uz", 193),
    ("vi", 193),
    ("zh", 193),
    ("zht", 193),
];

/// Total number of dict entries across every ported locale.
pub fn total_entry_count() -> usize {
    LOCALES.iter().map(|l| entry_count(l)).sum()
}

/// Entry count for `locale`, or 0 when the locale is unknown.
pub fn entry_count(locale: &str) -> usize {
    match locale {
        "am" => 193,
        "ar" => 209,
        "az" => 193,
        "bg" => 193,
        "bn" => 193,
        "br" => 197,
        "bs" => 197,
        "ca" => 197,
        "cs" => 201,
        "da" => 193,
        "de" => 193,
        "dv" => 193,
        "dz" => 193,
        "el" => 193,
        "en" => 193,
        "es" => 197,
        "et" => 193,
        "fa" => 193,
        "fi" => 193,
        "fo" => 193,
        "fr" => 197,
        "hi" => 193,
        "hr" => 197,
        "hu" => 193,
        "hy" => 193,
        "id" => 193,
        "is" => 193,
        "it" => 197,
        "ja" => 193,
        "ka" => 193,
        "km" => 193,
        "ko" => 193,
        "lo" => 193,
        "lt" => 201,
        "lv" => 197,
        "mk" => 193,
        "mn" => 193,
        "ms" => 193,
        "my" => 193,
        "ne" => 193,
        "nl" => 193,
        "no" => 193,
        "pa" => 193,
        "pl" => 201,
        "ro" => 197,
        "ru" => 201,
        "si" => 193,
        "sk" => 201,
        "sl" => 201,
        "sq" => 193,
        "sr" => 197,
        "sv" => 193,
        "tg" => 193,
        "th" => 193,
        "tk" => 193,
        "tr" => 193,
        "uk" => 201,
        "ur" => 193,
        "uz" => 193,
        "vi" => 193,
        "zh" => 193,
        "zht" => 193,
        _ => 0,
    }
}

/// Every ported dict entry for `locale` (empty slice when unknown).
pub fn dict_entries(locale: &str) -> &'static [(&'static str, &'static str)] {
    match locale {
        "am" => am::DICT_ENTRIES,
        "ar" => ar::DICT_ENTRIES,
        "az" => az::DICT_ENTRIES,
        "bg" => bg::DICT_ENTRIES,
        "bn" => bn::DICT_ENTRIES,
        "br" => br::DICT_ENTRIES,
        "bs" => bs::DICT_ENTRIES,
        "ca" => ca::DICT_ENTRIES,
        "cs" => cs::DICT_ENTRIES,
        "da" => da::DICT_ENTRIES,
        "de" => de::DICT_ENTRIES,
        "dv" => dv::DICT_ENTRIES,
        "dz" => dz::DICT_ENTRIES,
        "el" => el::DICT_ENTRIES,
        "en" => en::DICT_ENTRIES,
        "es" => es::DICT_ENTRIES,
        "et" => et::DICT_ENTRIES,
        "fa" => fa::DICT_ENTRIES,
        "fi" => fi::DICT_ENTRIES,
        "fo" => fo::DICT_ENTRIES,
        "fr" => fr::DICT_ENTRIES,
        "hi" => hi::DICT_ENTRIES,
        "hr" => hr::DICT_ENTRIES,
        "hu" => hu::DICT_ENTRIES,
        "hy" => hy::DICT_ENTRIES,
        "id" => id::DICT_ENTRIES,
        "is" => is::DICT_ENTRIES,
        "it" => it::DICT_ENTRIES,
        "ja" => ja::DICT_ENTRIES,
        "ka" => ka::DICT_ENTRIES,
        "km" => km::DICT_ENTRIES,
        "ko" => ko::DICT_ENTRIES,
        "lo" => lo::DICT_ENTRIES,
        "lt" => lt::DICT_ENTRIES,
        "lv" => lv::DICT_ENTRIES,
        "mk" => mk::DICT_ENTRIES,
        "mn" => mn::DICT_ENTRIES,
        "ms" => ms::DICT_ENTRIES,
        "my" => my::DICT_ENTRIES,
        "ne" => ne::DICT_ENTRIES,
        "nl" => nl::DICT_ENTRIES,
        "no" => no::DICT_ENTRIES,
        "pa" => pa::DICT_ENTRIES,
        "pl" => pl::DICT_ENTRIES,
        "ro" => ro::DICT_ENTRIES,
        "ru" => ru::DICT_ENTRIES,
        "si" => si::DICT_ENTRIES,
        "sk" => sk::DICT_ENTRIES,
        "sl" => sl::DICT_ENTRIES,
        "sq" => sq::DICT_ENTRIES,
        "sr" => sr::DICT_ENTRIES,
        "sv" => sv::DICT_ENTRIES,
        "tg" => tg::DICT_ENTRIES,
        "th" => th::DICT_ENTRIES,
        "tk" => tk::DICT_ENTRIES,
        "tr" => tr::DICT_ENTRIES,
        "uk" => uk::DICT_ENTRIES,
        "ur" => ur::DICT_ENTRIES,
        "uz" => uz::DICT_ENTRIES,
        "vi" => vi::DICT_ENTRIES,
        "zh" => zh::DICT_ENTRIES,
        "zht" => zht::DICT_ENTRIES,
        _ => &[],
    }
}

/// Look up `key` in `locale`, mirroring the source partial-dict merge order.
pub fn lookup(locale: &str, key: &str) -> Option<&'static str> {
    dict_entries(locale)
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
}
