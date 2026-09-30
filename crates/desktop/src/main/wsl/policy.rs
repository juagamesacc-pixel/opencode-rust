//! Rust port of `src/main/wsl/policy.ts` (opencode v1.18.30).
//!
//! Original file: `packages/desktop/src/main/wsl/policy.ts`

use crate::preload::types::{WslDistroProbe, WslOpencodeCheck, WslServerItem};
use std::collections::HashMap;

pub fn wsl_server_id_to_restart(servers: &[WslServerItem], distro: &str) -> Option<String> {
    servers
        .iter()
        .find(|item| item.config.distro == distro)
        .map(|item| item.config.id.clone())
}

pub struct ClearedWslDistroState {
    pub distro_probes: HashMap<String, WslDistroProbe>,
    pub opencode_checks: HashMap<String, WslOpencodeCheck>,
}

pub fn clear_wsl_distro_state(
    distro_probes: &HashMap<String, WslDistroProbe>,
    opencode_checks: &HashMap<String, WslOpencodeCheck>,
    distro: &str,
) -> ClearedWslDistroState {
    let mut next_distro_probes = distro_probes.clone();
    let mut next_opencode_checks = opencode_checks.clone();
    next_distro_probes.remove(distro);
    next_opencode_checks.remove(distro);
    ClearedWslDistroState {
        distro_probes: next_distro_probes,
        opencode_checks: next_opencode_checks,
    }
}

pub fn wsl_terminal_args(distro: Option<&str>) -> Vec<String> {
    let mut args = vec![
        "/c".to_string(),
        "start".to_string(),
        String::new(),
        "wsl".to_string(),
    ];
    // Mirrors `...(distro ? ["-d", distro] : [])`: empty strings are falsy.
    if let Some(distro) = distro.filter(|distro| !distro.is_empty()) {
        args.push("-d".to_string());
        args.push(distro.to_string());
    }
    args
}

pub fn require_wsl_ipc_string(name: &str, value: &serde_json::Value) -> Result<String, String> {
    match value.as_str() {
        Some(value) if !value.is_empty() => Ok(value.to_string()),
        _ => Err(format!("Invalid {}", name)),
    }
}

pub fn require_wsl_ipc_strings(
    name: &str,
    value: &serde_json::Value,
) -> Result<Vec<String>, String> {
    let items = match value.as_array() {
        Some(items) => items,
        None => return Err(format!("Invalid {}", name)),
    };
    let mut values = Vec::with_capacity(items.len());
    for item in items {
        values.push(require_wsl_ipc_string(name, item)?);
    }
    if values.is_empty() {
        return Err(format!("Invalid {}", name));
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    // Assertions from `src/main/wsl/servers.test.ts` targeting `./policy`
    // (the plan records the split: each assertion lives in the module it
    // targets).
    use super::*;
    use crate::preload::types::{WslServerConfig, WslServerRuntime};

    #[test]
    fn restarts_an_existing_distro_server_after_updating_opencode() {
        let servers = [WslServerItem {
            config: WslServerConfig {
                id: "wsl:Debian".to_string(),
                distro: "Debian".to_string(),
            },
            runtime: WslServerRuntime::Ready {
                url: String::new(),
                username: None,
                password: None,
            },
        }];
        assert_eq!(
            wsl_server_id_to_restart(&servers, "Debian"),
            Some("wsl:Debian".to_string())
        );
        assert_eq!(wsl_server_id_to_restart(&[], "Debian"), None);
    }

    #[test]
    fn clears_cached_distro_probes_when_removing_a_wsl_server() {
        let cleared = clear_wsl_distro_state(
            &HashMap::from([(
                "Debian".to_string(),
                WslDistroProbe {
                    name: "Debian".to_string(),
                    can_execute: true,
                    has_bash: true,
                    has_curl: true,
                    error: None,
                },
            )]),
            &HashMap::from([(
                "Debian".to_string(),
                WslOpencodeCheck {
                    distro: "Debian".to_string(),
                    resolved_path: Some("/home/luke/.opencode/bin/opencode".to_string()),
                    version: Some("1.16.2".to_string()),
                    expected_version: "1.16.2".to_string(),
                    matches_desktop: Some(true),
                    error: None,
                },
            )]),
            "Debian",
        );
        assert!(cleared.distro_probes.is_empty());
        assert!(cleared.opencode_checks.is_empty());
    }

    #[test]
    fn opens_terminals_for_distro_names_containing_spaces() {
        assert_eq!(
            wsl_terminal_args(Some("Ubuntu Preview")),
            vec!["/c", "start", "", "wsl", "-d", "Ubuntu Preview"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn validates_wsl_ipc_identifiers_at_the_module_boundary() {
        assert_eq!(
            require_wsl_ipc_string("distro", &serde_json::json!("Debian")),
            Ok("Debian".to_string())
        );
        assert_eq!(
            require_wsl_ipc_strings("distro", &serde_json::json!(["Debian", "Ubuntu"])),
            Ok(vec!["Debian".to_string(), "Ubuntu".to_string()])
        );
        assert_eq!(
            require_wsl_ipc_string("distro", &serde_json::json!("")),
            Err("Invalid distro".to_string())
        );
        assert_eq!(
            require_wsl_ipc_string("server id", &serde_json::Value::Null),
            Err("Invalid server id".to_string())
        );
        assert_eq!(
            require_wsl_ipc_strings("distro", &serde_json::json!([])),
            Err("Invalid distro".to_string())
        );
    }
}
