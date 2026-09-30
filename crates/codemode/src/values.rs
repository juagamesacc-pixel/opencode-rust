//! Port of `src/values.ts`.
//!
//! Sandbox value wrappers PLUS the owned runtime value model (`RtValue` and
//! friends). The value model lives in this leaf module so every container
//! (`SandboxMap`/`SandboxSet` entries, scopes, AST-adjacent structures) can
//! hold LIVE values with JS reference identity (clones share container ids),
//! exactly like the TS `unknown` value space. Evaluation behavior lives in
//! `interpreter_runtime`; this module owns representation only.

use crate::interpreter_model::{
    AstNode, CoercionKind, GlobalNamespaceName, PromiseMethodName, UriKind,
};
use std::collections::HashMap;

/// Eagerly started tool-call promise. Sync state-machine port of the
/// Effect-fiber-backed `SandboxPromise`.
///
/// The TS `fiber: Fiber | undefined` field becomes [`PromiseState`]; the
/// `immediate?: Effect` field becomes a pre-settled [`PromiseImmediate`].
/// `interrupted` marks a `Promise.race` loser (see `interpreter_runtime`).
#[derive(Debug, Clone)]
pub struct SandboxPromise {
    /// Whether this call was interrupted as a `Promise.race` loser.
    pub interrupted: bool,
    /// Settlement backing store.
    pub state: PromiseState,
}

/// Settlement backing a [`SandboxPromise`].
#[derive(Debug, Clone)]
pub enum PromiseState {
    /// Eagerly admitted tool call; index into the runtime tool-call ledger.
    Pending { call_index: usize },
    /// Already-settled immediate value (`Promise.resolve` / `Promise.reject`).
    Immediate { value: PromiseImmediate },
}

/// Pre-settled immediate promise payload.
#[derive(Debug, Clone)]
pub enum PromiseImmediate {
    Fulfilled(serde_json::Value),
    Rejected(String),
}

impl SandboxPromise {
    /// Mirrors `new SandboxPromise(fiber, undefined)` for a pending call.
    pub fn pending(call_index: usize) -> Self {
        SandboxPromise {
            interrupted: false,
            state: PromiseState::Pending { call_index },
        }
    }

    /// Mirrors `new SandboxPromise(undefined, immediate)` for combinators.
    pub fn immediate(value: PromiseImmediate) -> Self {
        SandboxPromise {
            interrupted: false,
            state: PromiseState::Immediate { value },
        }
    }
}

/// In-sandbox `Date` (milliseconds since epoch; may be non-finite).
#[derive(Debug, Clone)]
pub struct SandboxDate {
    /// Millisecond timestamp, mirroring `Date#getTime()`.
    pub time: f64,
}

impl SandboxDate {
    /// Mirrors `new SandboxDate(time)`.
    pub fn new(time: f64) -> Self {
        SandboxDate { time }
    }
}

/// In-sandbox `RegExp` (pattern + flags; applied host-side on dispatch).
///
/// `last_index` is shared across clones (mirroring the single host `RegExp`
/// object wrapped by the TS `SandboxRegExp`, whose `lastIndex` state survives
/// aliasing for global/sticky `test`/`exec`).
#[derive(Debug, Clone)]
pub struct SandboxRegExp {
    /// Pattern source, mirroring `RegExp#source`.
    pub pattern: String,
    /// Flags, mirroring `RegExp#flags`.
    pub flags: String,
    /// Mutable `lastIndex` shared by all aliases of this value.
    pub last_index: std::rc::Rc<std::cell::RefCell<usize>>,
}

impl SandboxRegExp {
    /// Mirrors `new SandboxRegExp(pattern, flags)` (which compiles
    /// `new RegExp(pattern, flags)` host-side; construction failure is
    /// reported at the construction site in `interpreter_runtime`).
    pub fn new(pattern: impl Into<String>, flags: impl Into<String>) -> Self {
        SandboxRegExp {
            pattern: pattern.into(),
            flags: flags.into(),
            last_index: std::rc::Rc::new(std::cell::RefCell::new(0)),
        }
    }
}

/// Fresh container identity for circular-insertion rejection and cycle
/// rendering. Clones share the id (mirroring JS reference identity, so
/// `m.set("self", m)` renders `[Circular]`); boundary copies mint new ids
/// (mirroring `copyIn` fresh objects).
static NEXT_CONTAINER_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);

/// Mints a fresh container identity.
pub fn alloc_container_id() -> u64 {
    NEXT_CONTAINER_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// Array value with JS own-properties (match results carry `index`/`groups`).
#[derive(Debug, Clone, Default)]
pub struct RtArray {
    /// Container identity (see [`NEXT_CONTAINER_ID`]).
    pub id: u64,
    pub items: Vec<RtValue>,
    pub props: Vec<(String, RtValue)>,
}

impl RtArray {
    /// Fresh array with a new identity.
    pub fn new(items: Vec<RtValue>) -> Self {
        RtArray {
            id: alloc_container_id(),
            items,
            props: vec![],
        }
    }
}

/// Plain object value (null-prototype semantics for data).
#[derive(Debug, Clone, Default)]
pub struct RtObject {
    /// Container identity (see [`NEXT_CONTAINER_ID`]).
    pub id: u64,
    pub entries: Vec<(String, RtValue)>,
}

impl RtObject {
    /// Fresh object with a new identity.
    pub fn new(entries: Vec<(String, RtValue)>) -> Self {
        RtObject {
            id: alloc_container_id(),
            entries,
        }
    }

    /// Reads an own property.
    pub fn get(&self, key: &str) -> Option<&RtValue> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Writes an own property (insertion-ordered, first-position kept).
    pub fn set(&mut self, key: &str, value: RtValue) {
        if let Some(slot) = self.entries.iter_mut().find(|(k, _)| k == key) {
            slot.1 = value;
        } else {
            self.entries.push((key.to_string(), value));
        }
    }
}

/// User function value. Mirrors `CodeModeFunction`.
#[derive(Debug, Clone)]
pub struct RtFunction {
    pub params: Vec<AstNode>,
    pub body: AstNode,
    /// Captured scope chain (cloned frames at definition time).
    pub captured: Vec<HashMap<String, RtBinding>>,
}

/// Branded error object value (`{ name, message }`). Mirrors the
/// `createErrorValue` brand (brand carried alongside the object since Rust
/// JSON values cannot hold symbols).
#[derive(Debug, Clone)]
pub struct RtErrorObj {
    pub name: String,
    pub message: String,
}

/// Every interpreter value. Data values map 1:1 to JSON; reference values
/// are opaque interpreter machinery (see `contains_runtime_reference` in
/// `interpreter_runtime`).
#[derive(Debug, Clone)]
pub enum RtValue {
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(RtArray),
    Object(RtObject),
    Function(RtFunction),
    ToolRef(Vec<String>),
    Intrinsic {
        receiver: Box<RtValue>,
        name: String,
    },
    Global(GlobalNamespaceName),
    GlobalMethod {
        namespace: String,
        name: String,
    },
    PromiseNs,
    PromiseMethod(PromiseMethodName),
    Coercion(CoercionKind),
    Uri(UriKind),
    ErrorCtor(String),
    Sandbox(SandboxValue),
    ErrorObj(RtErrorObj),
    /// Already-evaluated callee (member-call dispatch).
    Computed(Box<RtValue>),
    /// Optional-chain short-circuit marker. Mirrors `OptionalShortCircuit`.
    ShortCircuit,
}

/// Lexical binding over runtime values. Mirrors `Binding`.
#[derive(Debug, Clone)]
pub struct RtBinding {
    pub mutable: bool,
    pub value: RtValue,
    pub initialized: bool,
}

/// JS `SameValueZero` equality over runtime values: `NaN` equals `NaN`;
/// arrays/objects compare by container identity; sandbox wrappers compare by
/// value (dates by time, regexps by pattern+flags, maps/sets by identity,
/// URLs by href, params by pairs, promises by settlement identity).
pub fn same_value_zero(left: &RtValue, right: &RtValue) -> bool {
    match (left, right) {
        (RtValue::Undefined, RtValue::Undefined) => true,
        (RtValue::Null, RtValue::Null) => true,
        (RtValue::Bool(a), RtValue::Bool(b)) => a == b,
        (RtValue::Number(a), RtValue::Number(b)) => {
            if a.is_nan() && b.is_nan() {
                return true;
            }
            a == b
        }
        (RtValue::Str(a), RtValue::Str(b)) => a == b,
        (RtValue::Array(a), RtValue::Array(b)) => a.id == b.id,
        (RtValue::Object(a), RtValue::Object(b)) => a.id == b.id,
        (RtValue::Sandbox(a), RtValue::Sandbox(b)) => match (a, b) {
            (SandboxValue::Date(x), SandboxValue::Date(y)) => {
                x.time == y.time || (x.time.is_nan() && y.time.is_nan())
            }
            (SandboxValue::RegExp(x), SandboxValue::RegExp(y)) => {
                x.pattern == y.pattern && x.flags == y.flags
            }
            (SandboxValue::Map(x), SandboxValue::Map(y)) => x.id == y.id,
            (SandboxValue::Set(x), SandboxValue::Set(y)) => x.id == y.id,
            (SandboxValue::Url(x), SandboxValue::Url(y)) => x.href == y.href,
            (SandboxValue::UrlSearchParams(x), SandboxValue::UrlSearchParams(y)) => {
                x.pairs == y.pairs
            }
            (SandboxValue::Promise(x), SandboxValue::Promise(y)) => {
                promise_identity(x) == promise_identity(y)
            }
            _ => false,
        },
        _ => false,
    }
}

fn promise_identity(promise: &SandboxPromise) -> (u8, usize, String) {
    match &promise.state {
        PromiseState::Pending { call_index } => (0, *call_index, String::new()),
        PromiseState::Immediate { value } => match value {
            PromiseImmediate::Fulfilled(v) => (1, 0, serde_json::to_string(v).unwrap_or_default()),
            PromiseImmediate::Rejected(message) => (2, 0, message.clone()),
        },
    }
}

/// In-sandbox `Map` (insertion-ordered entries holding LIVE values with JS
/// key identity).
#[derive(Debug, Clone)]
pub struct SandboxMap {
    /// Container identity (shared across clones).
    pub id: u64,
    /// Entries in insertion order.
    pub entries: Vec<(RtValue, RtValue)>,
}

impl SandboxMap {
    /// Fresh empty map with a new identity.
    pub fn new() -> Self {
        SandboxMap {
            id: alloc_container_id(),
            entries: vec![],
        }
    }
}

impl Default for SandboxMap {
    fn default() -> Self {
        Self::new()
    }
}

/// In-sandbox `Set` (insertion-ordered members holding LIVE values).
#[derive(Debug, Clone)]
pub struct SandboxSet {
    /// Container identity (shared across clones).
    pub id: u64,
    /// Members in insertion order.
    pub members: Vec<RtValue>,
}

impl SandboxSet {
    /// Fresh empty set with a new identity.
    pub fn new() -> Self {
        SandboxSet {
            id: alloc_container_id(),
            members: vec![],
        }
    }
}

impl Default for SandboxSet {
    fn default() -> Self {
        Self::new()
    }
}

/// In-sandbox `URLSearchParams` (insertion-ordered pairs).
#[derive(Debug, Clone, Default)]
pub struct SandboxURLSearchParams {
    /// Pairs in insertion order.
    pub pairs: Vec<(String, String)>,
}

impl SandboxURLSearchParams {
    /// Mirrors `new SandboxURLSearchParams(params)`.
    pub fn new(pairs: Vec<(String, String)>) -> Self {
        SandboxURLSearchParams { pairs }
    }
}

/// In-sandbox `URL` (hosts its own `searchParams` identity like the TS
/// constructor wrapping `url.searchParams`).
#[derive(Debug, Clone)]
pub struct SandboxURL {
    /// Serialized href.
    pub href: String,
    /// Owned query pairs (mirrors `url.searchParams`).
    pub search_params: SandboxURLSearchParams,
}

impl SandboxURL {
    /// Mirrors `new SandboxURL(url)`.
    pub fn new(href: impl Into<String>, pairs: Vec<(String, String)>) -> Self {
        SandboxURL {
            href: href.into(),
            search_params: SandboxURLSearchParams { pairs },
        }
    }
}

/// All sandbox value wrappers, mirroring the `isSandboxValue` union:
/// `SandboxDate | SandboxRegExp | SandboxMap | SandboxSet | SandboxURL |
/// SandboxURLSearchParams`. Promises are deliberately NOT members (an
/// un-awaited promise never crosses a data checkpoint as `{}`).
#[derive(Debug, Clone)]
pub enum SandboxValue {
    Date(SandboxDate),
    RegExp(SandboxRegExp),
    Map(SandboxMap),
    Set(SandboxSet),
    Url(SandboxURL),
    UrlSearchParams(SandboxURLSearchParams),
    Promise(SandboxPromise),
}

/// Mirrors `isSandboxValue(value)`.
///
/// NOTE: the TS predicate excludes `SandboxPromise`; promise detection at
/// data checkpoints uses the dedicated un-awaited-Promise diagnostic
/// (`copyIn` in `tool_runtime`). This helper therefore reports wrapper
/// membership only; callers needing the TS-exact predicate should use
/// [`is_sandbox_data_value`].
pub fn is_sandbox_value(value: &SandboxValue) -> bool {
    matches!(
        value,
        SandboxValue::Date(_)
            | SandboxValue::RegExp(_)
            | SandboxValue::Map(_)
            | SandboxValue::Set(_)
            | SandboxValue::Url(_)
            | SandboxValue::UrlSearchParams(_)
    )
}

/// TS-exact `isSandboxValue`: true for the six data wrappers, false for
/// promises and everything else. The interpreter runtime calls this with its
/// own value representation; this free function documents the membership.
pub fn is_sandbox_data_value(kind: SandboxDataKind) -> bool {
    !matches!(kind, SandboxDataKind::Promise)
}

/// Nominal kind tag for TS-exact `isSandboxValue` membership checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandboxDataKind {
    Date,
    RegExp,
    Map,
    Set,
    Url,
    UrlSearchParams,
    Promise,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sandbox_promise_defaults_to_not_interrupted() {
        let p = SandboxPromise::pending(0);
        assert!(!p.interrupted);
    }

    #[test]
    fn is_sandbox_value_covers_six_wrappers() {
        assert!(is_sandbox_value(&SandboxValue::Date(SandboxDate::new(0.0))));
        assert!(is_sandbox_value(&SandboxValue::RegExp(SandboxRegExp::new(
            "a", ""
        ))));
        assert!(is_sandbox_value(&SandboxValue::Map(SandboxMap::default())));
        assert!(is_sandbox_value(&SandboxValue::Set(SandboxSet::default())));
        assert!(is_sandbox_value(&SandboxValue::Url(SandboxURL::new(
            "https://example.test/",
            vec![]
        ))));
        assert!(is_sandbox_value(&SandboxValue::UrlSearchParams(
            SandboxURLSearchParams::default()
        )));
        assert!(!is_sandbox_value(&SandboxValue::Promise(
            SandboxPromise::pending(0)
        )));
    }
}
