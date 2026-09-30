// source: src/account/url.ts — exports: normalizeServerUrl (verbatim:
// strip search+hash, trim trailing slashes, origin when empty pathname).

/// source: normalizeServerUrl — verbatim.
pub fn normalize_server_url(input: &str) -> Option<String> {
    let (scheme_end, rest) = input.split_once("://")?;
    let scheme = scheme_end;
    let (authority, path_query) = match rest.find('/') {
        Some(i) => (&rest[..i], &rest[i..]),
        None => (rest, "/"),
    };
    let path = path_query.split(['?', '#']).next().unwrap_or("");
    let pathname = path.trim_end_matches('/');
    if pathname.is_empty() {
        Some(format!("{}://{}", scheme, authority))
    } else {
        Some(format!("{}://{}{}", scheme, authority, pathname))
    }
}
