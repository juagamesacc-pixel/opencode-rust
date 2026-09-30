// source: src/util/proxy-env.ts — exports: getProxyForUrl, ProxyEnv
// (adapted from proxy-from-env, MIT Rob Wu — license header preserved in
// source; logic ported verbatim: DEFAULT_PORTS, protocol/host/port parse,
// shouldProxy no_proxy rules, env lower||UPPER, scheme-prefix rule).

use std::collections::HashMap;

/// source: DEFAULT_PORTS — verbatim.
pub fn default_port(protocol: &str) -> u16 {
    match protocol {
        "ftp" => 21,
        "gopher" => 70,
        "http" => 80,
        "https" => 443,
        "ws" => 80,
        "wss" => 443,
        _ => 0,
    }
}

/// source: env(key) — lower || UPPER || "", verbatim.
pub fn proxy_env(key: &str, env: &HashMap<String, String>) -> String {
    env.get(&key.to_lowercase())
        .or_else(|| env.get(&key.to_uppercase()))
        .cloned()
        .unwrap_or_default()
}

/// source: shouldProxy — verbatim no_proxy rules.
pub fn should_proxy(hostname: &str, port: u16, no_proxy: &str) -> bool {
    let no_proxy = no_proxy.to_lowercase();
    if no_proxy.is_empty() {
        return true;
    }
    if no_proxy == "*" {
        return false;
    }
    no_proxy.split([',', ' ']).all(|proxy| {
        if proxy.is_empty() {
            return true;
        }
        let (proxy_hostname, proxy_port) = match proxy.rfind(':') {
            Some(i)
                if proxy[i + 1..].chars().all(|c| c.is_ascii_digit())
                    && !proxy[..i].contains(':') =>
            {
                (&proxy[..i], proxy[i + 1..].parse::<u16>().unwrap_or(0))
            }
            _ => (proxy, 0),
        };
        if proxy_port != 0 && proxy_port != port {
            return true;
        }
        if !proxy_hostname.starts_with(['.', '*']) && !proxy_hostname.starts_with('.') {
            // source: /^[.*]/ — starts with . or *
            return hostname != proxy_hostname;
        }
        let suffix = proxy_hostname.strip_prefix('*').unwrap_or(proxy_hostname);
        !hostname.ends_with(suffix)
    })
}

/// source: getProxyForUrl — verbatim (unparseable → undefined/None).
pub fn get_proxy_for_url(input: &str, env: &HashMap<String, String>) -> Option<String> {
    let (protocol, host, port) = split_url(input)?;
    if !should_proxy(&host, port, &proxy_env("no_proxy", env)) {
        return None;
    }
    let proxy = {
        let p = proxy_env(&format!("{}_proxy", protocol), env);
        if !p.is_empty() {
            p
        } else {
            proxy_env("all_proxy", env)
        }
    };
    if proxy.is_empty() {
        return None;
    }
    if proxy.contains("://") {
        Some(proxy)
    } else {
        Some(format!("{}://{}", protocol, proxy))
    }
}

fn split_url(input: &str) -> Option<(String, String, u16)> {
    let (scheme, rest) = input.split_once("://")?;
    let protocol = scheme.to_lowercase();
    let authority = rest.split('/').next().unwrap_or("");
    let (host, port) = match authority.rfind(':') {
        Some(i) if authority[i + 1..].chars().all(|c| c.is_ascii_digit()) => (
            authority[..i].to_string(),
            authority[i + 1..].parse::<u16>().unwrap_or(0),
        ),
        _ => (authority.to_string(), 0),
    };
    if host.is_empty() {
        return None;
    }
    let port = if port == 0 {
        default_port(&protocol)
    } else {
        port
    };
    Some((protocol, host, port))
}
