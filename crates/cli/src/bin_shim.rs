//! Port of `packages/cli/bin/lildax.cjs` (v1.18.30 @3104c14).
//!
//! The `lildax` bin shim: resolves the platform-specific compiled binary and
//! `spawn`s it with inherited stdio, forwarding signals and propagating the
//! exit code/signal. This module is both the ported logic AND the bin-shim
//! note: the Rust `[[bin]] name = "lildax"` entrypoint (`main.rs`, port of
//! `src/index.ts`) is what the resolved binary runs; when this crate is
//! published per-platform, the shim logic below selects the artifact.
//!
//! 1:1 notes (every literal preserved):
//! - `forwardedSignals = ["SIGINT", "SIGTERM", "SIGHUP"]`; forwarders added
//!   before spawn-exit, removed on child exit.
//! - Child `error` -> `console.error(message)` + `exit(1)`.
//! - Child `exit(code, signal)`: `signal` -> re-kill self with the signal;
//!   else `exit(code ?? 0)`.
//! - `OPENCODE_BIN_PATH` env override; `.lildax` cache next to the script;
//!   `platform` map `{ darwin: "darwin", linux: "linux", win32: "windows" }`;
//!   `arch` map `{ x64: "x64", arm64: "arm64", arm: "arm" }`;
//!   `base = "@opencode-ai/cli-" + platform + "-" + arch`;
//!   windows binary `"lildax.exe"` else `"lildax"`.
//! - AVX2 baseline probing: x64-only; linux reads `/proc/cpuinfo` for the
//!   `avx2` token; darwin `sysctl -n hw.optional.avx2_0 === "1"` (1.5 s
//!   timeout); windows `IsProcessorFeaturePresent(40)` via
//!   powershell/pwsh (3 s timeout); default `false`.
//! - Linux musl detection: `/etc/alpine-release` exists OR `ldd --version`
//!   mentions musl; candidate orders (baseline/non-baseline x musl/glibc)
//!   are verbatim from source lines 91-101; non-linux: `[base,
//!   base-baseline]` (or baseline-first when no AVX2).
//! - `findBinary(startDir)`: walk up; at each level check
//!   `<level>/node_modules/<name>/bin/<binary>` for each name in order.
//! - Missing binary error (verbatim):
//!   `"It seems that your package manager failed to install the right lildax
//!   CLI package. Try manually installing " + names.map(n => `"${n}"`).join("
//!   or ") + " package"` + `exit(1)`.

use std::path::{Path, PathBuf};

/// Port of `forwardedSignals` (line 8).
pub const FORWARDED_SIGNALS: [&str; 3] = ["SIGINT", "SIGTERM", "SIGHUP"];

/// Port of the platform map (line 35).
pub fn platform_id(os: &str) -> String {
    match os {
        "darwin" => "darwin".to_string(),
        "linux" => "linux".to_string(),
        "win32" => "windows".to_string(),
        other => other.to_string(),
    }
}

/// Port of the arch map (line 36).
pub fn arch_id(arch: &str) -> String {
    match arch {
        "x64" => "x64".to_string(),
        "arm64" => "arm64".to_string(),
        "arm" => "arm".to_string(),
        other => other.to_string(),
    }
}

/// Port of `base` (line 37).
pub fn package_base(platform: &str, arch: &str) -> String {
    format!("@opencode-ai/cli-{platform}-{arch}")
}

/// Port of `binary` (line 38).
pub fn binary_name(platform: &str) -> &str {
    if platform == "windows" {
        "lildax.exe"
    } else {
        "lildax"
    }
}

/// Port of `supportsAvx2()` (lines 40-77). `cpuinfo`, `sysctl_stdout`, and
/// the powershell runner are injected so the pure ordering logic stays
/// testable without spawning.
pub fn supports_avx2(
    arch: &str,
    platform: &str,
    cpuinfo: Option<&str>,
    sysctl_stdout: Option<&str>,
    sysctl_status: i32,
    powershell_outputs: &[Option<String>],
) -> bool {
    if arch != "x64" {
        return false;
    }
    if platform == "linux" {
        return match cpuinfo {
            Some(text) => {
                let lower = text.to_lowercase();
                lower.split_whitespace().any(|token| token == "avx2")
            }
            None => false,
        };
    }
    if platform == "darwin" {
        return sysctl_status == 0 && sysctl_stdout.unwrap_or("").trim() == "1";
    }
    if platform == "windows" {
        for output in powershell_outputs {
            let Some(text) = output else { continue };
            match text.trim().to_lowercase().as_str() {
                "true" | "1" => return true,
                "false" | "0" => return false,
                _ => continue,
            }
        }
    }
    false
}

/// Port of the `names` IIFE (lines 79-104).
pub fn candidate_names(platform: &str, arch: &str, baseline: bool, musl: bool) -> Vec<String> {
    let base = package_base(platform, arch);
    if platform == "linux" {
        if musl {
            if arch == "x64" {
                return if baseline {
                    vec![
                        format!("{base}-baseline-musl"),
                        format!("{base}-musl"),
                        format!("{base}-baseline"),
                        base.clone(),
                    ]
                } else {
                    vec![
                        format!("{base}-musl"),
                        format!("{base}-baseline-musl"),
                        base.clone(),
                        format!("{base}-baseline"),
                    ]
                };
            }
            return vec![format!("{base}-musl"), base];
        }
        if arch == "x64" {
            return if baseline {
                vec![
                    format!("{base}-baseline"),
                    base.clone(),
                    format!("{base}-baseline-musl"),
                    format!("{base}-musl"),
                ]
            } else {
                vec![
                    base.clone(),
                    format!("{base}-baseline"),
                    format!("{base}-musl"),
                    format!("{base}-baseline-musl"),
                ]
            };
        }
        return vec![base.clone(), format!("{base}-musl")];
    }
    if arch == "x64" {
        return if baseline {
            vec![format!("{base}-baseline"), base.clone()]
        } else {
            vec![base.clone(), format!("{base}-baseline")]
        };
    }
    vec![base]
}

/// Port of `findBinary(startDir)` (lines 106-119): walk up checking
/// `<level>/node_modules/<name>/bin/<binary>`.
pub fn find_binary(
    start_dir: &Path,
    names: &[String],
    binary: &str,
    exists: &dyn Fn(&Path) -> bool,
) -> Option<PathBuf> {
    let mut current = start_dir.to_path_buf();
    loop {
        let modules = current.join("node_modules");
        if exists(&modules) {
            for name in names {
                let candidate = modules.join(name).join("bin").join(binary);
                if exists(&candidate) {
                    return Some(candidate);
                }
            }
        }
        match current.parent() {
            Some(parent) if parent != current => current = parent.to_path_buf(),
            _ => return None,
        }
    }
}

/// Port of the resolution (line 121):
/// `OPENCODE_BIN_PATH || (.lildax cache if present) || findBinary(scriptDir)`.
pub fn resolve_binary(
    env_path: Option<&str>,
    cached: &Path,
    cached_exists: bool,
    script_dir: &Path,
    names: &[String],
    binary: &str,
    exists: &dyn Fn(&Path) -> bool,
) -> Option<PathBuf> {
    if let Some(path) = env_path.filter(|p| !p.is_empty()) {
        return Some(PathBuf::from(path));
    }
    if cached_exists {
        return Some(cached.to_path_buf());
    }
    find_binary(script_dir, names, binary, exists)
}

/// Port of the missing-binary error (lines 122-129, verbatim message).
pub fn missing_binary_message(names: &[String]) -> String {
    format!(
        "It seems that your package manager failed to install the right lildax CLI package. Try manually installing {} package",
        names.iter().map(|name| format!("\"{name}\"")).collect::<Vec<_>>().join(" or ")
    )
}

/// Exit-code mapping for `child.on("exit", (code, signal))` (lines 25-29):
/// signal -> re-raise (`None` = killed by signal); else `code ?? 0`.
pub fn exit_code(code: Option<i32>, signaled: bool) -> Option<i32> {
    if signaled {
        return None;
    }
    Some(code.unwrap_or(0))
}
