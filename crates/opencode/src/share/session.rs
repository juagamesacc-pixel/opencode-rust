// source: src/share/session.ts — exports: Interface, Service, node, SessionShare
// PROVISIONAL pending @/session/session, @/config/config,
// @/effect/runtime-flags, ./share-next: "Sharing is disabled in
// configuration" + auto-share rule verbatim.

/// source: "Sharing is disabled in configuration" — verbatim.
pub const SHARE_DISABLED_MESSAGE: &str = "Sharing is disabled in configuration";

/// source: Interface — create/share/unshare, verbatim.
pub trait Interface {
    fn create(&self, session_id: Option<&str>) -> Result<String, String>;
    fn share(&self, session_id: &str) -> Result<String, String>;
    fn unshare(&self, session_id: &str) -> Result<(), String>;
}

/// source: Service "@opencode/SessionShare" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/SessionShare";

/// source: node deps [Config.node, Session.node, ShareNext.node, RuntimeFlags.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@/config/config.Config",
    "@/session/session.Session",
    "@/share/share-next.ShareNext",
    "@/effect/runtime-flags.RuntimeFlags",
];
