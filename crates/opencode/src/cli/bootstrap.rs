// source: src/cli/bootstrap.ts — exports: [bootstrap]
/// verbatim strings (source order, quoted for V2 audit):
/// - "../project/instance-runtime"
/// source: `export function bootstrap` — stub shell; CI verifies behavior.
pub fn bootstrap(payload: serde_json::Value) -> serde_json::Value {
    let _ = payload;
    serde_json::Value::Null
}
