//! Rust port of `packages/core/src/tool/http-body.ts`.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

pub fn is_safe_integer(v: i64) -> bool {
    const MAX_SAFE: i64 = 9_007_199_254_740_991;
    (-MAX_SAFE..=MAX_SAFE).contains(&v)
}

pub fn check_declared_size(
    content_length: Option<&str>,
    maximum_bytes: usize,
    too_large: impl Fn() -> String,
) -> Result<Option<usize>, String> {
    if let Some(raw) = content_length {
        if let Ok(parsed) = raw.parse::<i64>() {
            if is_safe_integer(parsed) && parsed >= 0 {
                let sz = parsed as usize;
                if sz > maximum_bytes {
                    return Err(too_large());
                }
                return Ok(Some(sz));
            }
        }
    }
    Ok(None)
}

pub fn collect_bounded_bytes(
    chunks: &[Vec<u8>],
    maximum_bytes: usize,
    declared: Option<usize>,
    too_large: impl Fn() -> String,
) -> Result<Vec<u8>, String> {
    if let Some(d) = declared {
        if d > maximum_bytes {
            return Err(too_large());
        }
    }
    let mut capacity = std::cmp::min(maximum_bytes, declared.unwrap_or(64 * 1024));
    if capacity == 0 {
        capacity = std::cmp::min(maximum_bytes, 64 * 1024);
    }
    let mut body = Vec::with_capacity(capacity);
    let mut size = 0usize;
    for chunk in chunks {
        if chunk.is_empty() {
            continue;
        }
        if size + chunk.len() > maximum_bytes {
            return Err(too_large());
        }
        if size + chunk.len() > body.capacity() {
            let grown = std::cmp::min(
                maximum_bytes,
                std::cmp::max(size + chunk.len(), body.capacity() * 2),
            );
            body.reserve(grown - body.capacity());
        }
        body.extend_from_slice(chunk);
        size += chunk.len();
    }
    Ok(body)
}

pub async fn collect_bounded_response_body(
    response: reqwest::Response,
    maximum_bytes: usize,
    too_large: impl Fn() -> String,
) -> Result<Vec<u8>, String> {
    let content_length = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let declared = check_declared_size(content_length.as_deref(), maximum_bytes, &too_large)?;
    if let Some(d) = declared {
        if d > maximum_bytes {
            return Err(too_large());
        }
    }
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;
    if bytes.len() > maximum_bytes {
        return Err(too_large());
    }
    Ok(bytes.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn early_rejects_declared_oversized() {
        let err = check_declared_size(Some("10485760"), 5 * 1024 * 1024, || {
            "too large".to_string()
        })
        .unwrap_err();
        assert_eq!(err, "too large");
    }
    #[test]
    fn collects_chunks_with_doubling() {
        let chunks = vec![vec![1u8; 1024], vec![2u8; 2048]];
        let out = collect_bounded_bytes(&chunks, 10 * 1024, Some(1024), || "too large".to_string())
            .unwrap();
        assert_eq!(out.len(), 3072);
    }
}
