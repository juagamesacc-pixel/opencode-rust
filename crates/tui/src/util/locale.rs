// source: packages/tui/src/util/locale.ts (86 lines, v1.18.30)
// 1:1 port — titlecase/number/duration/truncate/pluralize are exact;
// local calendar formatting reads the C library timezone via a direct
// libc FFI (no new crates) so output matches `toLocale*` in the user's
// local calendar formatting reads the C library timezone via a direct
// libc FFI (no new crates) so output matches `toLocale*` in the user's
// locale-independent en-US numeric shape used by the transcript.

#![allow(dead_code)]

/// Mirrors `titlecase` (`/\b\w/g` → uppercase).
pub fn titlecase(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut prev_word = false;
    for ch in input.chars() {
        let word = ch.is_alphanumeric() || ch == '_';
        out.push(if word && !prev_word {
            ch.to_uppercase().next().unwrap_or(ch)
        } else {
            ch
        });
        prev_word = word;
    }
    out
}

#[repr(C)]
struct Tm {
    sec: i32,
    min: i32,
    hour: i32,
    mday: i32,
    mon: i32,
    year: i32,
    wday: i32,
    yday: i32,
    isdst: i32,
    gmtoff: i64,
    zone: *const u8,
}

#[link(name = "c")]
extern "C" {
    fn localtime_r(timep: *const i64, result: *mut Tm) -> *mut Tm;
}

#[derive(Debug, Clone, Copy, Default)]
struct Local {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
}

fn local_parts(ms: i64) -> Option<Local> {
    let secs = ms.div_euclid(1000);
    let mut tm = Tm {
        sec: 0,
        min: 0,
        hour: 0,
        mday: 0,
        mon: 0,
        year: 0,
        wday: 0,
        yday: 0,
        isdst: 0,
        gmtoff: 0,
        zone: std::ptr::null(),
    };
    // SAFETY: localtime_r is thread-safe; fields are plain ints read after the call.
    let ok = unsafe { !localtime_r(&secs, &mut tm).is_null() };
    if !ok {
        return None;
    }
    Some(Local {
        year: tm.year + 1900,
        month: (tm.mon + 1) as u32,
        day: tm.mday as u32,
        hour: tm.hour as u32,
        minute: tm.min as u32,
        second: tm.sec as u32,
    })
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn short_time(parts: Local) -> String {
    let (hour12, suffix) = match parts.hour {
        0 => (12, "AM"),
        1..=11 => (parts.hour, "AM"),
        12 => (12, "PM"),
        _ => (parts.hour - 12, "PM"),
    };
    format!("{}:{:02} {}", hour12, parts.minute, suffix)
}

/// Mirrors `time` — en-US `timeStyle: "short"` (`h:mm AM`).
pub fn time(input_ms: i64) -> String {
    local_parts(input_ms)
        .map(short_time)
        .unwrap_or_else(|| "12:00 AM".to_string())
}

/// Mirrors `datetime` — `time · M/D/YYYY`.
pub fn datetime(input_ms: i64) -> String {
    match local_parts(input_ms) {
        Some(parts) => format!(
            "{} · {}/{}/{}",
            short_time(parts),
            parts.month,
            parts.day,
            parts.year
        ),
        None => time(input_ms),
    }
}

/// Mirrors `todayTimeOrDateTime`.
pub fn today_time_or_date_time(input_ms: i64) -> String {
    let (a, b) = (local_parts(input_ms), local_parts(now_ms()));
    match (a, b) {
        (Some(date), Some(now))
            if date.year == now.year && date.month == now.month && date.day == now.day =>
        {
            short_time(date)
        }
        _ => datetime(input_ms),
    }
}

/// Mirrors `Date.toDateString()` en-US (`Wed Oct 01 2025`).
pub fn date_string(input_ms: i64) -> String {
    const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let days = input_ms.div_euclid(86_400_000);
    // 1970-01-01 was a Thursday (index 4).
    let weekday = DAYS[(days + 4).rem_euclid(7) as usize];
    match local_parts(input_ms) {
        Some(parts) => format!(
            "{} {} {:02} {}",
            weekday,
            MONTHS[(parts.month.saturating_sub(1) as usize).min(11)],
            parts.day,
            parts.year
        ),
        None => String::new(),
    }
}
/// Mirrors the transcript header clock — en-US `toLocaleString()`
/// (`M/D/YYYY, h:mm:ss AM`).
pub fn locale_string(input_ms: i64) -> String {
    match local_parts(input_ms) {
        Some(parts) => {
            let (hour12, suffix) = match parts.hour {
                0 => (12, "AM"),
                1..=11 => (parts.hour, "AM"),
                12 => (12, "PM"),
                _ => (parts.hour - 12, "PM"),
            };
            format!(
                "{}/{}/{}, {}:{:02}:{:02} {}",
                parts.month, parts.day, parts.year, hour12, parts.minute, parts.second, suffix
            )
        }
        None => String::new(),
    }
}

/// Mirrors `number` — `1.5K` / `2.0M` collapsing.
pub fn number(num: f64) -> String {
    if num >= 1_000_000.0 {
        format!("{:.1}M", num / 1_000_000.0)
    } else if num >= 1000.0 {
        format!("{:.1}K", num / 1000.0)
    } else if num.fract() == 0.0 {
        format!("{}", num as i64)
    } else {
        format!("{num}")
    }
}

/// Mirrors `duration` — ms/s/m/h/d buckets verbatim.
pub fn duration(input_ms: i64) -> String {
    if input_ms < 1000 {
        format!("{input_ms}ms")
    } else if input_ms < 60_000 {
        format!("{:.1}s", input_ms as f64 / 1000.0)
    } else if input_ms < 3_600_000 {
        format!("{}m {}s", input_ms / 60_000, (input_ms % 60_000) / 1000)
    } else if input_ms < 86_400_000 {
        format!(
            "{}h {}m",
            input_ms / 3_600_000,
            (input_ms % 3_600_000) / 60_000
        )
    } else {
        format!(
            "{}d {}h",
            input_ms / 86_400_000,
            (input_ms % 86_400_000) / 3_600_000
        )
    }
}

fn char_len(s: &str) -> usize {
    s.chars().count()
}

fn take_chars(s: &str, n: usize) -> String {
    s.chars().take(n).collect()
}

/// Mirrors `truncate` (char-safe equivalent of the UTF-16 slice).
pub fn truncate(input: &str, len: usize) -> String {
    if char_len(input) <= len {
        return input.to_string();
    }
    take_chars(input, len.saturating_sub(1)) + "…"
}

/// Mirrors `truncateLeft`.
pub fn truncate_left(input: &str, len: usize) -> String {
    if char_len(input) <= len {
        return input.to_string();
    }
    let tail: String = input
        .chars()
        .rev()
        .take(len.saturating_sub(1))
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("…{tail}")
}

/// Mirrors `truncateMiddle` (default 35).
pub fn truncate_middle(input: &str, max_length: usize) -> String {
    if char_len(input) <= max_length {
        return input.to_string();
    }
    let keep = max_length.saturating_sub(1);
    let keep_start = keep.div_ceil(2);
    let keep_end = keep / 2;
    let tail: String = input
        .chars()
        .rev()
        .take(keep_end)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("{}…{tail}", take_chars(input, keep_start))
}

/// Mirrors `pluralize` — replaces the first `{}` in the chosen template.
pub fn pluralize(count: i64, singular: &str, plural: &str) -> String {
    let template = if count == 1 { singular } else { plural };
    template.replacen("{}", &count.to_string(), 1)
}
