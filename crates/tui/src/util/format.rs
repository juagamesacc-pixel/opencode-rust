// source: packages/tui/src/util/format.ts (20 lines, v1.18.30)
// 1:1 port — coarse duration buckets verbatim (s/m/h/~days/~weeks).

#![allow(dead_code)]

/// Mirrors `formatDuration` (seconds in, display string out).
pub fn format_duration(secs: f64) -> String {
    if secs <= 0.0 {
        return String::new();
    }
    if secs < 60.0 {
        return format!("{secs}s");
    }
    if secs < 3600.0 {
        let mins = (secs / 60.0).floor();
        let remaining = secs % 60.0;
        return if remaining > 0.0 {
            format!("{mins}m {remaining}s")
        } else {
            format!("{mins}m")
        };
    }
    if secs < 86400.0 {
        let hours = (secs / 3600.0).floor();
        let remaining = ((secs % 3600.0) / 60.0).floor();
        return if remaining > 0.0 {
            format!("{hours}h {remaining}m")
        } else {
            format!("{hours}h")
        };
    }
    if secs < 604800.0 {
        let days = (secs / 86400.0).floor();
        return if days == 1.0 {
            "~1 day".to_string()
        } else {
            format!("~{days} days")
        };
    }
    let weeks = (secs / 604800.0).floor();
    if weeks == 1.0 {
        "~1 week".to_string()
    } else {
        format!("~{weeks} weeks")
    }
}
