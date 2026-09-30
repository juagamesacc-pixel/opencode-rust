//! Rust port of `packages/server/src/handlers/message.ts` (opencode v1.18.30).
//!
//! Source 72 lines: `MessageHandler` with `session.messages` (cursor = base64url JSON {id,order,direction}),
//! DefaultMessagesLimit=50, order handling, cursor encode/decode, error mapping.
//!
//! PROVISIONAL: `SessionV2.Service`, `SessionMessage.ID` pending `crates/core`.
//! Error strings: "Cursor cannot be combined with order", "Invalid cursor".

pub const GROUP: &str = "server.message";
pub const OPERATION: &str = "session.messages";
pub const DEFAULT_MESSAGES_LIMIT: usize = 50;

pub const ERR_CURSOR_WITH_ORDER: &str = "Cursor cannot be combined with order";
pub const ERR_INVALID_CURSOR: &str = "Invalid cursor";

/// Cursor struct mirrors `Schema.Struct({ id, order: asc|desc, direction: previous|next })`.
#[derive(Clone, Debug, PartialEq)]
pub struct Cursor {
    pub id: String,
    pub order: String,
    pub direction: String,
}

/// Encode mirrors `Buffer.from(JSON.stringify({id,order,direction})).toString("base64url")`.
pub fn encode_cursor(id: &str, order: &str, direction: &str) -> String {
    let json = format!(
        r#"{{"id":"{}","order":"{}","direction":"{}"}}"#,
        id, order, direction
    );
    base64url_encode(json.as_bytes())
}

/// Decode mirrors `JSON.parse(Buffer.from(input,"base64url").toString("utf8"))`.
pub fn decode_cursor(input: &str) -> Result<Cursor, String> {
    let bytes = base64url_decode(input).ok_or_else(|| ERR_INVALID_CURSOR.to_string())?;
    let s = String::from_utf8(bytes).map_err(|_| ERR_INVALID_CURSOR.to_string())?;
    // Minimal parse for the three fields (order not validated beyond asc/desc shape).
    let v: serde_json::Value =
        serde_json::from_str(&s).map_err(|_| ERR_INVALID_CURSOR.to_string())?;
    let id = v
        .get("id")
        .and_then(|x| x.as_str())
        .ok_or_else(|| ERR_INVALID_CURSOR.to_string())?
        .to_string();
    let order = v
        .get("order")
        .and_then(|x| x.as_str())
        .ok_or_else(|| ERR_INVALID_CURSOR.to_string())?
        .to_string();
    let direction = v
        .get("direction")
        .and_then(|x| x.as_str())
        .ok_or_else(|| ERR_INVALID_CURSOR.to_string())?
        .to_string();
    if order != "asc" && order != "desc" {
        return Err(ERR_INVALID_CURSOR.to_string());
    }
    if direction != "previous" && direction != "next" {
        return Err(ERR_INVALID_CURSOR.to_string());
    }
    Ok(Cursor {
        id,
        order,
        direction,
    })
}

const B64URL_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn base64url_encode(bytes: &[u8]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < bytes.len() {
        let b0 = bytes[i] as u32;
        let b1 = if i + 1 < bytes.len() {
            bytes[i + 1] as u32
        } else {
            0
        };
        let b2 = if i + 2 < bytes.len() {
            bytes[i + 2] as u32
        } else {
            0
        };
        let remaining = bytes.len() - i;
        out.push(B64URL_ALPHABET[((b0 >> 2) & 63) as usize] as char);
        out.push(B64URL_ALPHABET[(((b0 << 4) | (b1 >> 4)) & 63) as usize] as char);
        if remaining > 1 {
            out.push(B64URL_ALPHABET[(((b1 << 2) | (b2 >> 6)) & 63) as usize] as char);
        }
        if remaining > 2 {
            out.push(B64URL_ALPHABET[(b2 & 63) as usize] as char);
        }
        i += 3;
    }
    out
}

fn b64url_val(b: u8) -> Option<u32> {
    match b {
        b'A'..=b'Z' => Some((b - b'A') as u32),
        b'a'..=b'z' => Some((b - b'a' + 26) as u32),
        b'0'..=b'9' => Some((b - b'0' + 52) as u32),
        b'-' => Some(62),
        b'_' => Some(63),
        _ => None,
    }
}

fn base64url_decode(input: &str) -> Option<Vec<u8>> {
    let bytes = input.as_bytes();
    if bytes.len() % 4 == 1 {
        return None;
    }
    let mut out = Vec::new();
    let (chunks, rem) = bytes.as_chunks::<4>();
    for chunk in chunks {
        let c0 = b64url_val(chunk[0])?;
        let c1 = b64url_val(chunk[1])?;
        let c2 = b64url_val(chunk[2])?;
        let c3 = b64url_val(chunk[3])?;
        out.push(((c0 << 2) | (c1 >> 4)) as u8);
        out.push((((c1 & 0xF) << 4) | (c2 >> 2)) as u8);
        out.push((((c2 & 0x3) << 6) | c3) as u8);
    }
    if rem.len() == 2 {
        let c0 = b64url_val(rem[0])?;
        let c1 = b64url_val(rem[1])?;
        out.push(((c0 << 2) | (c1 >> 4)) as u8);
    } else if rem.len() == 3 {
        let c0 = b64url_val(rem[0])?;
        let c1 = b64url_val(rem[1])?;
        let c2 = b64url_val(rem[2])?;
        out.push(((c0 << 2) | (c1 >> 4)) as u8);
        out.push((((c1 & 0xF) << 4) | (c2 >> 2)) as u8);
    }
    Some(out)
}
