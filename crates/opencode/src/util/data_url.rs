// source: src/util/data-url.ts — exports: decodeDataUrl (verbatim).
/// source: decodeDataUrl — verbatim (no comma → "", ;base64 → base64-decode,
/// else decodeURIComponent).
pub fn decode_data_url(url: &str) -> String {
    let idx = match url.find(',') {
        Some(i) => i,
        None => return String::new(),
    };
    let head = &url[..idx];
    let body = &url[idx + 1..];
    if head.contains(";base64") {
        return decode_base64_utf8_lossy(body);
    }
    percent_decode(body)
}

fn decode_base64_utf8_lossy(body: &str) -> String {
    // minimal base64 decoder (no new deps) — verbatim-equivalent output.
    let mut out: Vec<u8> = Vec::new();
    let mut buf: u32 = 0;
    let mut bits: u32 = 0;
    for c in body.chars() {
        let v = match c {
            'A'..='Z' => c as u32 - 'A' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 26,
            '0'..='9' => c as u32 - '0' as u32 + 52,
            '+' | '-' => 62,
            '/' | '_' => 63,
            '=' => break,
            _ => continue,
        };
        buf = (buf << 6) | v;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

fn percent_decode(body: &str) -> String {
    // decodeURIComponent equivalent for %XX sequences — verbatim output.
    let mut out: Vec<u8> = Vec::new();
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                out.push(h * 16 + l);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

fn hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
