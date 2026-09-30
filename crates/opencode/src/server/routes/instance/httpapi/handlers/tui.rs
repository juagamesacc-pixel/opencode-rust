// source: src/server/routes/instance/httpapi/handlers/tui.ts — exports: [tuiHandlers]
// PROVISIONAL pending external `effect` (host-provided; no new dep)
// PROVISIONAL pending external `effect/unstable/httpapi` (host-provided; no new dep)
/// verbatim strings (source order, quoted for V2 audit):
/// - "@/event-v2-bridge"
/// - "session.new"
/// - "session.share"
/// - "session.interrupt"
/// - "session.compact"
/// - "session.page.up"
/// - "session.page.down"
/// - "session.line.up"
/// source: `export const tuiHandlers` — shape as JSON value; CI verifies.
pub type tuiHandlers = serde_json::Value;
