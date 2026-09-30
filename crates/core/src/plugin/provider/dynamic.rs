//! Rust port of `packages/core/src/plugin/provider/dynamic.ts`.

pub const ID: &str = "dynamic-provider";

pub fn package_requires_install(package: &str) -> bool {
    !package.starts_with("file://")
}

pub fn entrypoint_error(package: &str) -> String {
    format!("Package {package} has no import entrypoint")
}

pub fn no_factory_error(package: &str) -> String {
    format!("Package {package} has no provider factory export")
}

pub fn resolve_installed_path(package: &str, entrypoint: Option<&str>) -> Result<String, String> {
    if package.starts_with("file://") {
        return Ok(package.to_string());
    }
    entrypoint
        .map(|s| s.to_string())
        .ok_or_else(|| entrypoint_error(package))
}
