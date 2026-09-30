// source: packages/web/src/assets, packages/web/src/styles, packages/web/public
//
// SVGs, PNGs, ICOs, the webmanifest, robots.txt and the custom stylesheet are copied
// byte-for-byte into `assets/`, `styles/` and `public/` next to this crate.

/// A copied static asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Asset {
    /// Path relative to the crate root, e.g. `assets/logo-light.svg`.
    pub path: &'static str,
    /// Path relative to its root directory, e.g. `logo-light.svg`.
    pub rel: &'static str,
    /// Size of the copied file in bytes.
    pub bytes: usize,
}

/// Every file under `src/assets`, copied to `assets/`.
pub const ASSETS: [Asset; 13] = [
    Asset {
        path: "assets/lander/check.svg",
        rel: "lander/check.svg",
        bytes: 212,
    },
    Asset {
        path: "assets/lander/copy.svg",
        rel: "lander/copy.svg",
        bytes: 443,
    },
    Asset {
        path: "assets/lander/screenshot-github.png",
        rel: "lander/screenshot-github.png",
        bytes: 924094,
    },
    Asset {
        path: "assets/lander/screenshot-splash.png",
        rel: "lander/screenshot-splash.png",
        bytes: 369103,
    },
    Asset {
        path: "assets/lander/screenshot-vscode.png",
        rel: "lander/screenshot-vscode.png",
        bytes: 1022418,
    },
    Asset {
        path: "assets/lander/screenshot.png",
        rel: "lander/screenshot.png",
        bytes: 470646,
    },
    Asset {
        path: "assets/logo-dark.svg",
        rel: "logo-dark.svg",
        bytes: 1077,
    },
    Asset {
        path: "assets/logo-light.svg",
        rel: "logo-light.svg",
        bytes: 1077,
    },
    Asset {
        path: "assets/logo-ornate-dark.svg",
        rel: "logo-ornate-dark.svg",
        bytes: 1077,
    },
    Asset {
        path: "assets/logo-ornate-light.svg",
        rel: "logo-ornate-light.svg",
        bytes: 1077,
    },
    Asset {
        path: "assets/web/web-homepage-active-session.png",
        rel: "web/web-homepage-active-session.png",
        bytes: 747629,
    },
    Asset {
        path: "assets/web/web-homepage-new-session.png",
        rel: "web/web-homepage-new-session.png",
        bytes: 623348,
    },
    Asset {
        path: "assets/web/web-homepage-see-servers.png",
        rel: "web/web-homepage-see-servers.png",
        bytes: 680136,
    },
];

/// Every file under `public`, copied to `public/` (served at the site root).
pub const PUBLIC: [Asset; 15] = [
    Asset {
        path: "public/apple-touch-icon-v3.png",
        rel: "apple-touch-icon-v3.png",
        bytes: 1541,
    },
    Asset {
        path: "public/apple-touch-icon.png",
        rel: "apple-touch-icon.png",
        bytes: 1541,
    },
    Asset {
        path: "public/favicon-96x96-v3.png",
        rel: "favicon-96x96-v3.png",
        bytes: 536,
    },
    Asset {
        path: "public/favicon-96x96.png",
        rel: "favicon-96x96.png",
        bytes: 536,
    },
    Asset {
        path: "public/favicon-v3.ico",
        rel: "favicon-v3.ico",
        bytes: 15086,
    },
    Asset {
        path: "public/favicon-v3.svg",
        rel: "favicon-v3.svg",
        bytes: 612,
    },
    Asset {
        path: "public/favicon.ico",
        rel: "favicon.ico",
        bytes: 15086,
    },
    Asset {
        path: "public/favicon.svg",
        rel: "favicon.svg",
        bytes: 612,
    },
    Asset {
        path: "public/robots.txt",
        rel: "robots.txt",
        bytes: 87,
    },
    Asset {
        path: "public/site.webmanifest",
        rel: "site.webmanifest",
        bytes: 487,
    },
    Asset {
        path: "public/social-share-zen.png",
        rel: "social-share-zen.png",
        bytes: 21918,
    },
    Asset {
        path: "public/social-share.png",
        rel: "social-share.png",
        bytes: 14238,
    },
    Asset {
        path: "public/theme.json",
        rel: "theme.json",
        bytes: 7274,
    },
    Asset {
        path: "public/web-app-manifest-192x192.png",
        rel: "web-app-manifest-192x192.png",
        bytes: 1601,
    },
    Asset {
        path: "public/web-app-manifest-512x512.png",
        rel: "web-app-manifest-512x512.png",
        bytes: 7194,
    },
];

/// Every file under `src/styles`, copied to `styles/`.
pub const STYLES: [Asset; 1] = [Asset {
    path: "styles/custom.css",
    rel: "custom.css",
    bytes: 9171,
}];

/// Look up a copied asset by its crate-relative path.
pub fn find(path: &str) -> Option<&'static Asset> {
    ASSETS
        .iter()
        .chain(PUBLIC.iter())
        .chain(STYLES.iter())
        .find(|asset| asset.path == path)
}

/// Total number of copied static files.
pub const TOTAL_FILES: usize = ASSETS.len() + PUBLIC.len() + STYLES.len();
