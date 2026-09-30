//! Port of `src/openapi/types.ts` (16 types).
//!
//! Pure types only: `Document`, `Operation`, `SecurityScheme`, `Credential`,
//! `AuthResolver`, `Options`, `Skipped`, `Tools`, `Result`, `Parsed`,
//! `InputLocation`, `InputField`, `Body`, `OperationInput`,
//! `SecurityRequirement`, `Plan`, `AppliedAuth`.
//!
//! The Effect `AuthResolver` (returning `Effect<Credential|undefined,
//! unknown>`) becomes a sync closure returning
//! `Result<Option<Credential>, String>` with identical observable semantics:
//! `None` means unavailable (try next OR alternative); `Err` aborts the call.

use crate::tool::JsonSchema;
use serde_json::Value;
use std::collections::BTreeMap;

/// A parsed OpenAPI 3.x document. YAML must be parsed by the host.
/// Mirrors `Document`.
pub type Document = BTreeMap<String, Value>;

/// The operation identity handed to auth resolution and errors.
/// Mirrors `Operation`.
#[derive(Debug, Clone)]
pub struct Operation {
    pub operation_id: Option<String>,
    pub method: String,
    pub path: String,
    pub summary: Option<String>,
    pub description: Option<String>,
}

/// A resolved OpenAPI security scheme from `components.securitySchemes`.
/// Mirrors `SecurityScheme`.
#[derive(Debug, Clone)]
pub enum SecurityScheme {
    ApiKey {
        name: String,
        location: ApiKeyLocation,
    },
    Http {
        scheme: String,
    },
    OAuth2,
    OpenIdConnect,
}

/// Carrier of an `apiKey` scheme. Mirrors `"header" | "query" | "cookie"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiKeyLocation {
    Header,
    Query,
    Cookie,
}

/// Credential material returned by a host auth resolver. Mirrors
/// `Credential` verbatim (incl. the `header` escape hatch for nonstandard
/// schemes).
#[derive(Debug, Clone)]
pub enum Credential {
    Bearer { token: String },
    Basic { username: String, password: String },
    ApiKey { value: String },
    Header { name: String, value: String },
}

/// Auth resolution context. Mirrors the `AuthResolver` argument.
#[derive(Debug, Clone)]
pub struct AuthContext {
    pub name: String,
    pub definition: SecurityScheme,
    pub scopes: Vec<String>,
    pub operation: Operation,
}

/// Resolves credential material for one named security scheme at call time.
/// `Ok(None)` means unavailable (try the next OR alternative); `Err` aborts
/// the call rather than falling through. Mirrors `AuthResolver`.
///
/// Stored in an `Arc` so execution plans (one per operation) can share the
/// host resolver with identical observable semantics.
pub type AuthResolver =
    std::sync::Arc<dyn Fn(&AuthContext) -> Result<Option<Credential>, String> + Send + Sync>;

/// Adapter options. Mirrors `Options`.
pub struct Options {
    pub spec: serde_json::Value,
    /// Overrides all document, path, and operation `servers`. Required when
    /// no applicable absolute server URL exists.
    pub base_url: Option<String>,
    /// Host credential resolution, keyed by security scheme name.
    pub auth: Option<AuthConfig>,
    /// Static headers on every request. Not model-visible; declared header
    /// params may override them, auth always wins.
    pub headers: BTreeMap<String, String>,
}

impl std::fmt::Debug for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Options")
            .field("spec", &self.spec)
            .field("base_url", &self.base_url)
            .field("headers", &self.headers)
            .finish_non_exhaustive()
    }
}

/// Host auth configuration. Mirrors `{ readonly resolve: AuthResolver }`.
pub struct AuthConfig {
    pub resolve: AuthResolver,
}

impl std::fmt::Debug for AuthConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthConfig").finish_non_exhaustive()
    }
}

/// An operation that could not be represented as a tool, and why.
/// Mirrors `Skipped`.
#[derive(Debug, Clone)]
pub struct Skipped {
    pub method: String,
    pub path: String,
    pub reason: String,
}

/// Generated tool subtree. Mirrors `Tools` (insertion-ordered like the
/// source document walk).
#[derive(Debug, Default)]
pub struct Tools {
    pub entries: Vec<(String, ToolNode)>,
}

impl Tools {
    /// Looks up a child by name.
    pub fn get(&self, key: &str) -> Option<&ToolNode> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Looks up a child by name, mutably.
    pub fn get_mut(&mut self, key: &str) -> Option<&mut ToolNode> {
        self.entries
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    /// Inserts or replaces a child, preserving first-insertion position.
    pub fn insert(&mut self, key: String, value: ToolNode) {
        match self.entries.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) => slot.1 = value,
            None => self.entries.push((key, value)),
        }
    }

    /// Removes a child by name.
    pub fn remove(&mut self, key: &str) {
        self.entries.retain(|(k, _)| k != key);
    }

    /// Whether a child exists.
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }

    /// Child names in declaration order.
    pub fn keys(&self) -> Vec<String> {
        self.entries.iter().map(|(k, _)| k.clone()).collect()
    }
}

/// One node of the generated subtree: tool or nested namespace.
pub enum ToolNode {
    Tool(crate::tool::Definition),
    Namespace(Tools),
}

impl std::fmt::Debug for ToolNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolNode::Tool(d) => f.debug_tuple("Tool").field(d).finish(),
            ToolNode::Namespace(_) => f.debug_tuple("Namespace").finish_non_exhaustive(),
        }
    }
}

/// Adapter result. Mirrors `Result` (`tools` + `skipped`).
#[derive(Debug)]
pub struct AdapterResult {
    /// Tool subtree; the host places it under a key in its `tools` tree.
    pub tools: Tools,
    pub skipped: Vec<Skipped>,
}

/// Parse outcome. Mirrors `Parsed<T>`.
#[derive(Debug, Clone)]
pub enum Parsed<T> {
    Ok(T),
    Err(String),
}

impl<T> Parsed<T> {
    /// Mirrors the `{ ok: true, value }` shape.
    pub fn ok(value: T) -> Self {
        Parsed::Ok(value)
    }

    /// Mirrors the `{ ok: false, reason }` shape.
    pub fn err(reason: impl Into<String>) -> Self {
        Parsed::Err(reason.into())
    }

    /// Mirrors `if (!x.ok) return x` propagation.
    pub fn is_ok(&self) -> bool {
        matches!(self, Parsed::Ok(_))
    }
}

/// Model-visible field location. Mirrors `InputLocation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputLocation {
    Path,
    Query,
    Header,
    Body,
}

impl InputLocation {
    /// Verbatim wire spelling.
    pub fn as_str(&self) -> &'static str {
        match self {
            InputLocation::Path => "path",
            InputLocation::Query => "query",
            InputLocation::Header => "header",
            InputLocation::Body => "body",
        }
    }
}

/// One model-visible input field. Mirrors `InputField`.
#[derive(Debug, Clone)]
pub struct InputField {
    /// Model-visible field name after cross-location collision handling.
    pub input_name: String,
    /// Original parameter or body-property name used on the wire.
    pub name: String,
    pub location: InputLocation,
    pub required: bool,
    pub schema: JsonSchema,
    pub style: Option<ParamStyle>,
    pub explode: Option<bool>,
}

/// Parameter serialization style. Mirrors `"simple" | "form" |
/// "deepObject" | undefined`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamStyle {
    Simple,
    Form,
    DeepObject,
}

/// Request-body shape. Mirrors `Body`.
#[derive(Debug, Clone)]
pub struct Body {
    pub required: bool,
    pub mode: BodyMode,
    pub media_type: String,
}

/// Body mode. Mirrors `"object" | "value"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyMode {
    Object,
    Value,
}

/// Parsed operation input. Mirrors `OperationInput`.
#[derive(Debug, Clone)]
pub struct OperationInput {
    pub fields: Vec<InputField>,
    pub body: Option<Body>,
}

/// One OR alternative: scheme name -> required scopes. Empty = unauthenticated
/// acceptable. Mirrors `SecurityRequirement` (insertion-ordered).
#[derive(Debug, Clone, Default)]
pub struct SecurityRequirement {
    pub entries: Vec<(String, Vec<String>)>,
}

impl SecurityRequirement {
    /// Empty requirement.
    pub fn new() -> Self {
        SecurityRequirement { entries: vec![] }
    }

    /// Inserts or replaces a scheme's scopes.
    pub fn insert(&mut self, name: String, scopes: Vec<String>) {
        match self.entries.iter_mut().find(|(k, _)| *k == name) {
            Some(slot) => slot.1 = scopes,
            None => self.entries.push((name, scopes)),
        }
    }

    /// Looks up a scheme's scopes.
    pub fn get(&self, name: &str) -> Option<&Vec<String>> {
        self.entries.iter().find(|(k, _)| k == name).map(|(_, v)| v)
    }

    /// Scheme names in declaration order.
    pub fn keys(&self) -> Vec<&String> {
        self.entries.iter().map(|(k, _)| k).collect()
    }

    /// Whether no schemes are required.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Iterates scheme/scopes pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<String>)> {
        self.entries.iter().map(|(k, v)| (k, v))
    }
}

/// Execution plan for one operation. Mirrors `Plan`.
pub struct Plan {
    pub operation: Operation,
    pub url: String,
    pub fields: Vec<InputField>,
    pub body: Option<Body>,
    pub security: Vec<SecurityRequirement>,
    pub schemes: BTreeMap<String, SecurityScheme>,
    pub auth: Option<AuthConfig>,
    pub headers: BTreeMap<String, String>,
}

impl std::fmt::Debug for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Plan")
            .field("url", &self.url)
            .field("fields", &self.fields)
            .field("security", &self.security)
            .field("headers", &self.headers)
            .finish_non_exhaustive()
    }
}

/// Applied credentials. Mirrors `AppliedAuth`.
#[derive(Debug, Clone, Default)]
pub struct AppliedAuth {
    pub headers: BTreeMap<String, String>,
    pub query: BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_location_spellings_verbatim() {
        assert_eq!(InputLocation::Path.as_str(), "path");
        assert_eq!(InputLocation::Query.as_str(), "query");
        assert_eq!(InputLocation::Header.as_str(), "header");
        assert_eq!(InputLocation::Body.as_str(), "body");
    }
}
