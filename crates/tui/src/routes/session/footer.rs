// source: packages/tui/src/routes/session/footer.tsx (91 lines, v1.18.30)
// 1:1 port — directory label left; right side switches on welcome (5s)
// vs connected state (permissions/LSP/MCP/`/status`). Welcome cycle:
// 10s → show 5s → hide 10s, paused while disconnected.

#![allow(dead_code)]

/// Welcome timing verbatim (ms).
pub const WELCOME_SHOW_MS: u64 = 5000;
pub const WELCOME_HIDE_MS: u64 = 10_000;
pub const WELCOME_FIRST_MS: u64 = 10_000;

/// Welcome banner state machine (mirrors the timeout chain).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WelcomeCycle {
    pub welcome: bool,
    deadline_ms: Option<u64>,
    started: bool,
}

impl WelcomeCycle {
    pub fn new() -> Self {
        Self {
            welcome: false,
            deadline_ms: None,
            started: false,
        }
    }

    pub fn start(&mut self, now_ms: u64) {
        if !self.started {
            self.started = true;
            self.deadline_ms = Some(now_ms + WELCOME_FIRST_MS);
        }
    }

    /// Advance; returns true while flags changed (caller re-renders).
    pub fn poll(&mut self, now_ms: u64, connected: bool) -> bool {
        if connected {
            return false;
        }
        let Some(deadline) = self.deadline_ms else {
            return false;
        };
        if now_ms < deadline {
            return false;
        }
        if !self.welcome {
            self.welcome = true;
            self.deadline_ms = Some(now_ms + WELCOME_SHOW_MS);
        } else {
            self.welcome = false;
            self.deadline_ms = Some(now_ms + WELCOME_HIDE_MS);
        }
        true
    }
}

impl Default for WelcomeCycle {
    fn default() -> Self {
        Self::new()
    }
}

/// Right-side footer segments.
#[derive(Debug, Clone)]
pub enum FooterRight {
    Welcome,
    Connected {
        permissions: usize,
        lsp: usize,
        mcp_connected: usize,
        mcp_error: bool,
    },
    Empty,
}

/// Resolve the right side (mirrors the Switch order).
pub fn footer_right(
    connected: bool,
    welcome: bool,
    permissions: usize,
    lsp: usize,
    mcp_connected: usize,
    mcp_error: bool,
) -> FooterRight {
    if welcome {
        return FooterRight::Welcome;
    }
    if !connected {
        return FooterRight::Empty;
    }
    FooterRight::Connected {
        permissions,
        lsp,
        mcp_connected,
        mcp_error,
    }
}

/// Welcome banner copy verbatim (`Get started /connect`).
pub const WELCOME_TEXT: &str = "Get started";
pub const WELCOME_HINT: &str = "/connect";

/// Permission label (`△ N Permission(s)`, verbatim triangle).
pub fn permission_label(count: usize) -> Option<String> {
    if count == 0 {
        return None;
    }
    Some(format!(
        "△ {count} Permission{}",
        if count > 1 { "s" } else { "" }
    ))
}
