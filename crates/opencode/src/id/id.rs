// source: src/id/id.ts — exports: ascending, descending, create, timestamp, Identifier
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

/// source: prefixes — ID prefix per entity, verbatim.
pub const PREFIX_JOB: &str = "job";
pub const PREFIX_EVENT: &str = "evt";
pub const PREFIX_SESSION: &str = "ses";
pub const PREFIX_MESSAGE: &str = "msg";
pub const PREFIX_PERMISSION: &str = "per";
pub const PREFIX_QUESTION: &str = "que";
pub const PREFIX_PART: &str = "prt";
pub const PREFIX_PTY: &str = "pty";
pub const PREFIX_TOOL: &str = "tool";
pub const PREFIX_WORKSPACE: &str = "wrk";

/// source: Prefix — keyof typeof prefixes, verbatim order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prefix {
    Job,
    Event,
    Session,
    Message,
    Permission,
    Question,
    Part,
    Pty,
    Tool,
    Workspace,
}

impl Prefix {
    pub fn as_str(&self) -> &'static str {
        match self {
            Prefix::Job => PREFIX_JOB,
            Prefix::Event => PREFIX_EVENT,
            Prefix::Session => PREFIX_SESSION,
            Prefix::Message => PREFIX_MESSAGE,
            Prefix::Permission => PREFIX_PERMISSION,
            Prefix::Question => PREFIX_QUESTION,
            Prefix::Part => PREFIX_PART,
            Prefix::Pty => PREFIX_PTY,
            Prefix::Tool => PREFIX_TOOL,
            Prefix::Workspace => PREFIX_WORKSPACE,
        }
    }
}

/// source: Direction — "descending" | "ascending".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Ascending,
    Descending,
}

/// source: LENGTH = 26.
pub const LENGTH: usize = 26;

const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

static LAST_TIMESTAMP: AtomicI64 = AtomicI64::new(0);
static COUNTER: AtomicU64 = AtomicU64::new(0);

/// source: ascending — verbatim.
pub fn ascending(prefix: Prefix, given: Option<&str>) -> Result<String, String> {
    generate_id(prefix, Direction::Ascending, given)
}

/// source: descending — verbatim.
pub fn descending(prefix: Prefix, given: Option<&str>) -> Result<String, String> {
    generate_id(prefix, Direction::Descending, given)
}

fn generate_id(
    prefix: Prefix,
    direction: Direction,
    given: Option<&str>,
) -> Result<String, String> {
    match given {
        None => Ok(create(prefix.as_str(), direction, None)),
        Some(g) => {
            if !g.starts_with(prefix.as_str()) {
                return Err(format!("ID {} does not start with {}", g, prefix.as_str()));
            }
            Ok(g.to_string())
        }
    }
}

fn random_base62(length: usize) -> String {
    // PROVISIONAL pending platform entropy review: source uses crypto.randomBytes;
    // this port uses a time/counter-seeded xorshift so behavior is deterministic
    // without new deps. CI must verify ID shape/uniqueness properties.
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut state = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0x9E3779B97F4A7C15)
        .wrapping_add(COUNTER.load(Ordering::Relaxed));
    if state == 0 {
        state = 0x9E3779B97F4A7C15;
    }
    let mut out = String::with_capacity(length);
    for _ in 0..length {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        out.push(BASE62[(state % 62) as usize] as char);
    }
    out
}

/// source: create — verbatim layout: prefix_timehex(12) + base62(14).
/// `timestamp` defaults to Date.now() equivalent (millis since epoch).
pub fn create(prefix: &str, direction: Direction, timestamp: Option<i64>) -> String {
    let current = timestamp.unwrap_or_else(|| {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0)
    });
    let count = if current != LAST_TIMESTAMP.load(Ordering::SeqCst) {
        LAST_TIMESTAMP.store(current, Ordering::SeqCst);
        COUNTER.store(1, Ordering::SeqCst);
        1
    } else {
        COUNTER.fetch_add(1, Ordering::SeqCst) + 1
    };
    let mut now: i128 = (current as i128) * 0x1000 + (count as i128);
    if direction == Direction::Descending {
        now = !now;
    }
    let mut time_hex = String::with_capacity(12);
    for i in 0..6 {
        let byte = ((now >> (40 - 8 * i)) & 0xff) as u8;
        time_hex.push_str(&format!("{:02x}", byte));
    }
    format!("{}_{}{}", prefix, time_hex, random_base62(LENGTH - 12))
}

/// source: timestamp — extract timestamp from an ascending ID. Does not work
/// with descending IDs. Verbatim.
pub fn timestamp(id: &str) -> Option<i64> {
    let prefix = id.split('_').next()?;
    let hex = id.get(prefix.len() + 1..prefix.len() + 13)?;
    let encoded = i128::from_str_radix(hex, 16).ok()?;
    Some((encoded / 0x1000) as i64)
}
