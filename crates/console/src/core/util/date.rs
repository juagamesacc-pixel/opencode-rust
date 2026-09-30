// source: core/src/util/date.ts
//! 1:1 port of `getWeekBounds` / `getMonthlyBounds`.
//! Source pin: v1.18.30 @3104c14.

use crate::runtime::time::{date_utc, JsDate};

/// `{ start, end }` returned by both bounds helpers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Bounds {
    pub start: JsDate,
    pub end: JsDate,
}

/// `getWeekBounds(date)` — Monday 00:00:00.000 UTC of `date`'s week through the
/// following Monday 00:00:00.000 UTC.
pub fn get_week_bounds(date: JsDate) -> Bounds {
    let offset = (date.get_utc_day() + 6) % 7;
    let mut start = JsDate::new(date.get_time());
    start.set_utc_date(date.get_utc_date() - offset);
    start.set_utc_hours(0, 0, 0, 0);
    let mut end = JsDate::new(start.get_time());
    end.set_utc_date(start.get_utc_date() + 7);
    Bounds { start, end }
}

/// `getMonthlyBounds(now, subscribed)` — the billing window anchored on the
/// subscription's day-of-month/time, clamped to the target month's length, and
/// rolled back one month when this month's anchor is still in the future.
pub fn get_monthly_bounds(now: JsDate, subscribed: JsDate) -> Bounds {
    let day = subscribed.get_utc_date();
    let hh = subscribed.get_utc_hours();
    let mm = subscribed.get_utc_minutes();
    let ss = subscribed.get_utc_seconds();
    let ms = subscribed.get_utc_milliseconds();

    fn anchor(year: i64, month: i64) -> JsDate {
        // `Date.UTC(year, month + 1, 0)` is the last day of `month`.
        let max = JsDate::new(date_utc(year, month + 1, 0, 0, 0, 0, 0)).get_utc_date();
        JsDate::new(date_utc(year, month, day.min(max), hh, mm, ss, ms))
    }

    fn shift(year: i64, month: i64, delta: i64) -> (i64, i64) {
        let total = year * 12 + month + delta;
        (
            (total as f64 / 12.0).floor() as i64,
            ((total % 12) + 12) % 12,
        )
    }

    let mut y = now.get_utc_full_year();
    let mut m = now.get_utc_month();
    let mut start = anchor(y, m);
    if start.get_time() > now.get_time() {
        let shifted = shift(y, m, -1);
        y = shifted.0;
        m = shifted.1;
        start = anchor(y, m);
    }
    let (ny, nm) = shift(y, m, 1);
    let end = anchor(ny, nm);
    Bounds { start, end }
}
