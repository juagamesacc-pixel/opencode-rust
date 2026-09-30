//! Rust port of `packages/app/src/utils/time.ts` (opencode v1.18.30).
//!
//! Source 22 lines: `getRelativeTime` (verbatim i18n keys + 60s/60m/24h
//! boundaries).
//! Original file: `packages/app/src/utils/time.ts`

#![allow(dead_code)]

/// Mirrors the `TimeKey` union (verbatim keys).
pub const TIME_JUST_NOW: &str = "common.time.justNow";
pub const TIME_MINUTES_AGO: &str = "common.time.minutesAgo.short";
pub const TIME_HOURS_AGO: &str = "common.time.hoursAgo.short";
pub const TIME_DAYS_AGO: &str = "common.time.daysAgo.short";

/// Mirrors the `Translate` callback shape.
pub type Translate = dyn Fn(&str, Option<u64>) -> String;

/// Mirrors `getRelativeTime(dateString, t)` over a millisecond diff.
pub fn relative_time_key(diff_ms: i64) -> (&'static str, Option<u64>) {
    let diff_seconds = diff_ms.div_euclid(1000);
    let diff_minutes = diff_seconds.div_euclid(60);
    let diff_hours = diff_minutes.div_euclid(60);
    let diff_days = diff_hours.div_euclid(24);
    if diff_seconds < 60 {
        return (TIME_JUST_NOW, None);
    }
    if diff_minutes < 60 {
        return (TIME_MINUTES_AGO, Some(diff_minutes as u64));
    }
    if diff_hours < 24 {
        return (TIME_HOURS_AGO, Some(diff_hours as u64));
    }
    (TIME_DAYS_AGO, Some(diff_days as u64))
}

/// Mirrors `getRelativeTime(dateString, t)`.
pub fn get_relative_time(diff_ms: i64, t: &Translate) -> String {
    let (key, count) = relative_time_key(diff_ms);
    t(key, count)
}
