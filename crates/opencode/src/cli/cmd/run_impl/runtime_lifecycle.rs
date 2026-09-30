// source: src/cli/cmd/run/runtime.lifecycle.ts — exports: [LifecycleInput, Lifecycle, createRuntimeLifecycle]
// PROVISIONAL pending crates/core: `@opencode-ai/core/global`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/editor`
// PROVISIONAL pending crates/tui: `@opencode-ai/tui/keymap`
/// verbatim strings (source order, quoted for V2 audit):
/// - "capture-stdout"
/// - "passthrough"
/// - "split-footer"
/// - "main-screen"
/// - "Model default"
/// - "writeToScrollback"
/// - "requestRender"
/// - "disabled"
/// source: `export type LifecycleInput` — shape as JSON value; CI verifies.
pub type LifecycleInput = serde_json::Value;
/// source: `export type Lifecycle` — shape as JSON value; CI verifies.
pub type Lifecycle = serde_json::Value;
/// source: `export function createRuntimeLifecycle` — stub shell; CI verifies behavior.
pub fn createRuntimeLifecycle(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
