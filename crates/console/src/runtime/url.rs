//! 1:1 port of the WHATWG `URL` surface used by `packages/console`.
//!
//! Host runtime shim, not a source file. Absolute-URL parsing only (every call
//! site in the source constructs `new URL(absolute)`), exposing exactly the
//! members the ported code reads: `protocol`, `hostname`, `pathname`, `origin`
//! and `toString()`.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Url {
    protocol: String,
    hostname: String,
    port: String,
    pathname: String,
    search: String,
    hash: String,
}

impl Url {
    /// `new URL(input)` — `None` mirrors a thrown `TypeError`, which the source
    /// catches as `catch { return undefined }` / `URL.canParse` returning false.
    pub fn parse(input: &str) -> Option<Self> {
        let (scheme, rest) = input.split_once("://")?;
        if scheme.is_empty()
            || !scheme
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "+-.".contains(c))
        {
            return None;
        }
        let scheme = scheme.to_ascii_lowercase();
        // Credentials are never used by the ported call sites; skip past them.
        let rest = match rest.split_once('@') {
            Some((_, after)) => after,
            None => rest,
        };
        let (authority, tail) = match rest.find(['/', '?', '#']) {
            Some(index) => rest.split_at(index),
            None => (rest, ""),
        };
        let (host, port) = match authority.rsplit_once(':') {
            Some((host, port)) if port.chars().all(|c| c.is_ascii_digit()) => {
                (host, port.to_string())
            }
            _ => (authority, String::new()),
        };
        if host.is_empty() {
            return None;
        }
        let (before_hash, hash) = match tail.split_once('#') {
            Some((before, after)) => (before, format!("#{}", after)),
            None => (tail, String::new()),
        };
        let (pathname, search) = match before_hash.split_once('?') {
            Some((path, search)) => (path.to_string(), format!("?{}", search)),
            None => (before_hash.to_string(), String::new()),
        };
        Some(Self {
            protocol: format!("{}:", scheme),
            hostname: host.to_ascii_lowercase(),
            port,
            pathname: if pathname.is_empty() {
                "/".to_string()
            } else {
                pathname
            },
            search,
            hash,
        })
    }

    /// `URL.canParse(input)`.
    pub fn can_parse(input: &str) -> bool {
        Self::parse(input).is_some()
    }

    /// `url.protocol` — always includes the trailing colon, as in JS.
    pub fn protocol(&self) -> &str {
        &self.protocol
    }

    /// `url.hostname`.
    pub fn hostname(&self) -> &str {
        &self.hostname
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    /// `url.pathname`.
    pub fn pathname(&self) -> &str {
        &self.pathname
    }

    /// `url.origin` — `scheme://host[:port]`, omitting the default port.
    pub fn origin(&self) -> String {
        let default = match self.protocol.as_str() {
            "https:" => "443",
            "http:" => "80",
            _ => "",
        };
        if self.port.is_empty() || self.port == default {
            format!("{}//{}", self.protocol, self.hostname)
        } else {
            format!("{}//{}:{}", self.protocol, self.hostname, self.port)
        }
    }

    /// `url.toString()`.
    pub fn to_url_string(&self) -> String {
        let mut out = format!("{}//{}", self.protocol, self.hostname);
        if !self.port.is_empty() {
            out.push(':');
            out.push_str(&self.port);
        }
        out.push_str(&self.pathname);
        out.push_str(&self.search);
        out.push_str(&self.hash);
        out
    }

    /// `url.searchParams.get(name)` over the raw query string.
    pub fn search_param(&self, name: &str) -> Option<String> {
        let query = self.search.strip_prefix('?')?;
        for pair in query.split('&') {
            if pair.is_empty() {
                continue;
            }
            let (key, value) = match pair.split_once('=') {
                Some(split) => split,
                None => (pair, ""),
            };
            if percent_decode(key) == name {
                return Some(percent_decode(value));
            }
        }
        None
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_url_string())
    }
}

/// `decodeURIComponent` for the `application/x-www-form-urlencoded` subset the
/// source decodes (locale + referral cookies). Invalid escapes are passed
/// through, matching the browser's lenient decoding.
pub fn percent_decode(value: &str) -> String {
    let bytes = value.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        match bytes[index] {
            b'%' if index + 2 < bytes.len() => {
                let hex = &value[index + 1..index + 3];
                match u8::from_str_radix(hex, 16) {
                    Ok(byte) => {
                        out.push(byte);
                        index += 3;
                    }
                    Err(_) => {
                        out.push(bytes[index]);
                        index += 1;
                    }
                }
            }
            b'+' => {
                out.push(b' ');
                index += 1;
            }
            byte => {
                out.push(byte);
                index += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// `encodeURIComponent` — used by the locale and referral cookie writers.
pub fn percent_encode(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for byte in value.as_bytes() {
        let keep = byte.is_ascii_alphanumeric()
            || matches!(
                byte,
                b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
            );
        if keep {
            out.push(*byte as char);
        } else {
            out.push_str(&format!("%{:02X}", byte));
        }
    }
    out
}
