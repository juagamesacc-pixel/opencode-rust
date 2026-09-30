//! 1:1 port of the ECMAScript `Date` surface used by `packages/console`.
//!
//! Host runtime shim, not a source file. Only the members the ported source
//! actually touches are reproduced, with identical UTC semantics and
//! millisecond precision (JS `Date` is UTC internally).

/// Days from 1970-01-01 to the civil date (y, m, d) using Howard Hinnant's
/// `days_from_civil` algorithm — the same conversion JS `Date` performs.
pub fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = (m + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

/// Inverse of [`days_from_civil`].
pub fn civil_from_days(z: i64) -> (i64, i64, i64) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 2 } else { mp - 10 };
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub const MS_PER_DAY: i64 = 86_400_000;

/// ECMAScript `Date.UTC(...)` with the 0-based month and out-of-range rollover
/// the source depends on (`Date.UTC(year, month + 1, 0)` = last day of `month`).
pub fn date_utc(
    year: i64,
    month: i64,
    day: i64,
    hours: i64,
    minutes: i64,
    seconds: i64,
    ms: i64,
) -> i64 {
    let month_total = year * 12 + month;
    let y = month_total.div_euclid(12);
    let m = month_total.rem_euclid(12);
    days_from_civil(y, m + 1, 1) * MS_PER_DAY
        + (day - 1) * MS_PER_DAY
        + hours * 3_600_000
        + minutes * 60_000
        + seconds * 1_000
        + ms
}

/// UTC-only `Date` equivalent. Mirrors the `getUTC*` / `setUTC*` surface used by
/// `core/src/util/date.ts`, `app/src/routes/zen/util/*` and the console scripts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct JsDate {
    ms: i64,
}

impl JsDate {
    pub const fn new(ms: i64) -> Self {
        Self { ms }
    }

    pub fn now() -> Self {
        Self {
            ms: current_time_millis(),
        }
    }

    pub fn get_time(&self) -> i64 {
        self.ms
    }

    pub fn set_time(&mut self, ms: i64) {
        self.ms = ms;
    }

    fn parts(&self) -> (i64, i64, i64, i64, i64, i64, i64) {
        let days = self.ms.div_euclid(MS_PER_DAY);
        let rem = self.ms.rem_euclid(MS_PER_DAY);
        let (y, m, d) = civil_from_days(days);
        (
            y,
            m - 1,
            d,
            rem / 3_600_000,
            (rem / 60_000) % 60,
            (rem / 1_000) % 60,
            rem % 1_000,
        )
    }

    /// `getUTCDay()` — 0 is Sunday.
    pub fn get_utc_day(&self) -> i64 {
        (self.ms.div_euclid(MS_PER_DAY) + 4).rem_euclid(7)
    }

    pub fn get_utc_full_year(&self) -> i64 {
        self.parts().0
    }

    /// `getUTCMonth()` — 0-based, exactly as JS reports it.
    pub fn get_utc_month(&self) -> i64 {
        self.parts().1
    }

    pub fn get_utc_date(&self) -> i64 {
        self.parts().2
    }

    pub fn get_utc_hours(&self) -> i64 {
        self.parts().3
    }

    pub fn get_utc_minutes(&self) -> i64 {
        self.parts().4
    }

    pub fn get_utc_seconds(&self) -> i64 {
        self.parts().5
    }

    pub fn get_utc_milliseconds(&self) -> i64 {
        self.parts().6
    }

    /// `setUTCDate(day)` — rolls over into adjacent months like JS.
    pub fn set_utc_date(&mut self, day: i64) {
        let (y, m, d, h, mi, s, ms) = self.parts();
        self.ms = date_utc(y, m, day, h, mi, s, ms);
    }

    /// `setUTCHours(h, m, s, ms)`.
    pub fn set_utc_hours(&mut self, hours: i64, minutes: i64, seconds: i64, ms: i64) {
        let (y, m, d, _, _, _, _) = self.parts();
        self.ms = date_utc(y, m, d, hours, minutes, seconds, ms);
    }

    /// `Date#toISOString()` — always UTC with millisecond precision.
    pub fn to_iso_string(&self) -> String {
        let (y, m, d, h, mi, s, ms) = self.parts();
        let year = if (0..=9999).contains(&y) {
            format!("{:04}", y)
        } else if y < 0 {
            format!("-{:06}", -y)
        } else {
            format!("+{:06}", y)
        };
        format!(
            "{}-{}-{}T{:02}:{:02}:{:02}.{:03}Z",
            year,
            m + 1,
            d,
            h,
            mi,
            s,
            ms
        )
    }

    /// `new Date(isoString)` — accepts the exact `YYYY-MM-DDTHH:MM:SS(.mmm)Z`
    /// shape `toISOString` emits (plus bare `YYYY-MM-DDTHH:MM:SSZ`), matching the
    /// literal inputs the ported tests and sources use.
    pub fn parse(iso: &str) -> Option<Self> {
        let bytes = iso.as_bytes();
        if bytes.len() < 10 {
            return None;
        }
        let year: i64 = iso.get(0..4)?.parse().ok()?;
        if bytes[4] != b'-' {
            return None;
        }
        let month: i64 = iso.get(5..7)?.parse().ok()?;
        if bytes[7] != b'-' {
            return None;
        }
        let day: i64 = iso.get(8..10)?.parse().ok()?;
        let (mut hours, mut minutes, mut seconds, mut ms) = (0i64, 0i64, 0i64, 0i64);
        if bytes.len() > 10 {
            if bytes[10] != b'T' && bytes[10] != b' ' {
                return None;
            }
            hours = iso.get(11..13)?.parse().ok()?;
            if bytes[13] != b':' {
                return None;
            }
            minutes = iso.get(14..16)?.parse().ok()?;
            if bytes.len() > 16 {
                if bytes[16] != b':' {
                    return None;
                }
                seconds = iso.get(17..19)?.parse().ok()?;
            }
            let mut rest = 19;
            if bytes.len() > 19 && bytes[19] == b'.' {
                let end = (20..bytes.len())
                    .find(|index| !bytes[*index].is_ascii_digit())
                    .unwrap_or(bytes.len());
                let digits = iso.get(20..end)?;
                if digits.is_empty() {
                    return None;
                }
                let mut fraction = String::from(digits);
                fraction.truncate(3);
                while fraction.len() < 3 {
                    fraction.push('0');
                }
                ms = fraction.parse().ok()?;
                rest = end;
            }
            if bytes.get(rest) != Some(&b'Z') {
                return None;
            }
        }
        Some(Self {
            ms: date_utc(year, month - 1, day, hours, minutes, seconds, ms),
        })
    }
}

impl std::fmt::Display for JsDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_iso_string())
    }
}

/// Wall clock in milliseconds since the Unix epoch, UTC. Backed by
/// `std::time::SystemTime` so it matches `Date.now()`.
pub fn current_time_millis() -> i64 {
    match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(delta) => delta.as_millis() as i64,
        Err(_) => 0,
    }
}

/// `new Date().toISOString().replace(/[^0-9]/g, "").substring(0, n)` — the
/// interval-bucket expression shared by `function/src/stat.ts`,
/// `app/src/routes/zen/util/modelTpsLimiter.ts` and `keyRateLimiter.ts`.
pub fn compact_iso_digits(date: JsDate) -> String {
    date.to_iso_string()
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect()
}

/// `parseInt(compact_iso_digits(date).substring(0, 12))` for the MySQL DATETIME
/// minute bucket used by the TPS/TPM limiters.
pub fn to_interval(date: JsDate) -> i64 {
    let digits: String = compact_iso_digits(date).chars().take(12).collect();
    digits.parse().unwrap_or(0)
}
