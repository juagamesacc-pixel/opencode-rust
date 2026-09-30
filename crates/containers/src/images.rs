//! Rust port of `packages/containers/*/Dockerfile` (opencode v1.18.30).
//!
//! Source 5 Dockerfiles (base, bun-node, rust, tauri-linux, publish) — 1:1 verbatim.
//!
//! 1:1 notes:
//! - `base` FROM `ubuntu:24.04` with `DEBIAN_FRONTEND=noninteractive` and apt packages
//!   `build-essential ca-certificates curl git jq openssh-client pkg-config python3 unzip xz-utils zip`.
//! - `bun-node` FROM `${REGISTRY}/build/base:24.04`, BUN_VERSION from `packageManager bun@1.3.14`,
//!   NODE_VERSION `24.4.0`, arch branching `x64`/`arm64`, `curl nodejs.org` + `tar -xJf` + `corepack enable`,
//!   `curl bun.sh/install | bash -s -- "bun-v${BUN_VERSION}"`.
//! - `rust` FROM `${REGISTRY}/build/bun-node:24.04`, RUST_TOOLCHAIN `stable`,
//!   `CARGO_HOME=/opt/cargo RUSTUP_HOME=/opt/rustup`, `curl sh.rustup.rs | sh -s -- -y --profile minimal`.
//! - `tauri-linux` FROM `${REGISTRY}/build/rust:24.04` + apt `libappindicator3-dev libwebkit2gtk-4.1-dev librsvg2-dev patchelf`.
//! - `publish` FROM `${REGISTRY}/build/bun-node:24.04` + apt `docker.io pacman-package-manager`.

/// Image names verbatim from `build.ts` `images = ["base","bun-node","rust","tauri-linux","publish"]` order.
pub const IMAGE_NAMES: &[&str] = &["base", "bun-node", "rust", "tauri-linux", "publish"];

/// Default registry verbatim: `process.env.REGISTRY ?? "ghcr.io/anomalyco"`.
pub const DEFAULT_REGISTRY: &str = "ghcr.io/anomalyco";
/// Default tag verbatim: `process.env.TAG ?? "24.04"`.
pub const DEFAULT_TAG: &str = "24.04";

/// Dockerfile path template verbatim: `"packages/containers/{name}/Dockerfile"`.
pub fn dockerfile_path(name: &str) -> String {
    format!("packages/containers/{}/Dockerfile", name)
}

/// Image name template verbatim: `"{reg}/build/{name}:{tag}"`.
pub fn image_name(reg: &str, name: &str, tag: &str) -> String {
    format!("{}/build/{}:{}", reg, name, tag)
}

/// Platform verbatim from `build.ts`: `"linux/amd64,linux/arm64"`.
pub const PLATFORM: &str = "linux/amd64,linux/arm64";

/// Base image verbatim.
pub const BASE_FROM: &str = "ubuntu:24.04";

/// Debian frontend arg verbatim.
pub const DEBIAN_FRONTEND: &str = "noninteractive";

/// Base apt packages verbatim order.
pub const BASE_APT_PACKAGES: &[&str] = &[
    "build-essential",
    "ca-certificates",
    "curl",
    "git",
    "jq",
    "openssh-client",
    "pkg-config",
    "python3",
    "unzip",
    "xz-utils",
    "zip",
];

/// bun-node FROM verbatim (templated registry).
pub const BUN_NODE_BASE: &str = "${REGISTRY}/build/base:24.04";
/// Node version verbatim: `"24.4.0"`.
pub const NODE_VERSION: &str = "24.4.0";
/// Bun install env verbatim.
pub const BUN_INSTALL: &str = "/opt/bun";
/// PATH addition verbatim for bun-node/rust.
pub const BUN_PATH_PREFIX: &str = "/opt/bun/bin";

/// Rust FROM verbatim.
pub const RUST_BASE: &str = "${REGISTRY}/build/bun-node:24.04";
/// Rust toolchain default verbatim: `"stable"`.
pub const RUST_TOOLCHAIN_DEFAULT: &str = "stable";
pub const CARGO_HOME: &str = "/opt/cargo";
pub const RUSTUP_HOME: &str = "/opt/rustup";

/// Tauri-linux apt packages verbatim.
pub const TAURI_APT_PACKAGES: &[&str] = &[
    "libappindicator3-dev",
    "libwebkit2gtk-4.1-dev",
    "librsvg2-dev",
    "patchelf",
];

/// Publish apt packages verbatim.
pub const PUBLISH_APT_PACKAGES: &[&str] = &["docker.io", "pacman-package-manager"];

/// Rust port of a Dockerfile descriptor.
#[derive(Clone, Debug, PartialEq)]
pub struct Dockerfile {
    pub name: &'static str,
    pub from: &'static str,
}

pub const DOCKERFILES: &[Dockerfile] = &[
    Dockerfile {
        name: "base",
        from: BASE_FROM,
    },
    Dockerfile {
        name: "bun-node",
        from: BUN_NODE_BASE,
    },
    Dockerfile {
        name: "rust",
        from: RUST_BASE,
    },
    Dockerfile {
        name: "tauri-linux",
        from: RUST_BASE,
    },
    Dockerfile {
        name: "publish",
        from: BUN_NODE_BASE,
    },
];

/// Mirrors image descriptor `{ name, image, file }`.
#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    pub name: &'static str,
    pub file: &'static str,
}

pub const IMAGES: &[Image] = &[
    Image {
        name: "base",
        file: "packages/containers/base/Dockerfile",
    },
    Image {
        name: "bun-node",
        file: "packages/containers/bun-node/Dockerfile",
    },
    Image {
        name: "rust",
        file: "packages/containers/rust/Dockerfile",
    },
    Image {
        name: "tauri-linux",
        file: "packages/containers/tauri-linux/Dockerfile",
    },
    Image {
        name: "publish",
        file: "packages/containers/publish/Dockerfile",
    },
];
