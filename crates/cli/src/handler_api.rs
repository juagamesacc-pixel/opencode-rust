//! Port of `packages/cli/src/commands/handlers/api.ts` (v1.18.30 @3104c14).
//!
//! Source exports: default handler for `Commands.commands.api`
//! (`Runtime.handler(...)`, lines 17-43), `resolveOperation` (lines 45-53),
//! `rawRequest` (lines 55-58); internal `resolveRequest` (lines 60-73) and
//! `interpolate` (lines 75-85).
//!
//! 1:1 notes (every string preserved):
//! - HTTP methods set: delete/get/head/options/patch/post/put (lowercase).
//! - Header validation: `index < 1` -> fail
//!   `"Invalid header, expected name:value: {header}"`.
//! - Default content-type `application/json` when body present and unset.
//! - `rawRequest`: length 2, method known, path starts with `/`,
//!   else `undefined` (here `None`).
//! - `resolveOperation`: scan `paths` in order, skip unknown methods and
//!   non-matching `operationId`; on miss throw
//!   `"Operation not found: {operationID}"`.
//! - Non-raw input with length != 1 -> fail
//!   `"Expected an operation name or an HTTP method and path"`.
//! - OpenAPI fetch failure (!ok) -> `"Failed to load OpenAPI document: HTTP
//!   {status}"`; served from `/openapi.json` with transport headers.
//! - `interpolate`: `{name}` replaced with `encodeURIComponent(value)`;
//!   missing -> throw `"Missing path parameter: {name}"`; unused params
//!   become the query string; output printed with trailing EOL exactly once.
//! - The `api.test.ts` cases are ported as `#[cfg(test)]` unit tests below
//!   (same inputs, same expected outputs/error strings).

use std::collections::{BTreeMap, HashSet};

/// Port of `methods` (line 7).
pub fn http_methods() -> HashSet<&'static str> {
    ["delete", "get", "head", "options", "patch", "post", "put"]
        .into_iter()
        .collect()
}

/// Port of `Operation` / `OpenApi` types (lines 9-15).
#[derive(Debug, Clone, Default)]
pub struct Operation {
    pub operation_id: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct OpenApi {
    /// path -> method -> operation.
    pub paths: BTreeMap<String, BTreeMap<String, Operation>>,
}

/// Resolved request (method uppercased + interpolated path).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedRequest {
    pub method: String,
    pub path: String,
}

/// Port of `resolveOperation` (lines 45-53).
pub fn resolve_operation(
    spec: &OpenApi,
    operation_id: &str,
    params: &BTreeMap<String, String>,
) -> Result<ResolvedRequest, String> {
    let methods = http_methods();
    for (path, operations) in spec.paths.iter() {
        for (method, operation) in operations.iter() {
            if !methods.contains(method.as_str())
                || operation.operation_id.as_deref() != Some(operation_id)
            {
                continue;
            }
            return Ok(ResolvedRequest {
                method: method.to_uppercase(),
                path: interpolate(path, params)?,
            });
        }
    }
    Err(format!("Operation not found: {operation_id}"))
}

/// Port of `rawRequest` (lines 55-58).
pub fn raw_request(input: &[String]) -> Option<ResolvedRequest> {
    if input.len() != 2
        || !http_methods().contains(input[0].to_lowercase().as_str())
        || !input[1].starts_with('/')
    {
        return None;
    }
    Some(ResolvedRequest {
        method: input[0].to_uppercase(),
        path: input[1].clone(),
    })
}

/// Port of `resolveRequest` (lines 60-73): pure dispatch half. The OpenAPI
/// document fetch is injected (`openapi_doc`) so behavior stays identical
/// without coupling to an HTTP client.
pub fn resolve_request(
    input: &[String],
    params: &BTreeMap<String, String>,
    openapi_doc: Option<&OpenApi>,
    openapi_status: u16,
) -> Result<ResolvedRequest, String> {
    if let Some(raw) = raw_request(input) {
        return Ok(raw);
    }
    if input.len() != 1 {
        return Err("Expected an operation name or an HTTP method and path".to_string());
    }
    match openapi_doc {
        None => Err(format!(
            "Failed to load OpenAPI document: HTTP {openapi_status}"
        )),
        Some(spec) => {
            if openapi_status != 200 {
                return Err(format!(
                    "Failed to load OpenAPI document: HTTP {openapi_status}"
                ));
            }
            resolve_operation(spec, &input[0], params)
        }
    }
}

/// Port of `interpolate` (lines 75-85).
pub fn interpolate(path: &str, params: &BTreeMap<String, String>) -> Result<String, String> {
    let mut used = HashSet::new();
    let mut out = String::new();
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            if let Some(end) = path[i..].find('}') {
                let name = &path[i + 1..i + end];
                match params.get(name) {
                    None => return Err(format!("Missing path parameter: {name}")),
                    Some(value) => {
                        used.insert(name.to_string());
                        out.push_str(&encode_uri_component(value));
                    }
                }
                i += end + 1;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    let query: Vec<(String, String)> = params
        .iter()
        .filter(|(name, _)| !used.contains(*name))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    if query.is_empty() {
        Ok(out)
    } else {
        Ok(format!("{out}?{}", encode_query(&query)))
    }
}

/// `encodeURIComponent` equivalent for path segments.
fn encode_uri_component(value: &str) -> String {
    let mut out = String::new();
    for b in value.bytes() {
        match b {
            b'A'..=b'Z'
            | b'a'..=b'z'
            | b'0'..=b'9'
            | b'-'
            | b'_'
            | b'.'
            | b'!'
            | b'~'
            | b'*'
            | b'\''
            | b'('
            | b')' => out.push(b as char),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn encode_query(pairs: &[(String, String)]) -> String {
    pairs
        .iter()
        .map(|(k, v)| {
            format!(
                "{}={}",
                encode_query_component(k),
                encode_query_component(v)
            )
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn encode_query_component(value: &str) -> String {
    let mut out = String::new();
    for b in value.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// Port of the header loop (lines 25-29): validates `name:value` shape and
/// applies overrides onto the transport headers. Default content-type
/// (line 31) applied when body present and unset.
pub fn apply_headers(
    transport_headers: &BTreeMap<String, String>,
    headers: &[String],
    body: Option<&str>,
) -> Result<BTreeMap<String, String>, String> {
    let mut out: BTreeMap<String, String> = transport_headers
        .iter()
        .map(|(k, v)| (k.to_lowercase(), v.clone()))
        .collect();
    for header in headers {
        let index = header.find(':').unwrap_or(usize::MAX);
        if index == usize::MAX || index < 1 {
            return Err(format!("Invalid header, expected name:value: {header}"));
        }
        out.insert(
            header[..index].trim().to_lowercase(),
            header[index + 1..].trim().to_string(),
        );
    }
    if body.is_some() && !out.contains_key("content-type") {
        out.insert("content-type".to_string(), "application/json".to_string());
    }
    Ok(out)
}

/// Port of the output write (lines 40-41): append EOL unless already present
/// or output empty (empty output writes nothing).
pub fn format_output(output: &str, eol: &str) -> Option<String> {
    if output.is_empty() {
        return None;
    }
    if output.ends_with(eol) {
        Some(output.to_string())
    } else {
        Some(format!("{output}{eol}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> OpenApi {
        let mut ops = BTreeMap::new();
        ops.insert(
            "get".to_string(),
            Operation {
                operation_id: Some("v2.session.get".to_string()),
            },
        );
        let mut paths = BTreeMap::new();
        paths.insert("/api/session/{sessionID}".to_string(), ops);
        OpenApi { paths }
    }

    // Port of api.test.ts: "resolves an operation ID with path and query parameters".
    #[test]
    fn resolves_operation_with_path_and_query() {
        let mut params = BTreeMap::new();
        params.insert("sessionID".to_string(), "ses/a".to_string());
        params.insert("workspace".to_string(), "work".to_string());
        assert_eq!(
            resolve_operation(&spec(), "v2.session.get", &params).unwrap(),
            ResolvedRequest {
                method: "GET".to_string(),
                path: "/api/session/ses%2Fa?workspace=work".to_string(),
            }
        );
    }

    // Port of api.test.ts: "rejects a missing path parameter".
    #[test]
    fn rejects_missing_path_parameter() {
        let err = resolve_operation(&spec(), "v2.session.get", &BTreeMap::new()).unwrap_err();
        assert_eq!(err, "Missing path parameter: sessionID");
    }

    // Port of api.test.ts: "resolves curl-like method and path input".
    #[test]
    fn resolves_raw_method_and_path() {
        assert_eq!(
            raw_request(&["post".to_string(), "/api/foo".to_string()]).unwrap(),
            ResolvedRequest {
                method: "POST".to_string(),
                path: "/api/foo".to_string()
            }
        );
        assert!(raw_request(&["v2.session.list".to_string()]).is_none());
    }
}
