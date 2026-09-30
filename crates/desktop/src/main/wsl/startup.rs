//! Rust port of `src/main/wsl/startup.ts` (opencode v1.18.30).
//!
//! Fully ported: `wslServerIdsToStartOnInitialize`, `expectOpencodeVersion`
//! (message via `nativeT("desktop.wsl.error.updateVersion")`), and
//! `pendingRestartAfterWslInstall`. PROVISIONAL: `pollWslHealth` (needs an
//! async runtime with `AbortSignal` + timers).
//!
//! Original file: `packages/desktop/src/main/wsl/startup.ts`

use crate::main::native_translations::native_t;

pub trait ServerIdentity {
    fn server_id(&self) -> &str;
}

pub fn wsl_server_ids_to_start_on_initialize(servers: &[impl ServerIdentity]) -> Vec<String> {
    servers
        .iter()
        .map(|server| server.server_id().to_string())
        .collect()
}

pub fn expect_opencode_version(
    installed: Option<&str>,
    expected: &str,
    distro: &str,
) -> Result<(), String> {
    if installed == Some(expected) {
        return Ok(());
    }
    let no_version = native_t("desktop.wsl.error.noVersion", &[]);
    Err(native_t(
        "desktop.wsl.error.updateVersion",
        &[
            ("distro", distro.to_string()),
            ("installed", installed.unwrap_or(&no_version).to_string()),
            ("expected", expected.to_string()),
        ],
    ))
}

pub fn pending_restart_after_wsl_install(runtime_available: bool) -> bool {
    !runtime_available
}

// PROVISIONAL(packages/desktop/src/main/wsl/startup.ts): needs an async
// runtime (`AbortSignal`, `setTimeout` loop, default `interval = 100`).
// Signature mirrors `pollWslHealth(check, signal, interval = 100)`.
pub fn poll_wsl_health(_interval_ms: u64) {
    unimplemented!("async runtime + AbortSignal binding")
}

#[cfg(test)]
mod tests {
    // Assertions from `src/main/wsl/servers.test.ts` targeting `./startup`.
    // Test-only bundle fixture mirrors the `DesktopNativeBundle` shape; the
    // `updateVersion` value reproduces the English text pinned by the
    // source test (`"OpenCode update finished but Debian still reports
    // 1.14.35; expected 1.16.2"`).
    use super::*;
    use crate::main::native_translations::set_native_translations;
    use std::collections::HashMap;

    fn install_test_bundle() {
        set_native_translations(
            "test",
            HashMap::from([
                (
                    "desktop.wsl.error.updateVersion".to_string(),
                    "OpenCode update finished but {{distro}} still reports {{installed}}; expected {{expected}}".to_string(),
                ),
                ("desktop.wsl.error.noVersion".to_string(), "no version".to_string()),
            ]),
        );
    }

    struct TestServer {
        id: String,
    }

    impl ServerIdentity for TestServer {
        fn server_id(&self) -> &str {
            &self.id
        }
    }

    #[test]
    fn starts_every_configured_wsl_server_on_initialization() {
        assert_eq!(
            wsl_server_ids_to_start_on_initialize(&[
                TestServer {
                    id: "wsl:Debian".to_string()
                },
                TestServer {
                    id: "wsl:Ubuntu-24.04".to_string()
                },
            ]),
            vec!["wsl:Debian".to_string(), "wsl:Ubuntu-24.04".to_string()]
        );
    }

    #[test]
    fn rejects_an_update_that_did_not_install_the_desktop_version() {
        install_test_bundle();
        assert!(expect_opencode_version(Some("1.16.2"), "1.16.2", "Debian").is_ok());
        let error =
            expect_opencode_version(Some("1.14.35"), "1.16.2", "Debian").expect_err("mismatch");
        assert!(
            error.contains(
                "OpenCode update finished but Debian still reports 1.14.35; expected 1.16.2"
            ),
            "unexpected message: {}",
            error
        );
    }

    #[test]
    fn derives_a_required_windows_restart_from_the_post_install_runtime_probe() {
        assert!(pending_restart_after_wsl_install(false));
        assert!(!pending_restart_after_wsl_install(true));
    }
}
