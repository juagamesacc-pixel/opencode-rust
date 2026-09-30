// source: packages/web/astro.config.mjs
//
// Descriptor of the Astro/Starlight site configuration. Building or serving the site needs the
// Astro toolchain, so this module records the configuration verbatim:
// PROVISIONAL: Astro runtime, Starlight integration and Cloudflare adapter are not executed here.

/// Base path the site is served from.
pub const BASE: &str = "/docs";

/// Astro output mode.
pub const OUTPUT: &str = "server";

/// Site title shown in the Starlight header and tab titles.
pub const TITLE: &str = "OpenCode";

/// Default locale key.
pub const DEFAULT_LOCALE: &str = "root";

/// Favicon served from the site root.
pub const FAVICON: &str = "/favicon-v3.svg";

/// Starlight `lastUpdated` flag.
pub const LAST_UPDATED: bool = true;

/// `markdown.headingLinks` flag.
pub const HEADING_LINKS: bool = false;

/// Custom stylesheet entry.
pub const CUSTOM_CSS: &str = "./src/styles/custom.css";

/// Expressive-code color themes.
pub const EXPRESSIVE_CODE_THEMES: [&str; 2] = ["github-light", "github-dark"];

/// `markdown.rehypePlugins` entries.
pub const REHYPE_PLUGINS: [&str; 2] = ["rehypeHeadingIds", "rehype-autolink-headings"];

/// `rehype-autolink-headings` behavior option.
pub const AUTOLINK_HEADINGS_BEHAVIOR: &str = "wrap";

/// Astro integrations, in configuration order.
pub const INTEGRATIONS: [&str; 4] = [
    "configSchema",
    "solidJs",
    "starlight",
    "toolbeam-docs-theme",
];

/// Cloudflare adapter image service mode.
pub const IMAGE_SERVICE: &str = "passthrough";

/// Dev server bind address.
pub const SERVER_HOST: &str = "0.0.0.0";

/// Astro dev toolbar flag.
pub const DEV_TOOLBAR_ENABLED: bool = false;

/// Logo asset paths handed to Starlight.
pub const LOGO_LIGHT: &str = "./src/assets/logo-light.svg";

/// Logo asset paths handed to Starlight.
pub const LOGO_DARK: &str = "./src/assets/logo-dark.svg";

/// `logo.replacesTitle` flag.
pub const LOGO_REPLACES_TITLE: bool = true;

/// One Starlight `locales` entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StarlightLocale {
    /// Locale key used in URLs and in the docs tree.
    pub key: &'static str,
    /// Label shown in the language selector.
    pub label: &'static str,
    /// BCP-47 language tag.
    pub lang: &'static str,
    /// Text direction.
    pub dir: &'static str,
}

/// Starlight `locales`, in configuration order.
pub const STARLIGHT_LOCALES: [StarlightLocale; 18] = [
    StarlightLocale {
        key: "root",
        label: "English",
        lang: "en",
        dir: "ltr",
    },
    StarlightLocale {
        key: "ar",
        label: "العربية",
        lang: "ar",
        dir: "rtl",
    },
    StarlightLocale {
        key: "bs",
        label: "Bosanski",
        lang: "bs-BA",
        dir: "ltr",
    },
    StarlightLocale {
        key: "da",
        label: "Dansk",
        lang: "da-DK",
        dir: "ltr",
    },
    StarlightLocale {
        key: "de",
        label: "Deutsch",
        lang: "de-DE",
        dir: "ltr",
    },
    StarlightLocale {
        key: "es",
        label: "Español",
        lang: "es-ES",
        dir: "ltr",
    },
    StarlightLocale {
        key: "fr",
        label: "Français",
        lang: "fr-FR",
        dir: "ltr",
    },
    StarlightLocale {
        key: "it",
        label: "Italiano",
        lang: "it-IT",
        dir: "ltr",
    },
    StarlightLocale {
        key: "ja",
        label: "日本語",
        lang: "ja-JP",
        dir: "ltr",
    },
    StarlightLocale {
        key: "ko",
        label: "한국어",
        lang: "ko-KR",
        dir: "ltr",
    },
    StarlightLocale {
        key: "nb",
        label: "Norsk Bokmål",
        lang: "nb-NO",
        dir: "ltr",
    },
    StarlightLocale {
        key: "pl",
        label: "Polski",
        lang: "pl-PL",
        dir: "ltr",
    },
    StarlightLocale {
        key: "pt-br",
        label: "Português (Brasil)",
        lang: "pt-BR",
        dir: "ltr",
    },
    StarlightLocale {
        key: "ru",
        label: "Русский",
        lang: "ru-RU",
        dir: "ltr",
    },
    StarlightLocale {
        key: "th",
        label: "ไทย",
        lang: "th-TH",
        dir: "ltr",
    },
    StarlightLocale {
        key: "tr",
        label: "Türkçe",
        lang: "tr-TR",
        dir: "ltr",
    },
    StarlightLocale {
        key: "zh-cn",
        label: "简体中文",
        lang: "zh-CN",
        dir: "ltr",
    },
    StarlightLocale {
        key: "zh-tw",
        label: "繁體中文",
        lang: "zh-TW",
        dir: "ltr",
    },
];

/// One `head` icon link.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeadIcon {
    /// Link relation (`icon` or `apple-touch-icon`).
    pub rel: &'static str,
    /// Icon URL.
    pub href: &'static str,
    /// Icon sizes attribute.
    pub sizes: &'static str,
    /// Optional MIME type.
    pub mime: Option<&'static str>,
}

/// Starlight `head` icon links, in configuration order.
pub const HEAD_ICONS: [HeadIcon; 3] = [
    HeadIcon {
        rel: "icon",
        href: "/favicon-v3.ico",
        sizes: "32x32",
        mime: None,
    },
    HeadIcon {
        rel: "icon",
        href: "/favicon-96x96-v3.png",
        sizes: "96x96",
        mime: Some("image/png"),
    },
    HeadIcon {
        rel: "apple-touch-icon",
        href: "/apple-touch-icon-v3.png",
        sizes: "180x180",
        mime: None,
    },
];

/// Starlight `social` entries; `href` values resolve from [`crate::config::Config`] at runtime.
pub const SOCIAL: [(&str, &str); 2] = [("github", "GitHub"), ("discord", "Discord")];

/// A translated sidebar group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SidebarGroup {
    /// Default (English) label.
    pub label: &'static str,
    /// `(lang-tag, label)` translations, verbatim.
    pub translations: &'static [(&'static str, &'static str)],
    /// Optional single link target instead of `items`.
    pub link: Option<&'static str>,
    /// Doc slugs in the group.
    pub items: &'static [&'static str],
}

/// A Starlight sidebar entry: either a bare slug or a group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarEntry {
    /// Bare doc slug; `""` is the index page.
    Link(&'static str),
    /// Labelled group.
    Group(SidebarGroup),
}

/// Starlight `sidebar`, in configuration order.
pub const SIDEBAR: [SidebarEntry; 10] = [
    SidebarEntry::Link(""),
    SidebarEntry::Link("config"),
    SidebarEntry::Link("providers"),
    SidebarEntry::Link("network"),
    SidebarEntry::Link("enterprise"),
    SidebarEntry::Link("troubleshooting"),
    SidebarEntry::Group(SidebarGroup {
        label: "Windows",
        translations: &[
            ("en", "Windows"),
            ("ar", "Windows"),
            ("bs-BA", "Windows"),
            ("da-DK", "Windows"),
            ("de-DE", "Windows"),
            ("es-ES", "Windows"),
            ("fr-FR", "Windows"),
            ("it-IT", "Windows"),
            ("ja-JP", "Windows"),
            ("ko-KR", "Windows"),
            ("nb-NO", "Windows"),
            ("pl-PL", "Windows"),
            ("pt-BR", "Windows"),
            ("ru-RU", "Windows"),
            ("th-TH", "Windows"),
            ("tr-TR", "Windows"),
            ("zh-CN", "Windows"),
            ("zh-TW", "Windows"),
        ],
        link: Some("windows-wsl"),
        items: &[],
    }),
    SidebarEntry::Group(SidebarGroup {
        label: "Usage",
        translations: &[
            ("en", "Usage"),
            ("ar", "الاستخدام"),
            ("bs-BA", "Korištenje"),
            ("da-DK", "Brug"),
            ("de-DE", "Nutzung"),
            ("es-ES", "Uso"),
            ("fr-FR", "Utilisation"),
            ("it-IT", "Utilizzo"),
            ("ja-JP", "使い方"),
            ("ko-KR", "사용"),
            ("nb-NO", "Bruk"),
            ("pl-PL", "Użycie"),
            ("pt-BR", "Uso"),
            ("ru-RU", "Использование"),
            ("th-TH", "การใช้งาน"),
            ("tr-TR", "Kullanım"),
            ("zh-CN", "使用"),
            ("zh-TW", "使用"),
        ],
        link: None,
        items: &[
            "go", "tui", "cli", "web", "ide", "zen", "share", "github", "gitlab",
        ],
    }),
    SidebarEntry::Group(SidebarGroup {
        label: "Configure",
        translations: &[
            ("en", "Configure"),
            ("ar", "الإعداد"),
            ("bs-BA", "Podešavanje"),
            ("da-DK", "Konfiguration"),
            ("de-DE", "Konfiguration"),
            ("es-ES", "Configuración"),
            ("fr-FR", "Configuration"),
            ("it-IT", "Configurazione"),
            ("ja-JP", "設定"),
            ("ko-KR", "구성"),
            ("nb-NO", "Konfigurasjon"),
            ("pl-PL", "Konfiguracja"),
            ("pt-BR", "Configuração"),
            ("ru-RU", "Настройка"),
            ("th-TH", "การกำหนดค่า"),
            ("tr-TR", "Yapılandırma"),
            ("zh-CN", "配置"),
            ("zh-TW", "設定"),
        ],
        link: None,
        items: &[
            "tools",
            "rules",
            "agents",
            "models",
            "themes",
            "keybinds",
            "commands",
            "formatters",
            "permissions",
            "policies",
            "lsp",
            "mcp-servers",
            "acp",
            "skills",
            "references",
            "custom-tools",
        ],
    }),
    SidebarEntry::Group(SidebarGroup {
        label: "Develop",
        translations: &[
            ("en", "Develop"),
            ("ar", "التطوير"),
            ("bs-BA", "Razvoj"),
            ("da-DK", "Udvikling"),
            ("de-DE", "Entwicklung"),
            ("es-ES", "Desarrollo"),
            ("fr-FR", "Développement"),
            ("it-IT", "Sviluppo"),
            ("ja-JP", "開発"),
            ("ko-KR", "개발"),
            ("nb-NO", "Utvikling"),
            ("pl-PL", "Rozwój"),
            ("pt-BR", "Desenvolvimento"),
            ("ru-RU", "Разработка"),
            ("th-TH", "การพัฒนา"),
            ("tr-TR", "Geliştirme"),
            ("zh-CN", "开发"),
            ("zh-TW", "開發"),
        ],
        link: None,
        items: &["sdk", "server", "plugins", "ecosystem"],
    }),
];

/// `true` when `slug` appears anywhere in the sidebar (bare link, group link or group item).
pub fn sidebar_contains(slug: &str) -> bool {
    SIDEBAR.iter().any(|entry| match entry {
        SidebarEntry::Link(target) => *target == slug,
        SidebarEntry::Group(group) => group.link == Some(slug) || group.items.contains(&slug),
    })
}

/// Look up a Starlight locale entry by key.
pub fn starlight_locale(key: &str) -> Option<&'static StarlightLocale> {
    STARLIGHT_LOCALES.iter().find(|entry| entry.key == key)
}
