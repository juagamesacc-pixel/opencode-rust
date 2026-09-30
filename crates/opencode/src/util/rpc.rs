// source: src/util/rpc.ts — exports: listen, emit, client, Rpc
// Worker-postMessage RPC: type tags "rpc.request"/"rpc.result"/"rpc.event"
// verbatim; transport-agnostic channel modelled as trait.

use std::collections::HashMap;

/// source: wire tags — verbatim.
pub const REQ: &str = "rpc.request";
pub const RES: &str = "rpc.result";
pub const EVT: &str = "rpc.event";

/// source: request envelope { type, method, input, id } — verbatim fields.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Request {
    #[serde(rename = "type")]
    pub kind: String,
    pub method: String,
    pub input: serde_json::Value,
    pub id: u64,
}

/// source: result envelope { type, result, id } — verbatim fields.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Result_ {
    #[serde(rename = "type")]
    pub kind: String,
    pub result: serde_json::Value,
    pub id: u64,
}

/// source: event envelope { type, event, data } — verbatim fields.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub kind: String,
    pub event: String,
    pub data: serde_json::Value,
}

/// source: listen() — dispatch rpc.request → post rpc.result, verbatim.
pub fn dispatch_request(
    handlers: &HashMap<String, Box<dyn Fn(serde_json::Value) -> serde_json::Value>>,
    raw: &str,
) -> Option<String> {
    let parsed: serde_json::Value = serde_json::from_str(raw).ok()?;
    if parsed.get("type")?.as_str()? != REQ {
        return None;
    }
    let method = parsed.get("method")?.as_str()?;
    let input = parsed
        .get("input")
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let id = parsed.get("id")?.as_u64()?;
    let result = handlers
        .get(method)
        .map(|f| f(input))
        .unwrap_or(serde_json::Value::Null);
    serde_json::to_string(&serde_json::json!({ "type": RES, "result": result, "id": id })).ok()
}

/// source: emit() — post rpc.event, verbatim.
pub fn emit_envelope(event: &str, data: &serde_json::Value) -> String {
    serde_json::to_string(&serde_json::json!({ "type": EVT, "event": event, "data": data }))
        .unwrap()
}

/// source: client() — pending map by id + listener sets + unsubscribe.
/// Verbatim semantics (id++ per call).
pub struct Client {
    next_id: u64,
    pending: HashMap<u64, serde_json::Value>,
    listeners: HashMap<String, Vec<usize>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            pending: HashMap::new(),
            listeners: HashMap::new(),
        }
    }

    /// source: call() — requestId = id++, verbatim envelope.
    pub fn call_envelope(&mut self, method: &str, input: &serde_json::Value) -> String {
        let id = self.next_id;
        self.next_id += 1;
        serde_json::to_string(
            &serde_json::json!({ "type": REQ, "method": method, "input": input, "id": id }),
        )
        .unwrap()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
