//! Rust port of `src/main/index.ts` (opencode v1.18.30).
//!
//! The desktop entrypoint. The pure, testable decision points of the
//! bootstrap are ported as live code — the channel-based app identity
//! (`APP_NAMES`/`APP_IDS`), the onboarding/sidecar environment flags, the
//! deep-link queue, and the loopback `NO_PROXY` upsert. The Electron runtime
//! wiring itself is [`main`], which stays PROVISIONAL until the runtime
//! bindings exist; its doc comment preserves the full ordered effect.
//!
//! Original file: `packages/desktop/src/main/index.ts`

use crate::main::constants::Channel;
use std::cell::RefCell;
use std::collections::HashMap;

/// Mirrors `const APP_IDS: Record<string, string>` selected by
/// `app.isPackaged ? APP_IDS[CHANNEL] : "ai.opencode.desktop.dev"`.
pub fn app_id(is_packaged: bool, channel: Channel) -> &'static str {
    if !is_packaged {
        return "ai.opencode.desktop.dev";
    }
    match channel {
        Channel::Dev => "ai.opencode.desktop.dev",
        Channel::Beta => "ai.opencode.desktop.beta",
        Channel::Prod => "ai.opencode.desktop",
    }
}

/// Mirrors `const APP_NAMES: Record<string, string>` selected by
/// `app.setName(app.isPackaged ? APP_NAMES[CHANNEL] : "OpenCode Dev")`.
pub fn app_name(is_packaged: bool, channel: Channel) -> &'static str {
    if !is_packaged {
        return "OpenCode Dev";
    }
    match channel {
        Channel::Dev => "OpenCode Dev",
        Channel::Beta => "OpenCode Beta",
        Channel::Prod => "OpenCode",
    }
}

/// The `process.env.OPENCODE_TEST_ONBOARDING === "1"` comparison behind
/// `const TEST_ONBOARDING`.
pub fn test_onboarding_from(raw: Option<&str>) -> bool {
    raw == Some("1")
}

/// `const TEST_ONBOARDING` evaluated from the environment.
pub fn test_onboarding() -> bool {
    test_onboarding_from(std::env::var("OPENCODE_TEST_ONBOARDING").ok().as_deref())
}

/// The ternary behind `const SIDECAR_VERSION`.
pub fn sidecar_version_from(raw: Option<&str>) -> &'static str {
    if raw == Some("1") {
        "v2"
    } else {
        "v1"
    }
}

/// `const SIDECAR_VERSION` evaluated from the environment.
pub fn sidecar_version() -> &'static str {
    sidecar_version_from(std::env::var("OPENCODE_SIDECAR_V2").ok().as_deref())
}

/// `const jsCallStackFeature`;
/// `app.commandLine.appendSwitch("enable-features", ...)` composes it with
/// the existing feature list.
pub const JS_CALL_STACK_FEATURE: &str = "DocumentPolicyIncludeJSCallStacksInCrashReports";

/// `const pendingDeepLinks: string[] = []` — the renderer never sees these
/// until `consumeInitialDeepLinks` (from `ipc.ts`) drains them.
thread_local! {
    static PENDING_DEEP_LINKS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

/// `emitDeepLinks(urls)`: queue the urls and, when a window is focused, hand
/// them to `sendDeepLinks(win, urls)`.
///
/// Returns the urls that would be sent — the empty vec when there is no
/// focused window (`getLastFocusedWindow` — PROVISIONAL, so the caller
/// supplies the determination). An empty batch is dropped before queuing,
/// exactly like the source's early return.
pub fn emit_deep_links(urls: &[String], window_available: bool) -> Vec<String> {
    if urls.is_empty() {
        return Vec::new();
    }
    PENDING_DEEP_LINKS.with(|pending| pending.borrow_mut().extend_from_slice(urls));
    if window_available {
        urls.to_vec()
    } else {
        Vec::new()
    }
}

/// `consumeInitialDeepLinks` — `pendingDeepLinks.splice(0)` returns every
/// queued url and empties the queue.
pub fn consume_initial_deep_links() -> Vec<String> {
    PENDING_DEEP_LINKS.with(|pending| std::mem::take(&mut *pending.borrow_mut()))
}

/// `const loopback = ["127.0.0.1", "localhost", "::1"]` from
/// `ensureLoopbackNoProxy`.
pub const LOOPBACK_HOSTS: [&str; 3] = ["127.0.0.1", "localhost", "::1"];

/// One `upsert(key)` run: split the current value on commas, trim, drop empty
/// segments, then append every loopback host that is not already present
/// (compared case-insensitively, as `value.toLowerCase() === host`).
pub fn upsert_loopback_hosts(current: &str) -> String {
    let mut items: Vec<String> = current
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect();
    for host in LOOPBACK_HOSTS {
        if items.iter().any(|value| value.eq_ignore_ascii_case(host)) {
            continue;
        }
        items.push((*host).to_string());
    }
    items.join(",")
}

/// `ensureLoopbackNoProxy()`: apply the upsert to both `NO_PROXY` and
/// `no_proxy`, always rewriting them (the source always assigns).
///
/// `packages/desktop/src/main/sidecar.ts` carries an identical copy of this
/// function; that one is ported as `crate::main::sidecar::apply_loopback_no_proxy`.
pub fn ensure_loopback_no_proxy(env: &mut HashMap<String, String>) {
    for key in ["NO_PROXY", "no_proxy"] {
        let current = env.get(key).map(String::as_str).unwrap_or("");
        env.insert(key.to_string(), upsert_loopback_hosts(current));
    }
}

/// `useEnvProxy()` — `http.setGlobalProxyFromEnv()` guarded by a
/// `logger.warn("failed to load proxy environment", error)` catch.
pub fn use_env_proxy() {
    unimplemented!("Electron http.setGlobalProxyFromEnv binding")
}

/// `const main = Effect.gen(...)` then `Effect.runFork(main)` — the Electron
/// bootstrap. PROVISIONAL until the runtime bindings exist; the ordered steps
/// the effect performs, with their port counterparts where the port already
/// names them:
///
/// 1. `contextMenu({ showSaveImageAs: true, showLookUpSelection: false,
///    showSearchWithGoogle: false })` — Electron context-menu binding.
/// 2. `chdir(homedir())` ignoring failures (macOS runs in `/`, breaking
///    ripgrep).
/// 3. `process.env.OPENCODE_DISABLE_EMBEDDED_WEB_UI = "true"`.
/// 4. `TEST_ONBOARDING` → `opencode-onboarding-<uuid>` under `tmpdir()`, rm +
///    mkdir `data`/`config`/`cache`/`state`/`desktop`/`session`, then
///    `OPENCODE_DB = ":memory:"` and the four `XDG_*_HOME` env vars.
/// 5. `app.setName(app_name(is_packaged, channel()))`,
///    `app.setAppUserModelId(app_id(...))`, and
///    `app.setPath("userData", ...)` (plus `"sessionData"` under the
///    onboarding root when testing).
/// 6. `initialize_old_layout_eligibility(app.getPath("userData"))`.
/// 7. `initLogging()` → the `logger` used by every later step; then
///    `initCrashReporter()` (PROVISIONAL).
/// 8. `createWslServersController(app.getVersion(), spawnWslSidecar, logger)`
///    (`wsl::servers::create_wsl_servers_controller`) and the
///    `stopSidecars`/`relaunch` closures
///    (`windows::set_app_quitting`; `app.relaunch`/`app.quit` PROVISIONAL).
/// 9. `setDefaultCACertificates([...getCACertificates("default"),
///    ...getCACertificates("system")])` (PROVISIONAL;
///    `logger.warn("failed to load system certificates", ...)`).
/// 10. `logger.log("app starting", { version, packaged, onboardingTest })`.
/// 11. `ensure_loopback_no_proxy` over `std::env`, `use_env_proxy`,
///    `proxy-bypass-list = "<-loopback>"`, `enable-features` +
///    `JS_CALL_STACK_FEATURE` (composed with any existing value),
///    `remote-debugging-port 9222` when unpackaged.
/// 12. `requestSingleInstanceLock`, `app.quit()` + return when refused.
/// 13. `prefer_app_env(app.getPath("userData"))`
///    (`server::prefer_app_env`).
/// 14. Lifecycle handlers: `second-instance` (filter `opencode://` argv →
///    "deep link received via second-instance" `{urls}` + `emit_deep_links`;
///    show/focus the focused window), `open-url` (preventDefault;
///    "deep link received via open-url" `{url}`),
///    `before-quit`/`will-quit` (`set_app_quitting` + `stopSidecars`),
///    `child-process-gone`/`render-process-gone`
///    (`logging::write_log("utility"|"window", ..., "error")`).
/// 15. `setRelaunchHandler(relaunch)`; SIGINT/SIGTERM → `set_app_quitting`,
///    `stopSidecars`, quit.
/// 16. The `serverReady` deferred; `app.whenReady()`.
/// 17. `!TEST_ONBOARDING` → `migrate()` (PROVISIONAL). Then
///    `store_cleanup::cleanup_store_files(user_data_path)` →
///    "cleaned scoped store files" `{count, scanned}` (or
///    "failed to clean scoped store files").
/// 18. `setAsDefaultProtocolClient("opencode")`,
///    `windows::register_renderer_protocol`, `windows::set_dock_icon`.
/// 19. `setup_auto_updater(stopSidecars)`
///    (`updater::setup_auto_updater`) and `menuDeps` (`menu::MenuDeps`:
///    trigger → `windows::send_menu_command` on the focused window,
///    checkForUpdates → `updater::show_updater_dialog`, relaunch).
/// 20. `ipc::register_ipc_handlers(deps)` with the full deps bundle
///    (killSidecar, relaunch, awaitInitialization via `serverReady`,
///    `consume_initial_deep_links`, `server::get_default_server_url` /
///    `server::set_default_server_url`, onboarding + old-layout checks,
///    display-backend get/set, `apps::check_app_exists` /
///    `apps::resolve_app_path`, updater, showUpdater,
///    `windows::set_background_color`, `logging::export_debug_logs`,
///    recordFatalRendererError → `logging::write_log("renderer", ...)`,
///    setNativeTranslations → rebuild the menu when it changes); then
///    `wsl::ipc::register_wsl_ipc_handlers`; `updater.start()` and a
///    10-minute `setInterval` check, `unref()`d.
/// 21. `logging::start_net_log()` with "failed to start net log" on error.
/// 22. Loading task ("sidecar connection started" `{version:
///    SIDECAR_VERSION}`, `ensure_loopback_no_proxy` + `use_env_proxy`
///    again): v2 → `background_cli::start_background_cli(logger,
///    XDG_STATE_HOME)` and succeed `serverReady` with its url/username/
///    password; v1 → port selection (`OPENCODE_PORT` parse, else a
///    `listen(0)` probe), `http://127.0.0.1:<port>`, `randomUUID` password,
///    `server::spawn_local_server(hostname, port, password, opts)` (stdout/
///    stderr/exit → `logging::write_log`), succeed `serverReady`, then await
///    `health.wait` with a 30s timeout ("sidecar health check failed").
///    Win32 → `wslServers.initialize()` ("wsl server initialization
///    failed"). "loading task finished"; fork + awaited.
/// 23. `window-all-closed` (quit, except darwin) / `activate`
///    (`windows::restore_main_windows` when no windows remain).
/// 24. `restore_main_windows()`; `menu::create_menu(&mut menu_deps)` when it
///    restored at least one window.
pub fn main() {
    unimplemented!("Electron runtime bootstrap (see doc comment for the ordered steps)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unpackaged_apps_are_always_the_dev_identity() {
        for channel in [Channel::Dev, Channel::Beta, Channel::Prod] {
            assert_eq!(app_id(false, channel), "ai.opencode.desktop.dev");
            assert_eq!(app_name(false, channel), "OpenCode Dev");
        }
    }

    #[test]
    fn packaged_apps_use_the_channel_identity() {
        assert_eq!(app_id(true, Channel::Dev), "ai.opencode.desktop.dev");
        assert_eq!(app_id(true, Channel::Beta), "ai.opencode.desktop.beta");
        assert_eq!(app_id(true, Channel::Prod), "ai.opencode.desktop");
        assert_eq!(app_name(true, Channel::Dev), "OpenCode Dev");
        assert_eq!(app_name(true, Channel::Beta), "OpenCode Beta");
        assert_eq!(app_name(true, Channel::Prod), "OpenCode");
    }

    #[test]
    fn the_onboarding_flag_is_a_strict_one() {
        assert!(!test_onboarding_from(None));
        assert!(!test_onboarding_from(Some("")));
        assert!(!test_onboarding_from(Some("true")));
        assert!(!test_onboarding_from(Some("11")));
        assert!(test_onboarding_from(Some("1")));
    }

    #[test]
    fn sidecar_version_defaults_to_v1() {
        assert_eq!(sidecar_version_from(None), "v1");
        assert_eq!(sidecar_version_from(Some("0")), "v1");
        assert_eq!(sidecar_version_from(Some("1")), "v2");
    }

    #[test]
    fn the_js_call_stack_feature_is_exact() {
        assert_eq!(
            JS_CALL_STACK_FEATURE,
            "DocumentPolicyIncludeJSCallStacksInCrashReports"
        );
    }

    #[test]
    fn emit_queue_then_consume_returns_everything_in_order() {
        // Every test runs on its own thread, so the thread-local queue starts
        // empty and never leaks between tests.
        assert!(consume_initial_deep_links().is_empty());

        // An empty batch is dropped before queuing (the source early-returns).
        assert_eq!(emit_deep_links(&[], true), Vec::<String>::new());
        assert!(consume_initial_deep_links().is_empty());

        let a = "opencode://a".to_string();
        let b = "opencode://b".to_string();
        assert_eq!(
            emit_deep_links(&[a.clone(), b.clone()], true),
            vec![a.clone(), b.clone()]
        );
        assert_eq!(consume_initial_deep_links(), vec![a, b]);
    }

    #[test]
    fn deep_links_are_queued_even_without_a_focused_window() {
        assert!(consume_initial_deep_links().is_empty());
        let a = "opencode://a".to_string();
        assert_eq!(emit_deep_links(&[a.clone()], false), Vec::<String>::new());
        assert_eq!(consume_initial_deep_links(), vec![a]);
    }

    #[test]
    fn loopback_hosts_are_appended_until_present() {
        assert_eq!(upsert_loopback_hosts(""), "127.0.0.1,localhost,::1");
        assert_eq!(
            upsert_loopback_hosts("example.com"),
            "example.com,127.0.0.1,localhost,::1"
        );
        // Case-insensitive dedup: "LOCALHOST" already covers the host.
        assert_eq!(
            upsert_loopback_hosts("LOCALHOST,example.com"),
            "LOCALHOST,example.com,127.0.0.1,::1"
        );
        // Empty segments are trimmed away before the append.
        assert_eq!(
            upsert_loopback_hosts("a,, b, "),
            "a,b,127.0.0.1,localhost,::1"
        );
    }

    #[test]
    fn ensure_upserts_both_keys_and_is_stable() {
        let mut env = HashMap::new();
        env.insert("NO_PROXY".to_string(), "example.com".to_string());
        ensure_loopback_no_proxy(&mut env);
        assert_eq!(
            env.get("NO_PROXY").map(String::as_str),
            Some("example.com,127.0.0.1,localhost,::1")
        );
        assert_eq!(
            env.get("no_proxy").map(String::as_str),
            Some("127.0.0.1,localhost,::1")
        );
        // The source always reassigns; a second run stays identical.
        ensure_loopback_no_proxy(&mut env);
        assert_eq!(
            env.get("NO_PROXY").map(String::as_str),
            Some("example.com,127.0.0.1,localhost,::1")
        );
    }
}
