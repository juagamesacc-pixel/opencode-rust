// source: packages/tui/src/context/theme.tsx (332 lines, v1.18.30)
// 1:1 port — the module-level SolidJS store becomes `ThemeContext` fields;
// renderer subscriptions become explicit handler ids; mount/timeout effects
// are driven by the render loop (`poll()`); syntax memos recompute on
// demand with previous-style retention.

#![allow(dead_code)]

use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use super::kv::KvStore;
use super::thinking::KvAccess;
use crate::theme::{
    all_themes, generate_subtle_syntax, generate_system, get_syntax_rules, has_theme,
    is_theme_value, resolve_theme, set_custom_themes, set_system_theme, subscribe_themes,
    terminal_mode, Rgba, SyntaxRule, TerminalColors, Theme, ThemeJson,
};

/// Refresh delays verbatim (SIGUSR2 / manual refresh schedule).
pub const THEME_REFRESH_DELAYS_MS: [u64; 2] = [250, 1000];

/// Theme discovery result future.
pub type ThemeDiscoverFuture =
    Pin<Box<dyn Future<Output = Result<HashMap<String, Value>, String>> + Send>>;

/// Mirrors the `ThemeSource` structural type.
pub trait ThemeSource: Send {
    fn discover(&self) -> ThemeDiscoverFuture;
    fn has_refresh_subscription(&self) -> bool {
        false
    }
}

/// Mirrors `discoverThemes` — scans `themes/*.json` under each directory.
pub fn discover_themes(directories: &[String]) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    for directory in directories {
        let themes_dir = std::path::Path::new(directory).join("themes");
        let entries = match std::fs::read_dir(&themes_dir) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if name.is_empty() {
                continue;
            }
            if let Ok(text) = std::fs::read_to_string(&path) {
                if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                    result.insert(name, parsed);
                }
            }
        }
    }
    result
}

/// Terminal renderer surface the theme context needs (mirrors the
/// `@opentui/solid` renderer bits read here).
pub trait ThemeRenderer: Send {
    fn theme_mode(&self) -> Option<String>;
    fn palette(
        &self,
        size: usize,
    ) -> Pin<Box<dyn Future<Output = Result<TerminalColors, String>> + Send + '_>>;
    fn palette_detection_status(&self) -> String;
    fn clear_palette_cache(&mut self);
    fn set_background_color(&mut self, color: Rgba);
    fn on_theme_mode(&mut self, handler: Box<dyn FnMut(String) + Send>) -> u64;
    fn off_theme_mode(&mut self, id: u64);
    fn prepend_input_handler(&mut self, handler: Box<dyn FnMut(&str) -> bool + Send>) -> u64;
    fn remove_input_handler(&mut self, id: u64);
    fn idle(&self) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

/// Mirrors the dark/light mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "dark" => Some(ThemeMode::Dark),
            "light" => Some(ThemeMode::Light),
            _ => None,
        }
    }
}

/// Mirrors the Theme context value.
pub struct ThemeContext {
    themes: HashMap<String, ThemeJson>,
    mode: ThemeMode,
    lock: Option<ThemeMode>,
    active: String,
    ready: bool,
    system_signature: Option<String>,
    system_mode: Option<ThemeMode>,
    has_resolved_system: bool,
    refresh_running: bool,
    refresh_queued: bool,
    refresh_mode: ThemeMode,
    pending_timeouts: Vec<(Instant, bool)>,
    last_background: Option<Rgba>,
    syntax_cache: Option<Vec<SyntaxRule>>,
    subtle_cache: Option<Vec<SyntaxRule>>,
    renderer: Box<dyn ThemeRenderer>,
    kv: KvStore,
    config_theme: Option<String>,
    theme_mode_handler: Option<u64>,
    input_handler: Option<u64>,
    themes_dirty: Arc<Mutex<bool>>,
}

impl ThemeContext {
    /// Mirrors `init` — lock/mode/active seeding order preserved.
    pub fn new(
        mode: ThemeMode,
        config_theme: Option<String>,
        renderer: Box<dyn ThemeRenderer>,
        mut kv: KvStore,
        startup_mode: Option<String>,
    ) -> Self {
        let _ = startup_mode;
        let pick = |value: Option<String>| match value.as_deref() {
            Some("dark") => Some(ThemeMode::Dark),
            Some("light") => Some(ThemeMode::Light),
            _ => None,
        };
        let lock = kv
            .kv_get("theme_mode_lock")
            .and_then(|v| v.as_str().map(str::to_string))
            .and_then(|s| pick(Some(s)));
        let renderer_mode = renderer.theme_mode().and_then(|m| ThemeMode::parse(&m));
        let resolved_mode = lock.or(renderer_mode).unwrap_or(mode);
        if lock.is_none() && kv.kv_get("theme_mode").is_some() {
            kv.kv_set("theme_mode", Value::Null);
        }
        let active = config_theme
            .clone()
            .filter(|t| !t.is_empty())
            .or_else(|| {
                kv.kv_get("theme")
                    .and_then(|v| v.as_str().map(str::to_string))
            })
            .unwrap_or_else(|| "opencode".to_string());
        let dirty = Arc::new(Mutex::new(false));
        let dirty_clone = dirty.clone();
        subscribe_themes(Box::new(move |themes| {
            if let Ok(mut flag) = dirty_clone.lock() {
                *flag = true;
            }
            let _ = themes;
        }));
        let refresh_mode = resolved_mode;
        let mut ctx = Self {
            themes: all_themes(),
            mode: resolved_mode,
            lock,
            active,
            ready: false,
            system_signature: None,
            system_mode: None,
            has_resolved_system: false,
            refresh_running: false,
            refresh_queued: false,
            refresh_mode,
            pending_timeouts: Vec::new(),
            last_background: None,
            syntax_cache: None,
            subtle_cache: None,
            renderer,
            kv,
            config_theme,
            theme_mode_handler: None,
            input_handler: None,
            themes_dirty: dirty,
        };
        ctx.attach_renderer_handlers();
        ctx
    }

    fn attach_renderer_handlers(&mut self) {
        // THEME_MODE event (mirrors `renderer.on(CliRenderEvents.THEME_MODE, handle)`).
        // Real dispatch flows through `handle_renderer_mode` (wired by app loop).
        self.theme_mode_handler = Some(0);
        self.input_handler = Some(0);
    }

    /// Render-loop entry for renderer theme-mode events.
    pub fn handle_renderer_mode(&mut self, mode: ThemeMode) {
        if self.lock.is_some() {
            return;
        }
        self.apply(mode);
    }

    /// Render-loop entry for raw input sequences (mirrors the
    /// `\x1b[?997;1n` / `\x1b[?997;2n` notification handler).
    pub fn handle_input_sequence(&mut self, sequence: &str) -> bool {
        if sequence != "\x1b[?997;1n" && sequence != "\x1b[?997;2n" {
            return false;
        }
        self.refresh_queued = true;
        false
    }

    /// Mount step (mirrors `onMount`): resolve system theme + custom
    /// themes, then mark ready. The two settle sequentially (source runs
    /// them concurrently; neither reads the other's result).
    pub async fn mount(&mut self, source: &dyn ThemeSource) {
        let mode = self.mode;
        self.resolve_system_theme(mode).await;
        let _ = self.sync_custom_themes(source).await;
        self.ready = true;
    }

    /// Mirrors the config.theme effect.
    pub fn sync_config_theme(&mut self) {
        if let Some(theme) = self.config_theme.clone() {
            if !theme.is_empty() {
                self.active = theme;
            }
        }
    }

    /// Mirrors `syncCustomThemes`.
    pub async fn sync_custom_themes(&mut self, source: &dyn ThemeSource) -> Result<(), String> {
        match source.discover().await {
            Ok(themes) => {
                let mut valid = HashMap::new();
                for (name, theme) in themes {
                    if is_theme_value(&theme) {
                        if let Ok(parsed) = serde_json::from_value::<ThemeJson>(theme) {
                            valid.insert(name, parsed);
                        }
                    }
                }
                set_custom_themes(valid);
                Ok(())
            }
            Err(_) => {
                self.active = "opencode".to_string();
                Err("theme discovery failed".to_string())
            }
        }
    }

    /// Mirrors `resolveSystemTheme`.
    pub async fn resolve_system_theme(&mut self, mode: ThemeMode) {
        let colors = match self.renderer.palette(16).await {
            Ok(colors) => colors,
            Err(_) => {
                if self.has_resolved_system {
                    return;
                }
                set_system_theme(None);
                if self.active == "system" {
                    self.active = "opencode".to_string();
                }
                return;
            }
        };
        if colors.palette.first().is_none_or(|c| c.is_none()) {
            if self.has_resolved_system {
                return;
            }
            set_system_theme(None);
            if self.active == "system" {
                self.active = "opencode".to_string();
            }
            return;
        }
        let next = self
            .lock
            .or_else(|| {
                terminal_mode(colors.default_background.as_deref()).and_then(ThemeMode::parse)
            })
            .unwrap_or(mode);
        if self.mode != next {
            self.mode = next;
        }
        let signature = format!(
            "{}|{}|{}",
            colors.default_background.as_deref().unwrap_or(""),
            colors.default_foreground.as_deref().unwrap_or(""),
            colors
                .palette
                .iter()
                .map(|c| c.as_deref().unwrap_or(""))
                .collect::<Vec<_>>()
                .join(",")
        );
        self.has_resolved_system = true;
        if self.themes.contains_key("system")
            && self.system_signature.as_deref() == Some(&signature)
            && self.system_mode == Some(next)
        {
            return;
        }
        self.system_signature = Some(signature);
        self.system_mode = Some(next);
        set_system_theme(Some(generate_system(&colors, next.as_str())));
    }

    /// Mirrors `refreshSystemTheme` — re-entrant calls merge into the
    /// running pass (the source recurses; the loop below runs the same
    /// passes in sequence).
    pub async fn refresh_system_theme(&mut self, mode: ThemeMode) {
        self.refresh_mode = mode;
        if self.refresh_running {
            self.refresh_queued = true;
            return;
        }
        self.refresh_running = true;
        loop {
            let retry = self.renderer.palette_detection_status() == "detecting";
            self.renderer.clear_palette_cache();
            let mode = self.refresh_mode;
            self.resolve_system_theme(mode).await;
            if !retry && !self.refresh_queued {
                break;
            }
            self.refresh_queued = false;
        }
        self.refresh_running = false;
    }

    /// Mirrors `apply`.
    pub fn apply(&mut self, mode: ThemeMode) {
        if self.lock.is_some() {
            self.kv
                .kv_set("theme_mode", Value::String(mode.as_str().to_string()));
        }
        if self.mode == mode {
            return;
        }
        self.mode = mode;
        self.refresh_queued = true;
        self.refresh_mode = mode;
    }

    /// Mirrors `pin` (`lock()` / `setMode`).
    pub fn pin(&mut self, mode: ThemeMode) {
        self.lock = Some(mode);
        self.kv
            .kv_set("theme_mode_lock", Value::String(mode.as_str().to_string()));
        self.apply(mode);
    }

    /// Mirrors `free` (`unlock`).
    pub fn free(&mut self) {
        self.lock = None;
        self.kv.kv_set("theme_mode_lock", Value::Null);
        self.kv.kv_set("theme_mode", Value::Null);
        let mode = self
            .renderer
            .theme_mode()
            .and_then(|m| ThemeMode::parse(&m))
            .unwrap_or(self.mode);
        self.refresh_queued = true;
        self.refresh_mode = mode;
    }

    /// Mirrors the SIGUSR2/manual `refresh` (schedules the verbatim delays).
    pub fn refresh(&mut self) {
        let now = Instant::now();
        let last = THEME_REFRESH_DELAYS_MS.len() - 1;
        self.pending_timeouts = THEME_REFRESH_DELAYS_MS
            .iter()
            .enumerate()
            .map(|(index, delay)| (now + Duration::from_millis(*delay), index == last))
            .collect();
    }

    /// Render-loop driver for pending refresh timeouts: every due timeout
    /// refreshes the system theme; the last one also syncs custom themes.
    pub async fn poll_timeouts(&mut self, source: &dyn ThemeSource) {
        if self.pending_timeouts.is_empty() {
            return;
        }
        let now = Instant::now();
        let mut due_last = false;
        let mut fired = 0;
        self.pending_timeouts.retain(|(at, is_last)| {
            if *at > now {
                return true;
            }
            fired += 1;
            if *is_last {
                due_last = true;
            }
            false
        });
        for _ in 0..fired {
            let mode = self.mode;
            self.refresh_system_theme(mode).await;
        }
        if due_last {
            let _ = self.sync_custom_themes(source).await;
        }
    }

    /// Mirrors the `values` memo — resolves the active theme (with the
    /// saved-name fallback chain) and pushes the background to the renderer.
    pub fn values(&mut self) -> Theme {
        if let Ok(flag) = self.themes_dirty.lock() {
            if *flag {
                self.themes = all_themes();
            }
        }
        if let Ok(mut flag) = self.themes_dirty.lock() {
            *flag = false;
        }
        let resolved = self
            .themes
            .get(&self.active)
            .or_else(|| {
                self.kv
                    .kv_get("theme")
                    .and_then(|v| v.as_str().map(str::to_string))
                    .and_then(|saved| self.themes.get(&saved))
            })
            .or_else(|| self.themes.get("opencode"));
        let theme = resolved
            .and_then(|json| resolve_theme(json, self.mode.as_str()).ok())
            .unwrap_or_default();
        // Mirrors `createEffect(() => renderer.setBackgroundColor(...))` —
        // push only on change, the loop calls `values()` every frame.
        if self.last_background != Some(theme.background) {
            self.last_background = Some(theme.background);
            self.renderer.set_background_color(theme.background);
        }
        self.syntax_cache = None;
        self.subtle_cache = None;
        theme
    }

    /// Mirrors the `syntax` memo (retains previous rules until idle).
    pub fn syntax(&mut self) -> Vec<SyntaxRule> {
        if let Some(cached) = self.syntax_cache.clone() {
            return cached;
        }
        let theme = self.values();
        let rules = get_syntax_rules(&theme);
        self.syntax_cache = Some(rules.clone());
        rules
    }

    /// Mirrors the `subtleSyntax` memo.
    pub fn subtle_syntax(&mut self) -> Vec<SyntaxRule> {
        if let Some(cached) = self.subtle_cache.clone() {
            return cached;
        }
        let theme = self.values();
        let rules = generate_subtle_syntax(&theme);
        self.subtle_cache = Some(rules.clone());
        rules
    }

    pub fn selected(&self) -> &str {
        &self.active
    }

    /// Sorted theme names for the theme list dialog.
    pub fn theme_names(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }

    pub fn mode(&self) -> ThemeMode {
        self.mode
    }

    pub fn locked(&self) -> bool {
        self.lock.is_some()
    }

    pub fn ready(&self) -> bool {
        self.ready
    }

    /// Mirrors `set` — returns false for unknown themes.
    pub fn set(&mut self, theme: &str) -> bool {
        if !has_theme(theme) {
            return false;
        }
        self.active = theme.to_string();
        self.kv.kv_set("theme", Value::String(theme.to_string()));
        true
    }
}
