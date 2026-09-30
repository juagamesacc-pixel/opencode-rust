// source: packages/enterprise/src/routes/share/[shareID].tsx — getData bucketing + og-image URL verbatim
//! 1:1 port — `getData` grouping, `SessionDataMissingError`, og-image URL construction, and
//! verbatim UI strings preserved. Solid reactivity/JSX is PROVISIONAL — modeled as descriptors.

use std::collections::HashMap;

use crate::core::share::{Data, MessagePayload, PartPayload, SessionPayload};

/// source: `"Missing shareID"` verbatim
pub const MISSING_SHARE_ID: &str = "Missing shareID";
/// source: `Session ${sessionID} not found` verbatim
pub fn session_not_found(session_id: &str) -> String {
    format!("Session {session_id} not found")
}
/// source: robots meta `"noindex, nofollow"` verbatim
pub const ROBOTS_NOINDEX: &str = "noindex, nofollow";
/// source: description meta verbatim
pub const META_DESCRIPTION: &str = "opencode - The AI coding agent built for the terminal.";
/// source: error fallback strings verbatim
pub const FALLBACK_TITLE: &str = "Unable to render this share.";
pub const FALLBACK_HINT: &str = "Check the console for more details.";
/// source: social card URL base + query names verbatim
pub const SOCIAL_CARD_BASE: &str = "https://social-cards.sst.dev/opencode-share/";
/// source: header links verbatim
pub const HREF_HOME: &str = "https://opencode.ai";
pub const HREF_GITHUB: &str = "https://github.com/anomalyco/opencode";
pub const HREF_DISCORD: &str = "https://opencode.ai/discord";
/// source: query name `"getShareData"` verbatim
pub const QUERY_NAME: &str = "getShareData";
/// source: tabs + review strings verbatim
pub const TAB_SESSION: &str = "Session";
pub const FILES_CHANGED_SUFFIX: &str = "Files Changed";
pub const UNKNOWN_MODEL: &str = "unknown";
pub const TITLE_SUFFIX: &str = "| OpenCode";

/// source: `SessionDataMissingError` — name + `{ sessionID, message? }` data verbatim
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionDataMissingError {
    pub name: String,
    pub session_id: String,
    pub message: Option<String>,
}

impl SessionDataMissingError {
    pub const NAME: &'static str = "SessionDataMissingError";

    pub fn new(session_id: &str) -> Self {
        Self {
            name: Self::NAME.to_string(),
            session_id: session_id.to_string(),
            message: None,
        }
    }

    /// source: `SessionDataMissingError.isInstance` — `NamedError.hasName(input, ...)` verbatim
    pub fn is_instance(name: &str) -> bool {
        name == Self::NAME
    }
}

impl std::fmt::Display for SessionDataMissingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}

impl std::error::Error for SessionDataMissingError {}

/// source: `getData` result shape verbatim
#[derive(Debug, Clone, Default)]
pub struct SharePage {
    pub session_id: String,
    pub share_id: String,
    pub session: Vec<SessionPayload>,
    pub session_diff: HashMap<String, Vec<serde_json::Value>>,
    pub session_status: HashMap<String, String>,
    pub message: HashMap<String, Vec<MessagePayload>>,
    pub part: HashMap<String, Vec<PartPayload>>,
    pub model: HashMap<String, Vec<serde_json::Value>>,
}

/// source: `getData` bucketing — initial `{ session_diff: { [sessionID]: [] }, session_status:
/// { [sessionID]: { type: "idle" } } }` + per-type grouping + binary-search presence check verbatim
pub fn bucket(
    share_id: &str,
    session_id: &str,
    data: Vec<Data>,
) -> Result<SharePage, SessionDataMissingError> {
    let mut page = SharePage {
        session_id: session_id.to_string(),
        share_id: share_id.to_string(),
        ..SharePage::default()
    };
    page.session_diff.insert(session_id.to_string(), vec![]);
    page.session_status
        .insert(session_id.to_string(), "idle".to_string());
    for item in data {
        match item {
            Data::Session { data } => page.session.push(data),
            Data::SessionDiff { data } => {
                page.session_diff.insert(session_id.to_string(), data);
            }
            Data::Message { data } => {
                page.message
                    .entry(data.session_id.clone())
                    .or_default()
                    .push(data);
            }
            Data::Part { data } => {
                page.part
                    .entry(data.message_id.clone())
                    .or_default()
                    .push(data);
            }
            Data::Model { data } => {
                page.model.insert(session_id.to_string(), data);
            }
        }
    }
    if !page.session.iter().any(|s| s.id == session_id) {
        return Err(SessionDataMissingError::new(session_id));
    }
    Ok(page)
}

/// source: `encodeURIComponent` — unreserved marks `A-Za-z0-9 - _ . ! ~ * ' ( )` verbatim (UTF-8 bytes escaped)
pub fn percent_encode(input: &str) -> String {
    let mut out = String::new();
    for b in input.bytes() {
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

/// source: `Base64.encode` (standard alphabet with padding) — `js-base64` equivalent
pub fn base64_encode(input: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(ALPHABET[((triple >> 18) & 63) as usize] as char);
        out.push(ALPHABET[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            ALPHABET[((triple >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            ALPHABET[(triple & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

/// source: og-image URL — `encodeURIComponent(Base64.encode(encodeURIComponent(title[..700])))`
/// + model param (`single` / `"a & b"` / `"a & N others"` / `"unknown"`) + `v{version}` + id verbatim
pub fn og_image_url(title: &str, model_ids: &[String], version: &str, share_id: &str) -> String {
    let title_700: String = title.chars().take(700).collect();
    let encoded_title = percent_encode(&base64_encode(percent_encode(&title_700).as_bytes()));
    let model_param = match model_ids {
        [single] => single.clone(),
        [a, b] => percent_encode(&format!("{a} & {b}")),
        [first, ..] => percent_encode(&format!("{first} & {} others", model_ids.len() - 1)),
        [] => UNKNOWN_MODEL.to_string(),
    };
    format!("{SOCIAL_CARD_BASE}{encoded_title}.png?model={model_param}&version=v{version}&id={share_id}")
}

/// source: title date format `"dd MMM yyyy, HH:mm"` (luxon) — UTC civil-date rendering verbatim shape
pub fn format_share_date(millis: i64) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let secs = millis.div_euclid(1000);
    let minute = secs / 60 % 60;
    let hour = secs / 3600 % 24;
    let mut days = secs.div_euclid(86400);
    let mut year: i64 = 1970;
    loop {
        let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        let len = if leap { 366 } else { 365 };
        if days < len {
            break;
        }
        days -= len;
        year += 1;
    }
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let month_len = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 0;
    while days >= month_len[month] {
        days -= month_len[month];
        month += 1;
    }
    format!(
        "{:02} {} {}, {:02}:{:02}",
        days + 1,
        MONTHS[month],
        year,
        hour,
        minute
    )
}

/// source: user-message sort + first/active selection verbatim
pub fn first_user_message<'a>(
    messages: &'a [MessagePayload],
    active_id: Option<&str>,
) -> Option<&'a MessagePayload> {
    let mut sorted: Vec<&MessagePayload> = messages.iter().collect();
    sorted.sort_by_key(|m| {
        m.extra
            .get("time")
            .and_then(|t| t.get("created"))
            .and_then(serde_json::Value::as_i64)
            .unwrap_or(0)
    });
    match active_id {
        Some(id) => sorted.into_iter().find(|m| m.id == id).or(None),
        None => sorted.into_iter().next(),
    }
}
