//! Port of `src/openapi/runtime.ts`.
//!
//! `invoke(plan, input)` execution. The Effect `HttpClient.HttpClient`
//! service becomes the sync [`HttpTransport`] handle supplied by the host
//! (same placement: required at call time, never model-visible). Network
//! reads stay bounded (`50 MiB`), error bodies truncate at 1,024 chars, and
//! every expected encoding/transport/decoding failure maps to a model-safe
//! [`ToolError`](crate::tool_error::ToolError) with verbatim strings.

use crate::openapi_types::{
    AppliedAuth, AuthContext, BodyMode, Credential, InputLocation, ParamStyle, Plan,
};
use crate::tool::ToolFailure;
use serde_json::Value;
use std::collections::BTreeMap;
use std::sync::Arc;

/// Alias to reduce type complexity for clippy::type_complexity.
pub type HttpHandler = Arc<dyn Fn(&HttpRequest) -> Result<HttpResponse, String> + Send + Sync>;

/// Maximum error-body chars in failure messages. Mirrors `maxErrorBodyChars`.
pub const MAX_ERROR_BODY_CHARS: usize = 1_024;

/// Maximum response body bytes. Mirrors `maxResponseBodyBytes` (50 MiB).
pub const MAX_RESPONSE_BODY_BYTES: usize = 50 * 1024 * 1024;

/// Outgoing HTTP request.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: BTreeMap<String, String>,
    /// JSON body bytes + declared media type.
    pub body: Option<(Vec<u8>, String)>,
}

/// Incoming HTTP response.
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

/// Host HTTP transport. Mirrors the `HttpClient.HttpClient` service
/// requirement: tools require it at call time. `Err` = transport failure.
#[derive(Clone)]
pub struct HttpTransport {
    handler: Option<HttpHandler>,
}

impl std::fmt::Debug for HttpTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpTransport")
            .field("present", &self.handler.is_some())
            .finish()
    }
}

impl HttpTransport {
    /// Builds a transport from a host handler.
    pub fn new(
        handler: impl Fn(&HttpRequest) -> Result<HttpResponse, String> + Send + Sync + 'static,
    ) -> Self {
        HttpTransport {
            handler: Some(Arc::new(handler)),
        }
    }

    /// Absent transport (every call fails with a transport error).
    pub fn none() -> Self {
        HttpTransport { handler: None }
    }
}

/// Execution failure: always a model-safe [`crate::tool_error::ToolError`].
#[derive(Debug, Clone)]
pub struct OpenApiError {
    pub tool_error: crate::tool_error::ToolError,
}

impl OpenApiError {
    /// Mirrors `toolError(message, cause?)`.
    pub fn tool_error(message: impl Into<String>) -> Self {
        OpenApiError {
            tool_error: crate::tool_error::tool_error(message, None),
        }
    }

    /// Converts into the tool-definition failure channel.
    pub fn into_tool_failure(self) -> ToolFailure {
        ToolFailure::ToolError(self.tool_error)
    }
}

impl std::fmt::Display for OpenApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.tool_error.message)
    }
}

impl std::error::Error for OpenApiError {}

/// Executes one planned operation. Mirrors `invoke(plan, input)`.
pub fn invoke(
    plan: &Plan,
    input: &Value,
    transport: &HttpTransport,
) -> Result<Value, OpenApiError> {
    let value: BTreeMap<String, Value> = match input.as_object() {
        Some(map) => map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
        None => BTreeMap::new(),
    };
    let mut request = build_request(plan, &value)?;
    let auth = resolve_auth(plan)?;
    for (name, item) in &auth.query {
        request.url = append_url_param(&request.url, name, item);
    }
    for (name, item) in &auth.headers {
        request.headers.insert(name.clone(), item.clone());
    }
    let handler = transport.handler.as_ref().ok_or_else(|| {
        OpenApiError::tool_error(format!(
            "{} {} failed: transport error",
            plan.operation.method, plan.operation.path
        ))
    })?;
    let response = handler(&request).map_err(|cause| OpenApiError {
        tool_error: crate::tool_error::tool_error(
            format!(
                "{} {} failed: transport error",
                plan.operation.method, plan.operation.path
            ),
            Some(cause),
        ),
    })?;
    if response.body.len() > MAX_RESPONSE_BODY_BYTES {
        return Err(OpenApiError::tool_error(format!(
            "{} {} response exceeds 50 MiB.",
            plan.operation.method, plan.operation.path
        )));
    }
    let text = String::from_utf8_lossy(&response.body).into_owned();
    let media_type = response
        .headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
        .map(|(_, v)| {
            v.split(';')
                .next()
                .unwrap_or("")
                .trim()
                .to_ascii_lowercase()
        });
    let is_json = media_type
        .as_deref()
        .is_some_and(|mt| mt == "application/json" || mt.ends_with("+json"));
    let decoded: Option<Value> = if text.is_empty() {
        Some(Value::Null)
    } else if is_json {
        serde_json::from_str(&text).ok()
    } else {
        None
    };
    let parsed: Value = if text.is_empty() {
        Value::Null
    } else if is_json {
        match decoded.clone() {
            Some(v) => v,
            None => Value::String(text.clone()),
        }
    } else {
        Value::String(text.clone())
    };
    if !(200..300).contains(&response.status) {
        let rendered = match &parsed {
            Value::String(s) => s.clone(),
            other => serde_json::to_string(other).unwrap_or_default(),
        };
        let summary = if rendered.is_empty() || rendered == "null" {
            "no response body".to_string()
        } else if rendered.len() > MAX_ERROR_BODY_CHARS {
            format!("{}...", &rendered[..MAX_ERROR_BODY_CHARS])
        } else {
            rendered
        };
        return Err(OpenApiError::tool_error(format!(
            "{} {} failed with HTTP {}: {}",
            plan.operation.method, plan.operation.path, response.status, summary
        )));
    }
    if is_json && decoded.is_none() && !text.is_empty() {
        return Err(OpenApiError::tool_error(format!(
            "{} {} returned malformed JSON.",
            plan.operation.method, plan.operation.path
        )));
    }
    Ok(parsed)
}

fn build_request(
    plan: &Plan,
    input: &BTreeMap<String, Value>,
) -> Result<HttpRequest, OpenApiError> {
    // Validate every model-controlled value before auth resolution.
    let url = build_url(plan, input).map_err(OpenApiError::tool_error)?;
    if let Some(missing) = plan.fields.iter().find(|field| {
        field.required
            && field.location != InputLocation::Path
            && !input.contains_key(&field.input_name)
    }) {
        let label = if missing.location == InputLocation::Body {
            "body field".to_string()
        } else {
            format!("{} parameter", missing.location.as_str())
        };
        return Err(OpenApiError::tool_error(format!(
            "Missing required {} '{}'.",
            label, missing.input_name
        )));
    }
    let mut request = HttpRequest {
        method: plan.operation.method.clone(),
        url,
        headers: BTreeMap::new(),
        body: None,
    };
    for field in &plan.fields {
        if field.location != InputLocation::Query {
            continue;
        }
        let item = match input.get(&field.input_name) {
            Some(v) => v,
            None => continue,
        };
        serialize_query(&mut request, plan, field, item)?;
    }
    // Host headers first, then declared header parameters.
    for (k, v) in &plan.headers {
        request.headers.insert(k.clone(), v.clone());
    }
    for field in &plan.fields {
        if field.location != InputLocation::Header {
            continue;
        }
        let item = match input.get(&field.input_name) {
            Some(v) => v,
            None => continue,
        };
        let serialized =
            serialize_simple(field, item, |s| s.to_string()).map_err(OpenApiError::tool_error)?;
        request.headers.insert(field.name.clone(), serialized);
    }
    if plan.body.as_ref().map(|b| b.mode) == Some(BodyMode::Value) {
        let field = plan
            .fields
            .iter()
            .find(|f| f.location == InputLocation::Body);
        let body = field.and_then(|f| input.get(&f.input_name));
        if let Some(body) = body {
            let media_type = plan
                .body
                .as_ref()
                .map(|b| b.media_type.clone())
                .unwrap_or_default();
            let bytes = serde_json::to_vec(body).map_err(|cause| OpenApiError {
                tool_error: crate::tool_error::tool_error(
                    format!(
                        "Invalid JSON body for {} {}.",
                        plan.operation.method, plan.operation.path
                    ),
                    Some(cause.to_string()),
                ),
            })?;
            request.body = Some((bytes, media_type.clone()));
            request
                .headers
                .insert("content-type".to_string(), media_type);
        }
    }
    if plan.body.as_ref().map(|b| b.mode) == Some(BodyMode::Object) {
        let mut entries = serde_json::Map::new();
        for field in &plan.fields {
            if field.location != InputLocation::Body {
                continue;
            }
            if let Some(item) = input.get(&field.input_name) {
                entries.insert(field.name.clone(), item.clone());
            }
        }
        let required = plan.body.as_ref().map(|b| b.required).unwrap_or(false);
        if required || !entries.is_empty() {
            let media_type = plan
                .body
                .as_ref()
                .map(|b| b.media_type.clone())
                .unwrap_or_default();
            let bytes =
                serde_json::to_vec(&Value::Object(entries)).map_err(|cause| OpenApiError {
                    tool_error: crate::tool_error::tool_error(
                        format!(
                            "Invalid JSON body for {} {}.",
                            plan.operation.method, plan.operation.path
                        ),
                        Some(cause.to_string()),
                    ),
                })?;
            request.body = Some((bytes, media_type.clone()));
            request
                .headers
                .insert("content-type".to_string(), media_type);
        }
    }
    Ok(request)
}

fn resolve_auth(plan: &Plan) -> Result<AppliedAuth, OpenApiError> {
    if plan.security.is_empty() {
        return Ok(AppliedAuth::default());
    }
    let mut unavailable: Vec<String> = vec![];
    for requirement in &plan.security {
        let names: Vec<&String> = requirement.keys();
        if names.is_empty() {
            return Ok(AppliedAuth::default());
        }
        let mut credentials: Vec<(String, crate::openapi_types::SecurityScheme, Credential)> =
            vec![];
        let mut skip = false;
        for name in names {
            let scheme = match plan.schemes.get(name) {
                Some(s) => s.clone(),
                None => {
                    unavailable.push(name.clone());
                    skip = true;
                    break;
                }
            };
            let resolver = match plan.auth.as_ref() {
                Some(a) => a,
                None => {
                    unavailable.push(name.clone());
                    skip = true;
                    break;
                }
            };
            let credential = (resolver.resolve)(&AuthContext {
                name: name.clone(),
                definition: scheme.clone(),
                scopes: requirement.get(name).cloned().unwrap_or_default(),
                operation: plan.operation.clone(),
            })
            .map_err(|_| {
                OpenApiError::tool_error(format!(
                    "{} {} requires authentication.",
                    plan.operation.method, plan.operation.path
                ))
            })?;
            match credential {
                None => {
                    unavailable.push(name.clone());
                    skip = true;
                    break;
                }
                Some(credential) => credentials.push((name.clone(), scheme, credential)),
            }
        }
        if skip {
            continue;
        }
        return apply_credentials(&credentials).map_err(OpenApiError::tool_error);
    }
    let mut unique: Vec<String> = vec![];
    for name in unavailable {
        if !unique.contains(&name) {
            unique.push(name);
        }
    }
    Err(OpenApiError::tool_error(format!(
        "{} {} requires authentication; no credential available for: {}.",
        plan.operation.method,
        plan.operation.path,
        unique.join(", ")
    )))
}

fn apply_credentials(
    credentials: &[(String, crate::openapi_types::SecurityScheme, Credential)],
) -> Result<AppliedAuth, String> {
    use crate::openapi_types::SecurityScheme;
    let mut headers: BTreeMap<String, String> = BTreeMap::new();
    let mut query: BTreeMap<String, String> = BTreeMap::new();
    let mut add = |carrier: &str, name: &str, value: String| -> Result<(), String> {
        let target = if carrier == "header" {
            &mut headers
        } else {
            &mut query
        };
        if target.contains_key(name) {
            return Err(format!(
                "Authentication resolves multiple credentials for {} '{}'.",
                carrier, name
            ));
        }
        target.insert(name.to_string(), value);
        Ok(())
    };
    for (name, definition, credential) in credentials {
        match credential {
            Credential::Bearer { token } => {
                add("header", "authorization", format!("Bearer {}", token))?;
            }
            Credential::Basic { username, password } => {
                // Base64 without a base64 dependency (credentials are
                // typically ASCII; UTF-8 bytes encoded per RFC 7617).
                add(
                    "header",
                    "authorization",
                    format!(
                        "Basic {}",
                        base64_encode(format!("{}:{}", username, password).as_bytes())
                    ),
                )?;
            }
            Credential::Header { name, value } => {
                add("header", &name.to_ascii_lowercase(), value.clone())?;
            }
            Credential::ApiKey { value } => {
                let (carrier, parameter) = match definition {
                    SecurityScheme::ApiKey { name, location } => match location {
                        crate::openapi_types::ApiKeyLocation::Header => {
                            ("header", name.to_ascii_lowercase())
                        }
                        crate::openapi_types::ApiKeyLocation::Query => ("query", name.clone()),
                        crate::openapi_types::ApiKeyLocation::Cookie => {
                            return Err(format!(
                                "Cookie authentication '{}' is not supported.",
                                name
                            ));
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Security scheme '{}' is not an apiKey scheme; resolve a bearer, basic, or header credential for it.",
                            name
                        ));
                    }
                };
                add(carrier, &parameter, value.clone())?;
            }
        }
    }
    Ok(AppliedAuth { headers, query })
}

fn base64_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[((triple >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[((triple >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[(triple & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn build_url(plan: &Plan, input: &BTreeMap<String, Value>) -> Result<String, String> {
    let mut url = plan.url.clone();
    for field in &plan.fields {
        if field.location != InputLocation::Path {
            continue;
        }
        let item = match input.get(&field.input_name) {
            Some(v) => v,
            None => {
                return Err(format!(
                    "Missing required path parameter '{}'.",
                    field.input_name
                ));
            }
        };
        let field_value = serialize_simple(field, item, |s| percent_encode_path_segment(&s))?;
        // '.'/'..' survive encoding and URL normalization collapses them,
        // letting a model-supplied value retarget the request.
        if field_value.is_empty() || field_value == "." || field_value == ".." {
            return Err(format!("Invalid path parameter '{}'.", field.input_name));
        }
        url = url.replace(&format!("{{{}}}", field.name), &field_value);
    }
    if let Some(unresolved) = find_unresolved_template(&url) {
        return Err(format!("Unresolved path parameter {}.", unresolved));
    }
    Ok(url)
}

fn find_unresolved_template(url: &str) -> Option<String> {
    let start = url.find('{')?;
    let rest = &url[start..];
    let end = rest.find('}')?;
    let candidate = &rest[..end + 1];
    if candidate.contains('{') && !candidate[1..].contains('{') {
        Some(candidate.to_string())
    } else {
        None
    }
}

fn percent_encode_path_segment(s: &str) -> String {
    // encodeURIComponent + `!'()*` upper-hex escaping (verbatim).
    let mut out = String::new();
    for b in s.bytes() {
        let c = b as char;
        if c.is_ascii_alphanumeric() || "-_.!~*'()".contains(c) {
            if "!'()*".contains(c) {
                out.push_str(&format!("%{:02X}", b));
            } else {
                out.push(c);
            }
        } else {
            // UTF-8 multi-byte passthrough: encode each byte.
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}

fn serialize_simple(
    field: &crate::openapi_types::InputField,
    value: &Value,
    encode: impl Fn(String) -> String,
) -> Result<String, String> {
    fn scalar(
        field: &crate::openapi_types::InputField,
        item: &Value,
        encode: &impl Fn(String) -> String,
    ) -> Result<String, String> {
        match item {
            Value::String(s) => Ok(encode(s.clone())),
            Value::Number(n) => Ok(encode(n.to_string())),
            Value::Bool(b) => Ok(encode(b.to_string())),
            Value::Null => Ok(encode("null".to_string())),
            _ => Err(format!(
                "Parameter '{}' contains an unsupported nested value.",
                field.input_name
            )),
        }
    }
    if let Some(items) = value.as_array() {
        let mut rendered = vec![];
        for item in items {
            rendered.push(scalar(field, item, &encode)?);
        }
        return Ok(rendered.join(","));
    }
    if let Some(obj) = value.as_object() {
        let explode = field.explode.unwrap_or(false);
        let mut entries = vec![];
        for (name, item) in obj {
            let rendered = scalar(field, item, &encode)?;
            if explode {
                entries.push(format!("{}={}", encode(name.clone()), rendered));
            } else {
                entries.push(encode(name.clone()));
                entries.push(rendered);
            }
        }
        return Ok(entries.join(","));
    }
    scalar(field, value, &encode)
}

fn serialize_query(
    request: &mut HttpRequest,
    plan: &Plan,
    field: &crate::openapi_types::InputField,
    value: &Value,
) -> Result<(), OpenApiError> {
    let fail = |message: String| OpenApiError::tool_error(message);
    if field.style == Some(ParamStyle::DeepObject) {
        let obj = match value.as_object() {
            Some(o) => o,
            None => {
                return Err(fail(format!(
                    "Deep-object parameter '{}' must be an object.",
                    field.input_name
                )))
            }
        };
        for (name, item) in obj {
            // Mirrors `item === undefined || (item !== null && typeof item ===
            // "object")`: JSON has no undefined; null serializes as "null".
            if item.is_object() || item.is_array() {
                return Err(fail(format!(
                    "Deep-object parameter '{}' contains an unsupported nested value.",
                    field.input_name
                )));
            }
            let rendered = match item {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
                _ => {
                    return Err(fail(format!(
                        "Deep-object parameter '{}' contains an unsupported nested value.",
                        field.input_name
                    )))
                }
            };
            request.url = append_url_param(
                &request.url,
                &format!("{}[{}]", field.name, name),
                &rendered,
            );
        }
        return Ok(());
    }
    if let Some(items) = value.as_array() {
        let rendered = serialize_simple(field, value, |s| s).map_err(fail)?;
        if field.explode != Some(true) {
            request.url = append_url_param(&request.url, &field.name, &rendered);
            return Ok(());
        }
        for item in items {
            if item.is_object() || item.is_array() {
                return Err(fail(format!(
                    "Query parameter '{}' contains an unsupported nested value.",
                    field.input_name
                )));
            }
            let text = match item {
                Value::String(s) => s.clone(),
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                Value::Null => "null".to_string(),
                _ => unreachable!(),
            };
            request.url = append_url_param(&request.url, &field.name, &text);
        }
        let _ = plan;
        return Ok(());
    }
    if let Some(obj) = value.as_object() {
        if field.explode == Some(true) {
            for (name, item) in obj {
                if item.is_object() || item.is_array() {
                    return Err(fail(format!(
                        "Query parameter '{}' contains an unsupported nested value.",
                        field.input_name
                    )));
                }
                let text = match item {
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    Value::Null => "null".to_string(),
                    _ => unreachable!(),
                };
                request.url = append_url_param(&request.url, name, &text);
            }
            return Ok(());
        }
    }
    let rendered = serialize_simple(field, value, |s| s).map_err(fail)?;
    request.url = append_url_param(&request.url, &field.name, &rendered);
    Ok(())
}

fn append_url_param(url: &str, name: &str, value: &str) -> String {
    let sep = if url.contains('?') { "&" } else { "?" };
    format!(
        "{}{}{}={}",
        url,
        sep,
        percent_encode_query_component(name),
        percent_encode_query_component(value)
    )
}

fn percent_encode_query_component(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi_types::{Operation, Plan};
    use std::collections::BTreeMap;

    fn test_plan() -> Plan {
        Plan {
            operation: Operation {
                operation_id: None,
                method: "GET".to_string(),
                path: "/users".to_string(),
                summary: None,
                description: None,
            },
            url: "https://api.example.test/users".to_string(),
            fields: vec![],
            body: None,
            security: vec![],
            schemes: BTreeMap::new(),
            auth: None,
            headers: BTreeMap::new(),
        }
    }

    #[test]
    fn transport_errors_map_to_tool_error() {
        let plan = test_plan();
        let err = invoke(&plan, &Value::Null, &HttpTransport::none()).unwrap_err();
        assert_eq!(err.tool_error.message, "GET /users failed: transport error");
    }

    #[test]
    fn http_errors_truncate_bodies() {
        let plan = test_plan();
        let transport = HttpTransport::new(|_| {
            Ok(HttpResponse {
                status: 500,
                headers: BTreeMap::new(),
                body: b"boom".to_vec(),
            })
        });
        let err = invoke(&plan, &Value::Null, &transport).unwrap_err();
        assert_eq!(
            err.tool_error.message,
            "GET /users failed with HTTP 500: boom"
        );
    }

    #[test]
    fn malformed_json_rejected_verbatim() {
        let plan = test_plan();
        let transport = HttpTransport::new(|_| {
            Ok(HttpResponse {
                status: 200,
                headers: [("content-type".to_string(), "application/json".to_string())]
                    .into_iter()
                    .collect(),
                body: b"{oops".to_vec(),
            })
        });
        let err = invoke(&plan, &Value::Null, &transport).unwrap_err();
        assert_eq!(
            err.tool_error.message,
            "GET /users returned malformed JSON."
        );
    }
}
