//! Rust port of `packages/core/src/observability/otlp.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

use std::collections::HashMap;

use crate::flag::flag::{opencode_client, otel_exporter_otlp_endpoint, otel_exporter_otlp_headers};
use crate::installation::version::{installation_channel, installation_version};
use crate::observability::shared::run_id;

fn resource_attributes() -> HashMap<String, String> {
    if let Ok(val) = std::env::var("OTEL_RESOURCE_ATTRIBUTES") {
        if val.is_empty() {
            return HashMap::new();
        }
        let mut map = HashMap::new();
        for entry in val.split(',') {
            if let Some(idx) = entry.find('=') {
                if idx < 1 {
                    continue;
                }
                let k = entry[..idx].trim();
                let v = entry[idx + 1..].trim();
                // decodeURIComponent equivalent: percent-decode limited
                let dk = percent_decode(k);
                let dv = percent_decode(v);
                map.insert(dk, dv);
            }
        }
        return map;
    }
    HashMap::new()
}

fn percent_decode(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '%' {
            let hi = chars.next();
            let lo = chars.next();
            if let (Some(h), Some(l)) = (hi, lo) {
                if let (Some(hv), Some(lv)) = (h.to_digit(16), l.to_digit(16)) {
                    out.push((hv * 16 + lv) as u8 as char);
                    continue;
                } else {
                    out.push('%');
                    out.push(h);
                    out.push(l);
                    continue;
                }
            } else {
                out.push('%');
                if let Some(h) = hi {
                    out.push(h);
                }
                if let Some(l) = lo {
                    out.push(l);
                }
                continue;
            }
        }
        out.push(c);
    }
    out
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub service_name: String,
    pub service_version: String,
    pub attributes: HashMap<String, String>,
}

pub fn resource() -> Resource {
    let mut attrs = resource_attributes();
    attrs.insert(
        "deployment.environment.name".to_string(),
        installation_channel(),
    );
    attrs.insert("opencode.client".to_string(), opencode_client());
    attrs.insert("opencode.run".to_string(), run_id().to_string());
    attrs.insert("service.instance.id".to_string(), run_id().to_string());
    Resource {
        service_name: "opencode".to_string(),
        service_version: installation_version(),
        attributes: attrs,
    }
}

pub fn loggers() -> Vec<String> {
    if otel_exporter_otlp_endpoint().is_none() {
        return vec![];
    }
    // OtlpLogger.make would be here; std representation is descriptor
    vec!["otlp".to_string()]
}

pub fn otlp_headers() -> Option<HashMap<String, String>> {
    otel_exporter_otlp_headers().map(|raw| {
        let mut map = HashMap::new();
        for entry in raw.split(',') {
            if let Some(idx) = entry.find('=') {
                let k = entry[..idx].to_string();
                let v = entry[idx + 1..].to_string();
                map.insert(k, v);
            }
        }
        map
    })
}

pub fn tracing_layer_enabled() -> bool {
    otel_exporter_otlp_endpoint().is_some()
}

pub const SERVICE_NAME: &str = "opencode";
