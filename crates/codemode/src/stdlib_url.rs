//! Port of `src/stdlib/url.ts`.

use crate::interpreter_model::{AstNode, DiagnosticKind, InterpreterRuntimeError};
use crate::stdlib_value::{bounded_data, coerce_to_string, DataVal};
use crate::values::{SandboxURL, SandboxURLSearchParams};

/// URL properties, verbatim. Mirrors `urlProperties`.
pub const URL_PROPERTIES: &[&str] = &[
    "href", "origin", "protocol", "username", "password", "host", "hostname", "port", "pathname",
    "search", "hash",
];

/// Writable URL properties, verbatim. Mirrors `urlWritableProperties`.
pub const URL_WRITABLE_PROPERTIES: &[&str] = &[
    "href", "protocol", "username", "password", "host", "hostname", "port", "pathname", "search",
    "hash",
];

/// URL methods, verbatim. Mirrors `urlMethods`.
pub const URL_METHODS: &[&str] = &["toString", "toJSON"];

/// URL statics, verbatim. Mirrors `urlStatics`.
pub const URL_STATICS: &[&str] = &["canParse", "parse"];

/// URLSearchParams methods, verbatim. Mirrors `urlSearchParamsMethods`.
pub const URL_SEARCH_PARAMS_METHODS: &[&str] = &[
    "append", "delete", "get", "getAll", "has", "set", "sort", "forEach", "keys", "values",
    "entries", "toString",
];

/// Mirrors `urlProperties.has(name)`.
pub fn is_url_property(name: &str) -> bool {
    URL_PROPERTIES.contains(&name)
}

/// Mirrors `urlStatics.has(name)` membership.
pub fn is_url_static(name: &str) -> bool {
    URL_STATICS.contains(&name)
}

/// Mirrors `urlSearchParamsMethods.has(name)` membership.
pub fn is_url_search_params_method(name: &str) -> bool {
    URL_SEARCH_PARAMS_METHODS.contains(&name)
}

/// Mirrors `uriArgument(value, label)` = `coerceToString(boundedData(value, label))`.
pub fn uri_argument(value: &DataVal, label: &str) -> Result<String, InterpreterRuntimeError> {
    let checked = bounded_data(value, label)?;
    Ok(coerce_to_string(&checked))
}

/// Mirrors `invokeUriFunction(ref, args, node)`.
pub fn invoke_uri_function(
    name: crate::interpreter_model::UriKind,
    args: &[DataVal],
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    let input = args.first().cloned().unwrap_or(DataVal::Undefined);
    let label = format!("{} input", name.as_str());
    let value = uri_argument(&input, &label)?;
    let result: Result<String, String> = match name {
        crate::interpreter_model::UriKind::EncodeUri => Ok(percent_encode_uri(&value, false)),
        crate::interpreter_model::UriKind::EncodeUriComponent => {
            Ok(percent_encode_uri(&value, true))
        }
        crate::interpreter_model::UriKind::DecodeUri => percent_decode(&value, false),
        crate::interpreter_model::UriKind::DecodeUriComponent => percent_decode(&value, true),
    };
    match result {
        Ok(s) => Ok(DataVal::Str(s)),
        Err(message) => Err(InterpreterRuntimeError::new(
            format!("{} received malformed URI data: {}", name.as_str(), message),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )
        .as_error("URIError")),
    }
}

/// Mirrors `urlArgument(value, label)`.
pub fn url_argument(value: &DataVal, label: &str) -> Result<String, InterpreterRuntimeError> {
    if let DataVal::Sandbox(crate::values::SandboxValue::Url(u)) = value {
        return Ok(u.href.clone());
    }
    uri_argument(value, label)
}

/// Parsed URL parts (minimal WHATWG-subset for orchestration code).
#[derive(Debug, Clone)]
pub struct ParsedUrl {
    pub href: String,
    pub protocol: String,
    pub username: String,
    pub password: String,
    pub host: String,
    pub hostname: String,
    pub port: String,
    pub pathname: String,
    pub search: String,
    pub hash: String,
    pub pairs: Vec<(String, String)>,
}

/// Minimal URL parser for absolute `http(s)` URLs + relative resolution
/// against a base. Returns `None` when unparseable (callers map to
/// `canParse → false` / `parse → null`, verbatim).
pub fn parse_url(input: &str, base: Option<&str>) -> Option<ParsedUrl> {
    // Absolute URL with scheme.
    if let Some(after_scheme) = split_scheme(input) {
        return parse_absolute(&after_scheme.0, after_scheme.1);
    }
    // Relative resolution against base (minimal: absolute-path + query).
    let base_parsed = base
        .and_then(|b| split_scheme(b))
        .and_then(|(s, r)| parse_absolute(&s, r))?;
    if input.starts_with('/') {
        let (path, query, hash) = split_path_query_hash(input);
        return Some(rebuild_with(&base_parsed, &path, &query, &hash));
    }
    if input.is_empty() {
        return Some(base_parsed);
    }
    None
}

fn split_scheme(input: &str) -> Option<(String, &str)> {
    let pos = input.find("://")?;
    let scheme = input[..pos].to_ascii_lowercase();
    if scheme.is_empty()
        || !scheme
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))
    {
        return None;
    }
    Some((scheme, &input[pos + 3..]))
}

fn split_path_query_hash(input: &str) -> (String, String, String) {
    let (before_hash, hash) = match input.split_once('#') {
        Some((b, h)) => (b, format!("#{}", h)),
        None => (input, String::new()),
    };
    let (path, query) = match before_hash.split_once('?') {
        Some((p, q)) => (p.to_string(), format!("?{}", q)),
        None => (before_hash.to_string(), String::new()),
    };
    (path, query, hash)
}

fn parse_absolute(scheme: &str, rest: &str) -> Option<ParsedUrl> {
    if scheme != "http" && scheme != "https" {
        return None;
    }
    let (authority, path_part) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    let (authority, _) = match authority.split_once(['?', '#']) {
        Some((a, _)) => (a, true),
        None => (authority, false),
    };
    if authority.is_empty() {
        return None;
    }
    let (userinfo, hostport) = match authority.rsplit_once('@') {
        Some((u, h)) => (u, h),
        None => ("", authority),
    };
    let (username, password) = match userinfo.split_once(':') {
        Some((u, p)) => (u.to_string(), p.to_string()),
        None => (userinfo.to_string(), String::new()),
    };
    let (hostname, port) = match hostport.rsplit_once(':') {
        Some((h, p)) if !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()) => {
            (h.to_string(), p.to_string())
        }
        _ => (hostport.to_string(), String::new()),
    };
    if hostname.is_empty() {
        return None;
    }
    let (pathname, search, hash) = split_path_query_hash(path_part);
    let pairs = parse_query_pairs(search.trim_start_matches('?'));
    let default_port = if scheme == "http" { "80" } else { "443" };
    let host = if port.is_empty() || port == default_port {
        hostname.clone()
    } else {
        format!("{}:{}", hostname, port)
    };
    let origin = format!("{}://{}", scheme, host);
    let href = format!("{}{}{}{}{}", origin, pathname, search, hash, "");
    Some(ParsedUrl {
        href,
        protocol: format!("{}:", scheme),
        username,
        password,
        host,
        hostname,
        port: if port == default_port {
            String::new()
        } else {
            port
        },
        pathname,
        search,
        hash,
        pairs,
    })
}

fn rebuild_with(base: &ParsedUrl, path: &str, query: &str, hash: &str) -> ParsedUrl {
    let origin_end = base.href.find(&base.pathname).unwrap_or(base.href.len());
    let origin = base.href[..origin_end].to_string();
    let pairs = parse_query_pairs(query.trim_start_matches('?'));
    ParsedUrl {
        href: format!("{}{}{}{}", origin, path, query, hash),
        protocol: base.protocol.clone(),
        username: base.username.clone(),
        password: base.password.clone(),
        host: base.host.clone(),
        hostname: base.hostname.clone(),
        port: base.port.clone(),
        pathname: path.to_string(),
        search: query.to_string(),
        hash: hash.to_string(),
        pairs,
    }
}

fn parse_query_pairs(query: &str) -> Vec<(String, String)> {
    if query.is_empty() {
        return vec![];
    }
    query
        .split('&')
        .filter(|p| !p.is_empty())
        .map(|pair| match pair.split_once('=') {
            Some((k, v)) => (form_decode(k), form_decode(v)),
            None => (form_decode(pair), String::new()),
        })
        .collect()
}

fn form_decode(s: &str) -> String {
    percent_decode_raw(&s.replace('+', " ")).unwrap_or_else(|_| s.replace('+', " "))
}

fn percent_decode_raw(s: &str) -> Result<String, ()> {
    let mut bytes = vec![];
    let mut chars = s.bytes();
    while let Some(b) = chars.next() {
        if b == b'%' {
            let hi = chars.next().ok_or(())?;
            let lo = chars.next().ok_or(())?;
            let hex = |c: u8| (c as char).to_digit(16).map(|d| d as u8).ok_or(());
            bytes.push(hex(hi)? * 16 + hex(lo)?);
        } else {
            bytes.push(b);
        }
    }
    String::from_utf8(bytes).map_err(|_| ())
}

/// Mirrors `invokeURLStatic(name, args, node)`.
pub fn invoke_url_static(
    name: &str,
    args: &[DataVal],
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    if !is_url_static(name) {
        return Err(InterpreterRuntimeError::new(
            format!("URL.{} is not available in CodeMode.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        ));
    }
    if args.is_empty() {
        return Err(InterpreterRuntimeError::new(
            format!("URL.{} requires a URL argument.", name),
            node.cloned(),
            DiagnosticKind::ExecutionFailure,
            None,
        )
        .as_error("TypeError"));
    }
    let input = url_argument(
        args.first().expect("checked"),
        &format!("URL.{} input", name),
    )?;
    let base = match args.get(1) {
        None | Some(DataVal::Undefined) => None,
        Some(b) => Some(url_argument(b, &format!("URL.{} base", name))?),
    };
    match parse_url(&input, base.as_deref()) {
        Some(parsed) => {
            if name == "canParse" {
                Ok(DataVal::Bool(true))
            } else {
                Ok(DataVal::Sandbox(crate::values::SandboxValue::Url(
                    SandboxURL::new(parsed.href, parsed.pairs),
                )))
            }
        }
        None => {
            if name == "canParse" {
                Ok(DataVal::Bool(false))
            } else {
                Ok(DataVal::Null)
            }
        }
    }
}

/// Mirrors `invokeURLMethod(value, name, node)` (`toString`/`toJSON`).
pub fn invoke_url_method(
    href: &str,
    name: &str,
    node: Option<&AstNode>,
) -> Result<DataVal, InterpreterRuntimeError> {
    if name == "toString" || name == "toJSON" {
        return Ok(DataVal::Str(href.to_string()));
    }
    Err(InterpreterRuntimeError::new(
        format!("URL method '{}' is not available in CodeMode.", name),
        node.cloned(),
        DiagnosticKind::ExecutionFailure,
        None,
    ))
}

/// Reads one URL property. The interpreter enforces read-only semantics
/// (`URL.* read-only` TypeError) for non-writable writes.
pub fn url_property(parsed: &ParsedUrl, name: &str) -> Option<String> {
    match name {
        "href" => Some(parsed.href.clone()),
        "origin" => Some(format!("{}//{}", parsed.protocol, parsed.host)),
        "protocol" => Some(parsed.protocol.clone()),
        "username" => Some(parsed.username.clone()),
        "password" => Some(parsed.password.clone()),
        "host" => Some(parsed.host.clone()),
        "hostname" => Some(parsed.hostname.clone()),
        "port" => Some(parsed.port.clone()),
        "pathname" => Some(parsed.pathname.clone()),
        "search" => Some(parsed.search.clone()),
        "hash" => Some(parsed.hash.clone()),
        _ => None,
    }
}

fn percent_encode_uri(s: &str, component: bool) -> String {
    // Unreserved + JS encodeURI reserved set.
    let mut out = String::new();
    for b in s.bytes() {
        let c = b as char;
        let unescaped = c.is_ascii_alphanumeric() || "-_.!~*'()".contains(c);
        let reserved = ";,/?:@&=+$-#".contains(c);
        if unescaped || (!component && reserved) {
            out.push(c);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}

fn percent_decode(s: &str, _component: bool) -> Result<String, String> {
    percent_decode_raw(s).map_err(|_| "URI malformed".to_string())
}

/// Sorts URLSearchParams pairs in place. Mirrors `params.sort()`.
pub fn sort_search_params(params: &mut SandboxURLSearchParams) {
    params.pairs.sort_by(|a, b| a.0.cmp(&b.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_guards_verbatim() {
        let err = invoke_url_static("createObjectURL", &[], None).unwrap_err();
        assert_eq!(
            err.message,
            "URL.createObjectURL is not available in CodeMode."
        );
        let err = invoke_url_static("parse", &[], None).unwrap_err();
        assert_eq!(err.message, "URL.parse requires a URL argument.");
    }

    #[test]
    fn url_parse_round_trip() {
        let parsed = parse_url("https://example.test:8080/a/b?x=1#h", None).unwrap();
        assert_eq!(parsed.hostname, "example.test");
        assert_eq!(parsed.port, "8080");
        assert_eq!(parsed.pathname, "/a/b");
        assert_eq!(parsed.search, "?x=1");
        assert_eq!(parsed.hash, "#h");
    }
}
