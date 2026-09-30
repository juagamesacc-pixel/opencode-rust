//! Rust port of `packages/server/src/location.ts` (opencode v1.18.30).
//!
//! Source 60 lines. Exports: `LocationServices` type alias, `LocationMiddleware`
//! service (`"@opencode/HttpApiLocation"`), `response`, `ref` helpers, `decode`,
//! and `layer`.
//!
//! 1:1 notes:
//! - Service id `"@opencode/HttpApiLocation"` verbatim.
//! - `ref(request)` query/header precedence: `location[workspace]` query > `x-opencode-workspace` header;
//!   `location[directory]` query > `x-opencode-directory` (decodeURIComponent) > `process.cwd()`.
//! - `decode` fallback returns input on decode failure.
//! - `response(data)` wraps with `Location.Info { directory, workspaceID, project }`.

/// Service ID verbatim: `"@opencode/HttpApiLocation"`.
pub const LOCATION_MIDDLEWARE_SERVICE_ID: &str = "@opencode/HttpApiLocation";

/// Mirrors `LocationMiddleware` service descriptor.
pub struct LocationMiddleware;

impl LocationMiddleware {
    pub fn service_id() -> &'static str {
        LOCATION_MIDDLEWARE_SERVICE_ID
    }
}

/// Mirrors `Location.Ref` — directory + optional workspaceID.
#[derive(Clone, Debug, PartialEq)]
pub struct LocationRef {
    pub directory: String,
    pub workspace_id: Option<String>,
}

impl LocationRef {
    pub fn make(directory: impl Into<String>, workspace_id: Option<String>) -> Self {
        Self {
            directory: directory.into(),
            workspace_id,
        }
    }
}

/// Mirrors `Location.Info` as returned by `response`.
#[derive(Clone, Debug, PartialEq)]
pub struct LocationInfo {
    pub directory: String,
    pub workspace_id: Option<String>,
    pub project: Option<String>,
}

/// Port of `function ref(request)` — pure extraction from URL + headers map.
///
/// `url`: the request URL string (may be path+query). `headers`: lower-cased header map.
/// `cwd_fallback`: used when neither query nor header provides directory (source: `process.cwd()`).
pub fn location_ref_from_request(
    url: &str,
    headers: &std::collections::HashMap<String, String>,
    cwd_fallback: &str,
) -> LocationRef {
    let query = parse_query(url);
    let workspace_id = query
        .get("location[workspace]")
        .cloned()
        .or_else(|| headers.get("x-opencode-workspace").cloned())
        .filter(|s| !s.is_empty());

    let directory = if let Some(d) = query.get("location[directory]").cloned() {
        d
    } else if let Some(encoded) = headers.get("x-opencode-directory") {
        decode_uri_component(encoded)
    } else {
        cwd_fallback.to_string()
    };

    LocationRef::make(directory, workspace_id)
}

fn parse_query(url: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    let Some(q_start) = url.find('?') else {
        return map;
    };
    let query_str = &url[q_start + 1..];
    let query_str = query_str.split('#').next().unwrap_or(query_str);
    for pair in query_str.split('&') {
        if pair.is_empty() {
            continue;
        }
        let (k, v) = match pair.find('=') {
            Some(i) => (&pair[..i], &pair[i + 1..]),
            None => (pair, ""),
        };
        let k = decode_uri_component(k);
        let v = decode_uri_component(v);
        map.insert(k, v);
    }
    map
}

/// Port of `function decode(input)` — `decodeURIComponent` with fallback to input.
pub fn decode_uri_component(input: &str) -> String {
    // Minimal percent-decode; on any error return input unchanged (mirrors catch branch).
    let mut out = String::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let hi = hex_val(bytes[i + 1]);
            let lo = hex_val(bytes[i + 2]);
            match (hi, lo) {
                (Some(h), Some(l)) => {
                    out.push((h << 4 | l) as char);
                    i += 3;
                    continue;
                }
                _ => return input.to_string(),
            }
        } else if bytes[i] == b'%' {
            return input.to_string();
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Port of `export function response<A,E,R>(data)` — wraps `data` with location info.
///
/// In Rust this is a pure descriptor: the caller provides `location_info` and `data`.
pub fn response_with_location<T>(location: LocationInfo, data: T) -> LocationResponse<T> {
    LocationResponse { location, data }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocationResponse<T> {
    pub location: LocationInfo,
    pub data: T,
}

/// Descriptor for `layer` (Effect layer wiring). No runtime `Effect` here;
/// this struct records the service IDs involved (LocationServiceMap -> LocationMiddleware).
pub struct LocationLayer;

impl LocationLayer {
    pub const PROVIDES: &str = LOCATION_MIDDLEWARE_SERVICE_ID;
    pub const DEPENDS_ON: &str = "@opencode/LocationServiceMap";
}
