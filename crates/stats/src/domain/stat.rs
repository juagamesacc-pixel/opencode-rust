// source: packages/stats/core/src/domain/stat.ts (1:1 port)

/// Maximum number of rows written per upsert batch.
pub const UPSERT_CHUNK_SIZE: usize = 500;
/// Site tiers that are presented together on the public site.
pub const DATA_SITE_TIERS: [&str; 4] = ["Go", "go", "Free", "free"];
const DAY_MS: i64 = 86_400_000;

/// Aggregation grain: daily or weekly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatGrain {
    Day,
    Week,
}

impl StatGrain {
    pub fn as_str(self) -> &'static str {
        match self {
            StatGrain::Day => "day",
            StatGrain::Week => "week",
        }
    }
}

/// Base aggregate produced by the ingestion SQL (mirrors `StatBaseAggregate`).
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatBaseAggregate {
    pub grain: String,
    pub period_key: String,
    pub dataset: String,
    pub tier: String,
    pub sessions: i64,
    pub requests: i64,
    pub unique_users: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub reasoning_tokens: i64,
    pub cache_read_tokens: i64,
    pub total_tokens: i64,
    pub input_cost_microcents: i64,
    pub output_cost_microcents: i64,
    pub total_cost_microcents: i64,
    pub avg_duration_ms: Option<f64>,
    pub p50_duration_ms: Option<i64>,
    pub p95_duration_ms: Option<i64>,
    pub avg_ttfb_ms: Option<f64>,
    pub p50_ttfb_ms: Option<i64>,
    pub p95_ttfb_ms: Option<i64>,
    pub avg_output_tps: Option<f64>,
    pub success_count: i64,
    pub error_count: i64,
    pub sample_count: i64,
}

/// Base stat row shape shared by model/provider/geo tables (mirrors `StatBaseRow`).
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct StatBaseRow {
    pub grain: String,
    pub period_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sessions: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_users: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_cost_microcents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_cost_microcents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cost_microcents: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_duration_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p50_duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p95_duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_ttfb_ms: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p50_ttfb_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p95_ttfb_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_output_tps: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_count: Option<i64>,
}

impl StatBaseRow {
    /// Column values carried by every base row (client/source default to "all").
    pub fn from_aggregate(data: &StatBaseAggregate) -> StatBaseRow {
        StatBaseRow {
            grain: data.grain.clone(),
            period_key: data.period_key.clone(),
            dataset: Some(data.dataset.clone()),
            tier: Some(data.tier.clone()),
            client: Some("all".to_string()),
            source: Some("all".to_string()),
            sessions: Some(data.sessions),
            requests: Some(data.requests),
            unique_users: Some(data.unique_users),
            input_tokens: Some(data.input_tokens),
            output_tokens: Some(data.output_tokens),
            reasoning_tokens: Some(data.reasoning_tokens),
            cache_read_tokens: Some(data.cache_read_tokens),
            total_tokens: Some(data.total_tokens),
            input_cost_microcents: Some(data.input_cost_microcents),
            output_cost_microcents: Some(data.output_cost_microcents),
            total_cost_microcents: Some(data.total_cost_microcents),
            avg_duration_ms: data.avg_duration_ms,
            p50_duration_ms: data.p50_duration_ms,
            p95_duration_ms: data.p95_duration_ms,
            avg_ttfb_ms: data.avg_ttfb_ms,
            p50_ttfb_ms: data.p50_ttfb_ms,
            p95_ttfb_ms: data.p95_ttfb_ms,
            avg_output_tps: data.avg_output_tps,
            success_count: Some(data.success_count),
            error_count: Some(data.error_count),
            sample_count: Some(data.sample_count),
        }
    }
}

/// Appends a synthesized `tier = "all"` row per dimension key (`synthesizeAllTierRows`).
pub fn synthesize_all_tier_rows<T: Clone + StatBaseRowLike>(rows: &[T]) -> Vec<T> {
    let mut out: Vec<T> = rows.to_vec();
    let mut merged: Vec<T> = Vec::new();
    for row in rows {
        let key = [
            row.grain_key(),
            row.period_key(),
            row.dataset_key(),
            row.client_key(),
            row.source_key(),
            row.dimension_key(),
        ]
        .join("\u{0}");
        if let Some(existing) = merged.iter_mut().find(|r| {
            [
                r.grain_key(),
                r.period_key(),
                r.dataset_key(),
                r.client_key(),
                r.source_key(),
                r.dimension_key(),
            ]
            .join("\u{0}")
                == key
        }) {
            *existing = combine_rows(existing.clone(), row.clone());
        } else {
            let mut synthesized = row.clone();
            synthesized.set_tier("all".to_string());
            merged.push(synthesized);
        }
    }
    out.extend(merged);
    out
}

/// Collapses rows sharing a dimension key, summing metrics (`collapseRows`).
pub fn collapse_rows<T: Clone + StatBaseRowLike>(rows: &[T]) -> Vec<T> {
    let mut merged: Vec<T> = Vec::new();
    for row in rows {
        let key = [
            row.grain_key(),
            row.period_key(),
            row.dataset_key(),
            row.tier_key(),
            row.client_key(),
            row.source_key(),
            row.dimension_key(),
        ]
        .join("\u{0}");
        if let Some(existing) = merged.iter_mut().find(|r| {
            [
                r.grain_key(),
                r.period_key(),
                r.dataset_key(),
                r.tier_key(),
                r.client_key(),
                r.source_key(),
                r.dimension_key(),
            ]
            .join("\u{0}")
                == key
        }) {
            *existing = combine_rows(existing.clone(), row.clone());
        } else {
            merged.push(row.clone());
        }
    }
    merged
}

/// Trait implemented by stat row structs so the shared combine/rank helpers
/// stay generic (mirrors the `T extends StatBaseRow` constraint).
///
/// The concrete row structs use [`impl_stat_base_row!`] which implements the
/// base metrics; tables with market-share/rank columns use the `market` or
/// `ranks` variants. Each struct supplies its own `dimension_key` method.
pub trait StatBaseRowLike: Clone {
    fn grain_key(&self) -> String;
    fn period_key(&self) -> String;
    fn dataset_key(&self) -> String;
    fn tier_key(&self) -> String;
    fn client_key(&self) -> String;
    fn source_key(&self) -> String;
    fn dimension_key(&self) -> String;
    fn set_tier(&mut self, tier: String);
    fn sessions(&self) -> i64;
    fn set_sessions(&mut self, value: i64);
    fn requests(&self) -> i64;
    fn set_requests(&mut self, value: i64);
    fn unique_users(&self) -> i64;
    fn set_unique_users(&mut self, value: i64);
    fn input_tokens(&self) -> i64;
    fn set_input_tokens(&mut self, value: i64);
    fn output_tokens(&self) -> i64;
    fn set_output_tokens(&mut self, value: i64);
    fn reasoning_tokens(&self) -> i64;
    fn set_reasoning_tokens(&mut self, value: i64);
    fn cache_read_tokens(&self) -> i64;
    fn set_cache_read_tokens(&mut self, value: i64);
    fn total_tokens(&self) -> i64;
    fn set_total_tokens(&mut self, value: i64);
    fn input_cost_microcents(&self) -> i64;
    fn set_input_cost_microcents(&mut self, value: i64);
    fn output_cost_microcents(&self) -> i64;
    fn set_output_cost_microcents(&mut self, value: i64);
    fn total_cost_microcents(&self) -> i64;
    fn set_total_cost_microcents(&mut self, value: i64);
    fn avg_duration_ms(&self) -> Option<f64>;
    fn set_avg_duration_ms(&mut self, value: Option<f64>);
    fn p50_duration_ms(&self) -> Option<i64>;
    fn set_p50_duration_ms(&mut self, value: Option<i64>);
    fn p95_duration_ms(&self) -> Option<i64>;
    fn set_p95_duration_ms(&mut self, value: Option<i64>);
    fn avg_ttfb_ms(&self) -> Option<f64>;
    fn set_avg_ttfb_ms(&mut self, value: Option<f64>);
    fn p50_ttfb_ms(&self) -> Option<i64>;
    fn set_p50_ttfb_ms(&mut self, value: Option<i64>);
    fn p95_ttfb_ms(&self) -> Option<i64>;
    fn set_p95_ttfb_ms(&mut self, value: Option<i64>);
    fn avg_output_tps(&self) -> Option<f64>;
    fn set_avg_output_tps(&mut self, value: Option<f64>);
    fn success_count(&self) -> i64;
    fn set_success_count(&mut self, value: i64);
    fn error_count(&self) -> i64;
    fn set_error_count(&mut self, value: i64);
    fn sample_count(&self) -> i64;
    fn set_sample_count(&mut self, value: i64);
    /// Optional market-share columns (provider/geo tables only).
    fn market_share_tokens(&self) -> Option<f64>;
    fn set_market_share_tokens(&mut self, value: Option<f64>);
    fn market_share_requests(&self) -> Option<f64>;
    fn set_market_share_requests(&mut self, value: Option<f64>);
    fn market_share_sessions(&self) -> Option<f64>;
    fn set_market_share_sessions(&mut self, value: Option<f64>);
    fn rank_by_tokens(&self) -> Option<i64>;
    fn set_rank_by_tokens(&mut self, value: Option<i64>);
    fn rank_by_requests(&self) -> Option<i64>;
    fn set_rank_by_requests(&mut self, value: Option<i64>);
    fn rank_by_sessions(&self) -> Option<i64>;
    fn set_rank_by_sessions(&mut self, value: Option<i64>);
    fn rank_by_cost(&self) -> Option<i64>;
    fn set_rank_by_cost(&mut self, value: Option<i64>);
}

/// Sums two rows, with weighted averages for the average columns (`combineRows`).
pub fn combine_rows<T: StatBaseRowLike>(left: T, right: T) -> T {
    let mut out = left;
    // Weighted averages weight by the LEFT row's own pre-sum request count
    // (matches `combineRows` in the source: weights come from each input row).
    let left_requests = out.requests() as f64;
    let right_requests = right.requests() as f64;
    let avg_duration = weighted_average(
        out.avg_duration_ms(),
        Some(left_requests),
        right.avg_duration_ms(),
        Some(right_requests),
    );
    let avg_ttfb = weighted_average(
        out.avg_ttfb_ms(),
        Some(left_requests),
        right.avg_ttfb_ms(),
        Some(right_requests),
    );
    let avg_output_tps = weighted_average(
        out.avg_output_tps(),
        Some(left_requests),
        right.avg_output_tps(),
        Some(right_requests),
    );
    out.set_avg_duration_ms(avg_duration);
    out.set_p50_duration_ms(None);
    out.set_p95_duration_ms(None);
    out.set_avg_ttfb_ms(avg_ttfb);
    out.set_p50_ttfb_ms(None);
    out.set_p95_ttfb_ms(None);
    out.set_avg_output_tps(avg_output_tps);
    out.set_sessions(out.sessions() + right.sessions());
    out.set_requests(left_requests as i64 + right.requests());
    out.set_unique_users(out.unique_users() + right.unique_users());
    out.set_input_tokens(out.input_tokens() + right.input_tokens());
    out.set_output_tokens(out.output_tokens() + right.output_tokens());
    out.set_reasoning_tokens(out.reasoning_tokens() + right.reasoning_tokens());
    out.set_cache_read_tokens(out.cache_read_tokens() + right.cache_read_tokens());
    out.set_total_tokens(out.total_tokens() + right.total_tokens());
    out.set_input_cost_microcents(out.input_cost_microcents() + right.input_cost_microcents());
    out.set_output_cost_microcents(out.output_cost_microcents() + right.output_cost_microcents());
    out.set_total_cost_microcents(out.total_cost_microcents() + right.total_cost_microcents());
    out.set_success_count(out.success_count() + right.success_count());
    out.set_error_count(out.error_count() + right.error_count());
    out.set_sample_count(out.sample_count() + right.sample_count());
    out
}

/// `isMissingUniqueUsersColumn`: planetscale reports this when the legacy
/// `unique_users` column is absent from the table.
pub fn is_missing_unique_users_column(cause: &dyn std::fmt::Display) -> bool {
    error_text_from_string(cause.to_string()).contains("Unknown column 'unique_users'")
}

/// `omitUniqueUsers`: drops the `unique_users` field from each row.
pub fn omit_unique_users<T: serde::Serialize>(rows: &[T]) -> Vec<serde_json::Value> {
    rows.iter()
        .map(|row| {
            let mut value = serde_json::to_value(row).unwrap_or(serde_json::Value::Null);
            if let serde_json::Value::Object(map) = &mut value {
                map.remove("unique_users");
            }
            value
        })
        .collect()
}

/// `statPeriodKey`: scope key used for grouping/ranking rows of one period.
pub fn stat_period_key<R: StatBaseRowLike>(row: &R) -> String {
    [
        row.grain_key(),
        row.period_key(),
        row.dataset_key(),
        row.tier_key(),
        row.client_key(),
        row.source_key(),
    ]
    .join("\u{0}")
}

pub struct StatRowScope {
    pub grains: Vec<String>,
    pub period_keys: Vec<String>,
    pub datasets: Vec<String>,
    pub clients: Vec<String>,
    pub sources: Vec<String>,
}

/// `statRowScope`: unique scope values across a row batch.
pub fn stat_row_scope<R: StatBaseRowLike>(rows: &[R]) -> Option<StatRowScope> {
    if rows.is_empty() {
        return None;
    }
    Some(StatRowScope {
        grains: unique(rows.iter().map(|row| row.grain_key())),
        period_keys: unique(rows.iter().map(|row| row.period_key())),
        datasets: unique(rows.iter().map(|row| row.dataset_key())),
        clients: unique(rows.iter().map(|row| row.client_key())),
        sources: unique(rows.iter().map(|row| row.source_key())),
    })
}

/// `periodKeyFor`: key of the bucket containing the given start-of-period date.
pub fn period_key_for(grain: StatGrain, period_start: &date_like::Date) -> String {
    match grain {
        StatGrain::Week => iso_week_id(period_start),
        StatGrain::Day => utc_date_id(period_start),
    }
}

/// Date helpers mimic the ECMAScript Date UTC semantics used by the source.
pub mod date_like {
    /// Minimal UTC date value (year, month 1-12, day).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Date {
        pub year: i32,
        pub month: u32,
        pub day: u32,
        /// Epoch milliseconds (UTC midnight of this day).
        pub ms: i64,
    }

    impl Date {
        pub fn from_ms(ms: i64) -> Date {
            // civil-from-days over proleptic Gregorian, UTC.
            let days = ms.div_euclid(86_400_000);
            let (year, month, day) = civil_from_days(days);
            Date {
                year,
                month,
                day,
                ms,
            }
        }

        pub fn utc(year: i32, month: u32, day: u32) -> Date {
            let days = days_from_civil(year, month, day);
            Date {
                year,
                month,
                day,
                ms: days * 86_400_000,
            }
        }

        pub fn add_days(&self, days: i64) -> Date {
            Date::from_ms(self.ms + days * 86_400_000)
        }

        pub fn add_ms(&self, delta: i64) -> Date {
            Date::from_ms(self.ms + delta)
        }

        pub fn weekday(&self) -> u32 {
            // 0 = Sunday (ECMAScript getUTCDay: 0=Sun .. 6=Sat).
            let days = self.ms.div_euclid(86_400_000);
            ((days + 4) % 7) as u32
        }

        pub fn iso(&self) -> String {
            format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
        }
    }

    fn days_from_civil(y: i32, m: u32, d: u32) -> i64 {
        let y = if m <= 2 { y - 1 } else { y };
        let era = (if y >= 0 { y } else { y - 399 }) / 400;
        let yoe = (y - era * 400) as i64;
        let mp = ((m + 9) % 12) as i64;
        let doy = (153 * mp + 2) / 5 + d as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era as i64 * 146_097 + doe - 719_468
    }

    /// Civil date (year, month, day) from epoch days, proleptic Gregorian, UTC.
    pub fn civil_from_days(z: i64) -> (i32, u32, u32) {
        let z = z + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        ((if m <= 2 { y + 1 } else { y }) as i32, m, d)
    }
}

/// `startOfUtcDay`: midnight UTC of the value's UTC date.
pub fn start_of_utc_day(value: &date_like::Date) -> date_like::Date {
    date_like::Date::utc(value.year, value.month, value.day)
}

/// `startOfIsoWeek`: Monday 00:00 UTC of the ISO week containing the value
/// (`Date.UTC(y, m, date - (dow || 7) + 1)` — the UTC constructor floors to
/// midnight, so the day is floored before stepping back to Monday).
pub fn start_of_iso_week(value: &date_like::Date) -> date_like::Date {
    let weekday = value.weekday(); // 0 Sun .. 6 Sat
    let days_since_monday = if weekday == 0 { 6 } else { weekday - 1 };
    start_of_utc_day(value).add_days(-(days_since_monday as i64))
}

/// `isoWeekId`: `YYYY-Www` of the ISO week containing the value.
pub fn iso_week_id(value: &date_like::Date) -> String {
    // Thursday of the same ISO week determines the year.
    let thursday = value.add_days(4 - value.weekday() as i64);
    let jan1 = date_like::Date::utc(thursday.year, 1, 1);
    let week = (thursday.ms - jan1.ms) / (7 * DAY_MS) + 1;
    let week = week.clamp(1, 53);
    format!("{:04}-W{:02}", thursday.year, week)
}

fn utc_date_id(value: &date_like::Date) -> String {
    format!("{:04}-{:02}-{:02}", value.year, value.month, value.day)
}

/// `rankBy`: 1-based ranks (descending by value) for the given scored entries.
pub fn rank_by(scored: &[(usize, i64)]) -> Vec<(usize, i64)> {
    let mut indexed: Vec<(usize, i64)> = scored.to_vec();
    indexed.sort_by(|a, b| b.1.cmp(&a.1));
    indexed
        .iter()
        .enumerate()
        .map(|(rank, (index, _))| (*index, rank as i64 + 1))
        .collect()
}

/// Shares (0..1) of value/total, rounded to 6 decimals, null when total <= 0.
pub fn share(value: Option<i64>, total: i64) -> Option<f64> {
    if total <= 0 {
        return None;
    }
    Some(round(value.unwrap_or(0) as f64 / total as f64, 6))
}

/// `rankRowsWithMarketShare`: groups rows by `group_key` (defaults to the
/// period scope key), then computes market-share fractions and ranks for
/// tokens/requests/sessions/cost per group.
pub fn rank_rows_with_market_share<T: StatBaseRowLike>(
    rows: &[T],
    group_key: impl Fn(&T) -> String,
) -> Vec<T> {
    let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let key = group_key(row);
        if let Some(group) = groups.iter_mut().find(|(k, _)| *k == key) {
            group.1.push(index);
        } else {
            groups.push((key, vec![index]));
        }
    }
    let mut out: Vec<T> = rows.to_vec();
    for (_, indices) in groups {
        let tokens: i64 = indices.iter().map(|&i| rows[i].total_tokens()).sum();
        let requests: i64 = indices.iter().map(|&i| rows[i].requests()).sum();
        let sessions: i64 = indices.iter().map(|&i| rows[i].sessions()).sum();
        let token_ranks = rank_by_group(&indices, |i| rows[*i].total_tokens());
        let request_ranks = rank_by_group(&indices, |i| rows[*i].requests());
        let session_ranks = rank_by_group(&indices, |i| rows[*i].sessions());
        let cost_ranks = rank_by_group(&indices, |i| rows[*i].total_cost_microcents());
        for &i in &indices {
            out[i].set_market_share_tokens(share(Some(rows[i].total_tokens()), tokens));
            out[i].set_market_share_requests(share(Some(rows[i].requests()), requests));
            out[i].set_market_share_sessions(share(Some(rows[i].sessions()), sessions));
            out[i].set_rank_by_tokens(rank_lookup(&token_ranks, i));
            out[i].set_rank_by_requests(rank_lookup(&request_ranks, i));
            out[i].set_rank_by_sessions(rank_lookup(&session_ranks, i));
            out[i].set_rank_by_cost(rank_lookup(&cost_ranks, i));
        }
    }
    out
}

/// Computes 1-based ranks within a group of row indices (descending by value).
pub fn rank_by_group(indices: &[usize], value: impl Fn(&usize) -> i64) -> Vec<(usize, i64)> {
    let scored: Vec<(usize, i64)> = indices.iter().map(|i| (*i, value(i))).collect();
    rank_by(&scored)
}

pub fn rank_lookup(ranks: &[(usize, i64)], index: usize) -> Option<i64> {
    ranks.iter().find(|(i, _)| *i == index).map(|(_, r)| *r)
}

/// `chunks`: splits items into fixed-size chunks (last chunk smaller).
pub fn chunks<T: Clone>(items: &[T], size: usize) -> Vec<Vec<T>> {
    items.chunks(size).map(|chunk| chunk.to_vec()).collect()
}

/// `inserted`: the MySQL `values(col)` expression used in on-duplicate updates.
pub fn inserted(column: &str) -> String {
    format!("values(`{column}`)")
}

fn unique(values: impl Iterator<Item = String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    values.filter(|value| seen.insert(value.clone())).collect()
}

/// `weightedAverage`: averages two nullable values weighted by their weights,
/// rounded to 2 decimals; null when total weight is zero.
pub fn weighted_average(
    left: Option<f64>,
    left_weight: Option<f64>,
    right: Option<f64>,
    right_weight: Option<f64>,
) -> Option<f64> {
    let left_weight = if left.is_some() {
        left_weight.unwrap_or(0.0)
    } else {
        0.0
    };
    let right_weight = if right.is_some() {
        right_weight.unwrap_or(0.0)
    } else {
        0.0
    };
    let total_weight = left_weight + right_weight;
    if total_weight == 0.0 {
        return None;
    }
    Some(round(
        (left.unwrap_or(0.0) * left_weight + right.unwrap_or(0.0) * right_weight) / total_weight,
        2,
    ))
}

/// `normalizeTier`: canonical tier names for the site.
pub fn normalize_tier(value: &str) -> String {
    match value.to_lowercase().as_str() {
        "paid" | "zen" => "Zen".to_string(),
        "go" => "Go".to_string(),
        "free" => "Free".to_string(),
        "enterprise" => "Enterprise".to_string(),
        "all" => "all".to_string(),
        _ => value.to_string(),
    }
}

/// `normalizeCountry`: uppercase 2-letter country code, `ZZ` when absent/invalid.
pub fn normalize_country(value: Option<&str>) -> String {
    match value {
        Some(v) if v.len() == 2 => v.to_uppercase(),
        _ => "ZZ".to_string(),
    }
}

fn error_text_from_string(text: String) -> String {
    text
}

pub fn round(value: f64, digits: u32) -> f64 {
    let factor = 10_f64.powi(digits as i32);
    (value * factor).round() / factor
}

/// Implements [`StatBaseRowLike`] for a stat row struct with the standard base
/// field names (`grain`, `period_key`, `dataset`, `tier`, `client`, `source`
/// and the numeric metric columns as `Option<T>`).
///
/// Variants:
/// - plain: base metrics only (market-share/rank columns are absent)
/// - `ranks`: additionally implements `rank_by_tokens/requests/cost`
/// - `market`: additionally implements market-share and all rank columns
///
/// The struct must define its own `dimension_key(&self) -> String`.
#[macro_export]
macro_rules! impl_stat_base_row {
    ($ty:ty) => {
        impl $crate::domain::stat::StatBaseRowLike for $ty {
            $crate::impl_stat_base_row!(@base);
            fn market_share_tokens(&self) -> Option<f64> { None }
            fn set_market_share_tokens(&mut self, _v: Option<f64>) {}
            fn market_share_requests(&self) -> Option<f64> { None }
            fn set_market_share_requests(&mut self, _v: Option<f64>) {}
            fn market_share_sessions(&self) -> Option<f64> { None }
            fn set_market_share_sessions(&mut self, _v: Option<f64>) {}
            fn rank_by_tokens(&self) -> Option<i64> { None }
            fn set_rank_by_tokens(&mut self, _v: Option<i64>) {}
            fn rank_by_requests(&self) -> Option<i64> { None }
            fn set_rank_by_requests(&mut self, _v: Option<i64>) {}
            fn rank_by_sessions(&self) -> Option<i64> { None }
            fn set_rank_by_sessions(&mut self, _v: Option<i64>) {}
            fn rank_by_cost(&self) -> Option<i64> { None }
            fn set_rank_by_cost(&mut self, _v: Option<i64>) {}
        }
    };
    ($ty:ty, ranks) => {
        impl $crate::domain::stat::StatBaseRowLike for $ty {
            $crate::impl_stat_base_row!(@base);
            fn market_share_tokens(&self) -> Option<f64> { None }
            fn set_market_share_tokens(&mut self, _v: Option<f64>) {}
            fn market_share_requests(&self) -> Option<f64> { None }
            fn set_market_share_requests(&mut self, _v: Option<f64>) {}
            fn market_share_sessions(&self) -> Option<f64> { None }
            fn set_market_share_sessions(&mut self, _v: Option<f64>) {}
            fn rank_by_tokens(&self) -> Option<i64> { self.rank_by_tokens }
            fn set_rank_by_tokens(&mut self, v: Option<i64>) { self.rank_by_tokens = v; }
            fn rank_by_requests(&self) -> Option<i64> { self.rank_by_requests }
            fn set_rank_by_requests(&mut self, v: Option<i64>) { self.rank_by_requests = v; }
            fn rank_by_sessions(&self) -> Option<i64> { None }
            fn set_rank_by_sessions(&mut self, _v: Option<i64>) {}
            fn rank_by_cost(&self) -> Option<i64> { self.rank_by_cost }
            fn set_rank_by_cost(&mut self, v: Option<i64>) { self.rank_by_cost = v; }
        }
    };
    ($ty:ty, market) => {
        impl $crate::domain::stat::StatBaseRowLike for $ty {
            $crate::impl_stat_base_row!(@base);
            fn market_share_tokens(&self) -> Option<f64> { self.market_share_tokens }
            fn set_market_share_tokens(&mut self, v: Option<f64>) { self.market_share_tokens = v; }
            fn market_share_requests(&self) -> Option<f64> { self.market_share_requests }
            fn set_market_share_requests(&mut self, v: Option<f64>) { self.market_share_requests = v; }
            fn market_share_sessions(&self) -> Option<f64> { self.market_share_sessions }
            fn set_market_share_sessions(&mut self, v: Option<f64>) { self.market_share_sessions = v; }
            fn rank_by_tokens(&self) -> Option<i64> { self.rank_by_tokens }
            fn set_rank_by_tokens(&mut self, v: Option<i64>) { self.rank_by_tokens = v; }
            fn rank_by_requests(&self) -> Option<i64> { self.rank_by_requests }
            fn set_rank_by_requests(&mut self, v: Option<i64>) { self.rank_by_requests = v; }
            fn rank_by_sessions(&self) -> Option<i64> { self.rank_by_sessions }
            fn set_rank_by_sessions(&mut self, v: Option<i64>) { self.rank_by_sessions = v; }
            fn rank_by_cost(&self) -> Option<i64> { self.rank_by_cost }
            fn set_rank_by_cost(&mut self, v: Option<i64>) { self.rank_by_cost = v; }
        }
    };
    (@base) => {
        fn grain_key(&self) -> String { self.grain.clone() }
        fn period_key(&self) -> String { self.period_key.clone() }
        fn dataset_key(&self) -> String { self.dataset.clone().unwrap_or_else(|| "all".to_string()) }
        fn tier_key(&self) -> String { self.tier.clone().unwrap_or_else(|| "all".to_string()) }
        fn client_key(&self) -> String { self.client.clone().unwrap_or_else(|| "all".to_string()) }
        fn source_key(&self) -> String { self.source.clone().unwrap_or_else(|| "all".to_string()) }
        fn dimension_key(&self) -> String { self.dimension_key() }
        fn set_tier(&mut self, tier: String) { self.tier = Some(tier); }
        fn sessions(&self) -> i64 { self.sessions.unwrap_or(0) }
        fn set_sessions(&mut self, v: i64) { self.sessions = Some(v); }
        fn requests(&self) -> i64 { self.requests.unwrap_or(0) }
        fn set_requests(&mut self, v: i64) { self.requests = Some(v); }
        fn unique_users(&self) -> i64 { self.unique_users.unwrap_or(0) }
        fn set_unique_users(&mut self, v: i64) { self.unique_users = Some(v); }
        fn input_tokens(&self) -> i64 { self.input_tokens.unwrap_or(0) }
        fn set_input_tokens(&mut self, v: i64) { self.input_tokens = Some(v); }
        fn output_tokens(&self) -> i64 { self.output_tokens.unwrap_or(0) }
        fn set_output_tokens(&mut self, v: i64) { self.output_tokens = Some(v); }
        fn reasoning_tokens(&self) -> i64 { self.reasoning_tokens.unwrap_or(0) }
        fn set_reasoning_tokens(&mut self, v: i64) { self.reasoning_tokens = Some(v); }
        fn cache_read_tokens(&self) -> i64 { self.cache_read_tokens.unwrap_or(0) }
        fn set_cache_read_tokens(&mut self, v: i64) { self.cache_read_tokens = Some(v); }
        fn total_tokens(&self) -> i64 { self.total_tokens.unwrap_or(0) }
        fn set_total_tokens(&mut self, v: i64) { self.total_tokens = Some(v); }
        fn input_cost_microcents(&self) -> i64 { self.input_cost_microcents.unwrap_or(0) }
        fn set_input_cost_microcents(&mut self, v: i64) { self.input_cost_microcents = Some(v); }
        fn output_cost_microcents(&self) -> i64 { self.output_cost_microcents.unwrap_or(0) }
        fn set_output_cost_microcents(&mut self, v: i64) { self.output_cost_microcents = Some(v); }
        fn total_cost_microcents(&self) -> i64 { self.total_cost_microcents.unwrap_or(0) }
        fn set_total_cost_microcents(&mut self, v: i64) { self.total_cost_microcents = Some(v); }
        fn avg_duration_ms(&self) -> Option<f64> { self.avg_duration_ms }
        fn set_avg_duration_ms(&mut self, v: Option<f64>) { self.avg_duration_ms = v; }
        fn p50_duration_ms(&self) -> Option<i64> { self.p50_duration_ms }
        fn set_p50_duration_ms(&mut self, v: Option<i64>) { self.p50_duration_ms = v; }
        fn p95_duration_ms(&self) -> Option<i64> { self.p95_duration_ms }
        fn set_p95_duration_ms(&mut self, v: Option<i64>) { self.p95_duration_ms = v; }
        fn avg_ttfb_ms(&self) -> Option<f64> { self.avg_ttfb_ms }
        fn set_avg_ttfb_ms(&mut self, v: Option<f64>) { self.avg_ttfb_ms = v; }
        fn p50_ttfb_ms(&self) -> Option<i64> { self.p50_ttfb_ms }
        fn set_p50_ttfb_ms(&mut self, v: Option<i64>) { self.p50_ttfb_ms = v; }
        fn p95_ttfb_ms(&self) -> Option<i64> { self.p95_ttfb_ms }
        fn set_p95_ttfb_ms(&mut self, v: Option<i64>) { self.p95_ttfb_ms = v; }
        fn avg_output_tps(&self) -> Option<f64> { self.avg_output_tps }
        fn set_avg_output_tps(&mut self, v: Option<f64>) { self.avg_output_tps = v; }
        fn success_count(&self) -> i64 { self.success_count.unwrap_or(0) }
        fn set_success_count(&mut self, v: i64) { self.success_count = Some(v); }
        fn error_count(&self) -> i64 { self.error_count.unwrap_or(0) }
        fn set_error_count(&mut self, v: i64) { self.error_count = Some(v); }
        fn sample_count(&self) -> i64 { self.sample_count.unwrap_or(0) }
        fn set_sample_count(&mut self, v: i64) { self.sample_count = Some(v); }
    };
}

/// Implements `From<&StatBaseRow>` for a stat row struct so each table module
/// can seed its row from the shared base metrics (base fields copied 1:1).
#[macro_export]
macro_rules! impl_from_stat_base {
    ($ty:ty) => {
        impl From<&$crate::domain::stat::StatBaseRow> for $ty {
            fn from(base: &$crate::domain::stat::StatBaseRow) -> Self {
                let mut out = <$ty>::default();
                out.grain = base.grain.clone();
                out.period_key = base.period_key.clone();
                out.dataset = base.dataset.clone();
                out.tier = base.tier.clone();
                out.client = base.client.clone();
                out.source = base.source.clone();
                out.sessions = base.sessions;
                out.requests = base.requests;
                out.unique_users = base.unique_users;
                out.input_tokens = base.input_tokens;
                out.output_tokens = base.output_tokens;
                out.reasoning_tokens = base.reasoning_tokens;
                out.cache_read_tokens = base.cache_read_tokens;
                out.total_tokens = base.total_tokens;
                out.input_cost_microcents = base.input_cost_microcents;
                out.output_cost_microcents = base.output_cost_microcents;
                out.total_cost_microcents = base.total_cost_microcents;
                out.avg_duration_ms = base.avg_duration_ms;
                out.p50_duration_ms = base.p50_duration_ms;
                out.p95_duration_ms = base.p95_duration_ms;
                out.avg_ttfb_ms = base.avg_ttfb_ms;
                out.p50_ttfb_ms = base.p50_ttfb_ms;
                out.p95_ttfb_ms = base.p95_ttfb_ms;
                out.avg_output_tps = base.avg_output_tps;
                out.success_count = base.success_count;
                out.error_count = base.error_count;
                out.sample_count = base.sample_count;
                out
            }
        }
    };
}
