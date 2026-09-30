//! Rust port of `packages/containers/script/build.ts` (opencode v1.18.30).
//!
//! Source 77 lines. Exports: build orchestration for 5 images.
//!
//! 1:1 notes:
//! - `rootDir = fileURLToPath(new URL("../../..", import.meta.url))` → `process.chdir(rootDir)`
//!   — chdir to repo root before reading `package.json`.
//! - `REGISTRY = process.env.REGISTRY ?? "ghcr.io/anomalyco"`; `TAG = process.env.TAG ?? "24.04"`;
//!   `PUSH = argv.includes("--push") || process.env.PUSH === "1"`.
//! - `packageManager` must be `bun@<version>` else throw `"packageManager must be bun@<version>"`.
//! - `images = ["base","bun-node","rust","tauri-linux","publish"]` order verbatim.
//! - `setup()` if `push`: `docker buildx ls` contains `"opencode"` → `docker buildx use opencode` else `create --name opencode --use`.
//! - `platform = "linux/amd64,linux/arm64"`.
//! - Per-image build args:
//!   - `base`: `docker buildx build --platform {platform} -f {file} -t {image} --push .` if push else `docker build -f {file} -t {image} .`
//!   - `bun-node`: adds `--build-arg REGISTRY={reg} --build-arg BUN_VERSION={bun}`.
//!   - others (`rust`, `tauri-linux`, `publish`): adds `--build-arg REGISTRY={reg}`.
//! - Logs `pushed {image}` if push.
//!
//! PROVISIONAL: `docker`/`docker buildx`/`$` (Bun shell) are host-provided — descriptor only.

use crate::images;

/// Default registry/tag/platform verbatim.
pub const DEFAULT_REGISTRY: &str = "ghcr.io/anomalyco";
pub const DEFAULT_TAG: &str = "24.04";
pub const PLATFORM: &str = "linux/amd64,linux/arm64";

/// Error string verbatim: `"packageManager must be bun@<version>"`.
pub const ERR_BUN_PACKAGE_MANAGER: &str = "packageManager must be bun@<version>";

/// Build fix — verbatim values for `buildx` setup.
pub const BUILDX_NAME: &str = "opencode";
pub const BUILDX_LS: &str = "docker buildx ls";
pub const BUILDX_USE: &str = "docker buildx use opencode";
pub const BUILDX_CREATE: &str = "docker buildx create --name opencode --use";

/// Mirrors `process.env` mapping for registry/tag/push.
#[derive(Clone, Debug, PartialEq)]
pub struct BuildConfig {
    pub registry: String,
    pub tag: String,
    pub push: bool,
}

impl BuildConfig {
    pub fn from_env(registry: Option<String>, tag: Option<String>, push: bool) -> Self {
        Self {
            registry: registry.unwrap_or_else(|| DEFAULT_REGISTRY.to_string()),
            tag: tag.unwrap_or_else(|| DEFAULT_TAG.to_string()),
            push,
        }
    }
}

/// Port of `BunVersion` extraction — `pkg.packageManager` must start with `bun@`.
pub fn parse_bun_version(package_manager: &str) -> Result<String, String> {
    if let Some(stripped) = package_manager.strip_prefix("bun@") {
        if stripped.is_empty() {
            return Err(ERR_BUN_PACKAGE_MANAGER.to_string());
        }
        Ok(stripped.to_string())
    } else {
        Err(ERR_BUN_PACKAGE_MANAGER.to_string())
    }
}

/// Port of `setup()` decision — returns which command would be run.
#[derive(Clone, Debug, PartialEq)]
pub enum SetupAction {
    UseExisting,
    CreateNew,
    Skip,
}

pub fn setup_action(push: bool, buildx_ls_output: Option<&str>) -> SetupAction {
    if !push {
        return SetupAction::Skip;
    }
    match buildx_ls_output {
        Some(out) if out.contains(BUILDX_NAME) => SetupAction::UseExisting,
        _ => SetupAction::CreateNew,
    }
}

/// Port of per-image docker command construction.
#[derive(Clone, Debug, PartialEq)]
pub struct BuildCommand {
    pub image: String,
    pub file: String,
    pub command: String,
}

pub fn build_commands(config: &BuildConfig, bun_version: &str) -> Vec<BuildCommand> {
    let mut out = Vec::with_capacity(images::IMAGE_NAMES.len());
    for name in images::IMAGE_NAMES {
        let image = images::image_name(&config.registry, name, &config.tag);
        let file = images::dockerfile_path(name);
        let command = if *name == "base" {
            if config.push {
                format!(
                    "docker buildx build --platform {} -f {} -t {} --push .",
                    PLATFORM, file, image
                )
            } else {
                format!("docker build -f {} -t {} .", file, image)
            }
        } else if *name == "bun-node" {
            if config.push {
                format!(
                    "docker buildx build --platform {} -f {} -t {} --build-arg REGISTRY={} --build-arg BUN_VERSION={} --push .",
                    PLATFORM, file, image, config.registry, bun_version
                )
            } else {
                format!(
                    "docker build -f {} -t {} --build-arg REGISTRY={} --build-arg BUN_VERSION={} .",
                    file, image, config.registry, bun_version
                )
            }
        } else if config.push {
            format!(
                "docker buildx build --platform {} -f {} -t {} --build-arg REGISTRY={} --push .",
                PLATFORM, file, image, config.registry
            )
        } else {
            format!(
                "docker build -f {} -t {} --build-arg REGISTRY={} .",
                file, image, config.registry
            )
        };
        out.push(BuildCommand {
            image,
            file,
            command,
        });
    }
    out
}

/// Mirrors `BuildScript` descriptor (no runtime docker exec — pure descriptor).
#[derive(Clone, Debug, PartialEq)]
pub struct BuildScript;

impl BuildScript {
    pub const IMAGES: &'static [&'static str] = images::IMAGE_NAMES;
    pub const PLATFORM: &'static str = PLATFORM;
}
