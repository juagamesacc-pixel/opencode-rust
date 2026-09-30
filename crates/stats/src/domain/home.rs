// source: packages/stats/core/src/domain/home.ts (1:1 port — pure aggregation logic)

use crate::domain::model_normalization::stat_provider;
use crate::domain::stat::{date_like, normalize_tier, round};
use std::collections::HashMap;

pub type UsageProduct = &'static str;
pub type TokenProduct = &'static str;
pub type UsageRange = &'static str;

pub const USAGE_PRODUCTS: [UsageProduct; 4] = ["All Users", "Zen", "Go", "Enterprise"];
pub const TOKEN_PRODUCTS: [TokenProduct; 3] = ["Zen", "Go", "Enterprise"];
pub const USAGE_RANGES: [UsageRange; 8] = ["1D", "1W", "2W", "1M", "2M", "3M", "YTD", "ALL"];

#[derive(Debug, Clone, serde::Serialize)]
pub struct UsagePoint {
    pub date: String,
    pub segments: Vec<UsageSegment>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UsageSegment {
    pub model: String,
    pub value: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MarketDay {
    pub date: String,
    pub total: f64,
    pub authors: Vec<MarketAuthor>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MarketAuthor {
    pub author: String,
    pub share: f64,
    pub tokens: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LeaderboardEntry {
    pub model: String,
    pub provider: String,
    pub author: String,
    pub tokens: i64,
    pub change: Option<i64>,
    pub rank: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct TokenCostEntry {
    pub model: String,
    pub total: f64,
    pub input: f64,
    pub output: f64,
    pub cached: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CacheRatioEntry {
    pub model: String,
    pub ratio: f64,
    pub cached: f64,
    pub uncached: f64,
    pub total: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionCostEntry {
    pub model: String,
    pub cost: f64,
    pub tokens: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct RetentionEntry {
    pub model: String,
    pub provider: String,
    pub author: String,
    pub rate: f64,
    pub eligible_user_weeks: i64,
    pub retained_user_weeks: i64,
    pub rank: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CountryEntry {
    pub country: String,
    pub continent: String,
    pub tokens: f64,
    pub share: f64,
    pub rank: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ModelUsagePoint {
    pub date: String,
    pub tokens: i64,
    pub users: i64,
    pub sessions: i64,
    pub cost: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ModelMixEntry {
    pub label: &'static str,
    pub tokens: i64,
    pub share: f64,
}

/// `StatsHomeData`: response shape of the home endpoint.
#[derive(Debug, Clone, serde::Serialize)]
pub struct StatsHomeData {
    pub updated_at: Option<String>,
    pub usage: HashMap<UsageProduct, HashMap<UsageRange, Vec<UsagePoint>>>,
    pub users: HashMap<UsageProduct, HashMap<UsageRange, Vec<UsagePoint>>>,
    pub leaderboard: HashMap<UsageProduct, HashMap<UsageRange, Vec<LeaderboardEntry>>>,
    pub market: HashMap<UsageRange, Vec<MarketDay>>,
    pub token_cost: HashMap<TokenProduct, Vec<TokenCostEntry>>,
    pub cache_ratio: HashMap<TokenProduct, Vec<CacheRatioEntry>>,
    pub session_cost: HashMap<TokenProduct, Vec<SessionCostEntry>>,
    pub retention: Vec<RetentionEntry>,
    pub country: Vec<CountryEntry>,
}

/// `ModelPeerEntry`: model-detail peers.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ModelPeerEntry {
    pub model: String,
    pub provider: String,
    pub author: String,
    pub rank: i64,
    pub tokens: i64,
    pub share: f64,
    pub slug: String,
}

/// `ProviderAggregate`: provider token totals.
#[derive(Debug, Clone)]
pub struct ProviderAggregate {
    pub provider: String,
    pub tokens: i64,
}

const DAY_MS: i64 = 86_400_000;
const TOKEN_SCALE: i64 = 1_000_000;
const DOLLARS_PER_MICROCENT: f64 = 1.0 / 100_000_000.0;
const METRIC_MODEL_LIMIT: usize = 10;
/// `RETENTION_MODEL_LIMIT` (`home.ts:143`): caps the ranked retention entries
/// at the home-data assembly site (`.slice(0, RETENTION_MODEL_LIMIT)`).
/// Public while that assembly is unported so the source constant stays named.
pub const RETENTION_MODEL_LIMIT: usize = 15;
const RETENTION_MIN_ELIGIBLE_USER_WEEKS: i64 = 100;
const RETENTION_COHORT_WEEKS: usize = 7;
const TOP_MODEL_SEGMENT_LIMIT: usize = 9;
/// Preserve the response shape while the public site presents Go and Free as one cohort.
const SITE_PRODUCT: &str = "Go";
/// `SITE_TIER_PLACEHOLDERS`: one `?` per site tier for SQL parameter binding.
pub const SITE_TIER_PLACEHOLDERS: &str = "?, ?, ?, ?";
const LEADERBOARD_CHANGE_MIN_MULTIPLE: i64 = 10;
const MONTHS: [&str; 12] = [
    "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
];

/// `StatMetricRow`: normalized daily metric row with resolved period/update timestamps.
#[derive(Debug, Clone)]
pub struct StatMetricRow {
    pub model: String,
    pub provider: String,
    pub tier: String,
    pub period_start: i64,
    pub updated_at: i64,
    pub sessions: i64,
    pub unique_users: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub reasoning_tokens: i64,
    pub cache_read_tokens: i64,
    pub total_tokens: i64,
    pub input_cost_microcents: i64,
    pub output_cost_microcents: i64,
    pub total_cost_microcents: i64,
}

/// `CountryTotalRow`: country column totals from `geo_stat`.
#[derive(Debug, Clone)]
pub struct CountryTotalRow {
    pub country: String,
    pub continent: String,
    pub tokens: i64,
    pub updated_at: i64,
}

/// `RetentionMetricRow`: weekly retention metric from `model_retention`.
#[derive(Debug, Clone)]
pub struct RetentionMetricRow {
    pub cohort_date: String,
    pub updated_at: i64,
    pub provider: String,
    pub model: String,
    pub eligible_users: i64,
    pub retained_users: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct DateWindow {
    pub start: i64,
    pub end: i64,
    pub previous_start: i64,
    pub previous_end: i64,
}

/// `Bucket`: one time bucket with its display label.
#[derive(Debug, Clone)]
pub struct Bucket {
    pub start: i64,
    pub end: i64,
    pub label: String,
}

#[derive(Debug, Clone)]
pub struct ModelAggregate {
    pub model: String,
    pub provider: String,
    pub sessions: i64,
    pub unique_users: i64,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub reasoning_tokens: i64,
    pub cache_read_tokens: i64,
    pub total_tokens: i64,
    pub input_cost_microcents: i64,
    pub output_cost_microcents: i64,
    pub total_cost_microcents: i64,
}

/// `normalizeStatRow`: resolves period start / update timestamps and canonical
/// tier/provider/model for one daily metric row.
pub fn normalize_stat_row(row: &crate::domain::model::ModelStatMetric) -> Vec<StatMetricRow> {
    let period_start = period_key_time(&row.period_key);
    let updated_at = parse_iso_8601(&row.updated_at);
    let (Some(period_start), Some(updated_at)) = (period_start, updated_at) else {
        return Vec::new();
    };
    let provider = stat_provider(Some(&row.model), None, Some(&row.provider))
        .unwrap_or_else(|| "unknown".to_string());
    vec![StatMetricRow {
        model: if row.model.is_empty() {
            "unknown".to_string()
        } else {
            row.model.clone()
        },
        provider,
        tier: normalize_tier(&row.tier),
        period_start,
        updated_at,
        sessions: row.sessions,
        unique_users: row.unique_users,
        input_tokens: row.input_tokens,
        output_tokens: row.output_tokens,
        reasoning_tokens: row.reasoning_tokens,
        cache_read_tokens: row.cache_read_tokens,
        total_tokens: row.total_tokens,
        input_cost_microcents: row.input_cost_microcents,
        output_cost_microcents: row.output_cost_microcents,
        total_cost_microcents: row.total_cost_microcents,
    }]
}

/// `buildRetentionEntries`: weekly retention entries with ranks.
pub fn build_retention_entries(rows: &[RetentionMetricRow]) -> Vec<RetentionEntry> {
    let mut cohort_dates: Vec<String> = Vec::new();
    for row in rows {
        if !cohort_dates.contains(&row.cohort_date) {
            cohort_dates.push(row.cohort_date.clone());
        }
    }
    cohort_dates.sort();
    let cohort_dates: Vec<String> =
        cohort_dates[cohort_dates.len().saturating_sub(RETENTION_COHORT_WEEKS)..].to_vec();

    let mut aggregated: Vec<(String, String, i64, i64)> = Vec::new(); // model, provider, eligible, retained
    for row in rows
        .iter()
        .filter(|row| cohort_dates.contains(&row.cohort_date))
    {
        if let Some(current) = aggregated
            .iter_mut()
            .find(|(model, _, _, _)| *model == row.model)
        {
            current.2 += row.eligible_users;
            current.3 += row.retained_users;
        } else {
            aggregated.push((
                row.model.clone(),
                row.provider.clone(),
                row.eligible_users,
                row.retained_users,
            ));
        }
    }

    let mut entries: Vec<RetentionEntry> = aggregated
        .iter()
        .map(|(model, provider, eligible, retained)| RetentionEntry {
            model: model.clone(),
            provider: provider.clone(),
            author: format_provider(provider),
            rate: if *eligible > 0 {
                round(*retained as f64 / *eligible as f64 * 100.0, 1)
            } else {
                0.0
            },
            eligible_user_weeks: *eligible,
            retained_user_weeks: *retained,
            rank: None,
        })
        .collect();

    let mut ranked: Vec<&RetentionEntry> = entries
        .iter()
        .filter(|item| item.eligible_user_weeks >= RETENTION_MIN_ELIGIBLE_USER_WEEKS)
        .collect();
    ranked.sort_by(|a, b| {
        b.rate
            .partial_cmp(&a.rate)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.eligible_user_weeks.cmp(&a.eligible_user_weeks))
            .then_with(|| a.model.cmp(&b.model))
    });
    let ranks: HashMap<String, i64> = ranked
        .iter()
        .enumerate()
        .map(|(index, item)| (item.model.clone(), index as i64 + 1))
        .collect();
    for entry in &mut entries {
        entry.rank = ranks.get(&entry.model).copied();
    }
    entries.sort_by(|a, b| a.rank.unwrap_or(i64::MAX).cmp(&b.rank.unwrap_or(i64::MAX)));
    entries
}

/// `buildUsagePoints`: per-bucket stacked segments (top models + `Other`).
pub fn build_usage_points(
    rows: &[StatMetricRow],
    product: UsageProduct,
    range: UsageRange,
    window: &DateWindow,
    rank_window: &DateWindow,
    metric: &str,
) -> Vec<UsagePoint> {
    let mut model_order: Vec<(String, f64)> = aggregate_by_model_name(&rows_for_product(
        rows,
        product,
        rank_window.start,
        rank_window.end,
    ))
    .into_iter()
    .map(|item| (item.model.clone(), model_usage_value(&item, metric)))
    .collect();
    model_order.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    model_order.truncate(TOP_MODEL_SEGMENT_LIMIT);

    create_buckets(window, range)
        .into_iter()
        .map(|bucket| {
            let bucket_rows =
                aggregate_by_model_name(&rows_for_product(rows, product, bucket.start, bucket.end));
            let by_model: HashMap<String, f64> = bucket_rows
                .iter()
                .map(|item| (item.model.clone(), model_usage_value(item, metric)))
                .collect();
            let segments: Vec<(String, f64)> = model_order
                .iter()
                .map(|(model, _)| (model.clone(), by_model.get(model).copied().unwrap_or(0.0)))
                .collect();
            let known_value: f64 = segments.iter().map(|(_, value)| value).sum();
            let total_value: f64 = bucket_rows
                .iter()
                .map(|item| model_usage_value(item, metric))
                .sum();
            let mut out_segments: Vec<UsageSegment> = segments
                .iter()
                .map(|(model, value)| UsageSegment {
                    model: model.clone(),
                    value: usage_point_value(*value, metric),
                })
                .collect();
            out_segments.push(UsageSegment {
                model: "Other".to_string(),
                value: usage_point_value((total_value - known_value).max(0.0), metric),
            });
            UsagePoint {
                date: bucket.label,
                segments: out_segments,
            }
        })
        .collect()
}

fn model_usage_value(item: &ModelAggregate, metric: &str) -> f64 {
    if metric == "users" {
        item.unique_users as f64
    } else {
        item.total_tokens as f64
    }
}

fn usage_point_value(value: f64, metric: &str) -> f64 {
    if metric == "users" {
        value
    } else {
        round(value / 1_000_000_000_000.0, 4)
    }
}

/// `buildLeaderboard`: top-18 models for one product's current 1W window.
pub fn build_leaderboard(
    rows: &[StatMetricRow],
    product: UsageProduct,
    rank_window: &DateWindow,
) -> Vec<LeaderboardEntry> {
    let previous: HashMap<String, i64> = aggregate_by_model_name(&rows_for_product(
        rows,
        product,
        rank_window.previous_start,
        rank_window.previous_end,
    ))
    .into_iter()
    .map(|item| (item.model.clone(), item.total_tokens))
    .collect();
    let mut current = aggregate_by_model_name(&rows_for_product(
        rows,
        product,
        rank_window.start,
        rank_window.end,
    ));
    current.sort_by(|a, b| {
        b.total_tokens
            .cmp(&a.total_tokens)
            .then_with(|| a.model.cmp(&b.model))
    });
    current
        .into_iter()
        .take(18)
        .enumerate()
        .map(|(index, item)| LeaderboardEntry {
            model: item.model.clone(),
            provider: item.provider.clone(),
            author: format_provider(&item.provider),
            tokens: (item.total_tokens as f64 / 1_000_000_000.0).round() as i64,
            change: leaderboard_change(
                item.total_tokens,
                previous.get(&item.model).copied().unwrap_or(0),
            ),
            rank: index as i64 + 1,
        })
        .collect()
}

/// `buildMarketShare`: per-bucket provider market-share days.
pub fn build_market_share(
    rows: &[StatMetricRow],
    product: UsageProduct,
    range: UsageRange,
    window: &DateWindow,
) -> Vec<MarketDay> {
    let mut provider_order: Vec<(String, i64)> =
        aggregate_by_provider(&rows_for_product(rows, product, window.start, window.end))
            .into_iter()
            .filter(|item| item.provider != "unknown")
            .map(|item| (item.provider.clone(), item.tokens))
            .collect();
    provider_order.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    provider_order.truncate(8);

    create_buckets(window, range)
        .into_iter()
        .flat_map(|bucket| {
            let total =
                aggregate_by_provider(&rows_for_product(rows, product, bucket.start, bucket.end));
            let total_tokens: i64 = total.iter().map(|item| item.tokens).sum();
            if total_tokens == 0 {
                return Vec::new();
            }
            let by_provider: HashMap<String, i64> = total
                .iter()
                .map(|item| (item.provider.clone(), item.tokens))
                .collect();
            let mut authors: Vec<(String, i64)> = provider_order
                .iter()
                .map(|(provider, _)| {
                    (
                        provider.clone(),
                        by_provider.get(provider).copied().unwrap_or(0),
                    )
                })
                .collect();
            let known_tokens: i64 = authors.iter().map(|(_, tokens)| tokens).sum();
            authors.push(("Other".to_string(), (total_tokens - known_tokens).max(0)));
            let authors: Vec<(String, i64)> = authors
                .into_iter()
                .filter(|(_, tokens)| *tokens > 0)
                .collect();
            vec![MarketDay {
                date: bucket.label,
                total: round(total_tokens as f64 / 1_000_000_000_000.0, 6),
                authors: authors
                    .iter()
                    .map(|(provider, tokens)| MarketAuthor {
                        author: if provider == "Other" {
                            "Other".to_string()
                        } else {
                            format_provider(provider)
                        },
                        share: round(*tokens as f64 / total_tokens as f64 * 100.0, 1),
                        tokens: round(*tokens as f64 / 1_000_000_000_000.0, 6),
                    })
                    .collect(),
            }]
        })
        .collect()
}

/// `buildCountryStats`: ranked country token shares.
pub fn build_country_stats(rows: &[CountryTotalRow]) -> Vec<CountryEntry> {
    let mut countries: Vec<&CountryTotalRow> = rows
        .iter()
        .filter(|item| item.tokens > 0 && item.country != "AQ")
        .collect();
    countries.sort_by(|a, b| b.tokens.cmp(&a.tokens));
    let total_tokens: i64 = countries.iter().map(|item| item.tokens).sum();
    if total_tokens == 0 {
        return Vec::new();
    }
    countries
        .iter()
        .enumerate()
        .map(|(index, item)| CountryEntry {
            country: item.country.clone(),
            continent: item.continent.clone(),
            tokens: round(item.tokens as f64 / 1_000_000_000_000.0, 4),
            share: round(item.tokens as f64 / total_tokens as f64 * 100.0, 1),
            rank: index as i64 + 1,
        })
        .collect()
}

/// `buildTokenCost`: cost-per-million-token entries, cheapest first.
pub fn build_token_cost(
    rows: &[StatMetricRow],
    product: TokenProduct,
    window: &DateWindow,
) -> Vec<TokenCostEntry> {
    let mut entries: Vec<TokenCostEntry> = top_models_by_usage(rows, product, window)
        .into_iter()
        .map(|item| TokenCostEntry {
            model: item.model.clone(),
            total: cost_per_million(item.total_cost_microcents, item.total_tokens),
            input: cost_per_million(item.input_cost_microcents, item.input_tokens),
            output: cost_per_million(
                item.output_cost_microcents,
                item.output_tokens + item.reasoning_tokens,
            ),
            cached: cost_per_million(
                item.input_cost_microcents,
                item.input_tokens + item.cache_read_tokens,
            ),
        })
        .collect();
    entries.sort_by(|a, b| {
        a.total
            .partial_cmp(&b.total)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    entries
}

/// `buildCacheRatio`: cache-hit ratios, highest first.
pub fn build_cache_ratio(
    rows: &[StatMetricRow],
    product: TokenProduct,
    window: &DateWindow,
) -> Vec<CacheRatioEntry> {
    let mut entries: Vec<CacheRatioEntry> = top_models_by_usage(rows, product, window)
        .into_iter()
        .filter_map(|item| {
            let total = item.input_tokens + item.cache_read_tokens;
            if total == 0 {
                return None;
            }
            Some(CacheRatioEntry {
                model: item.model.clone(),
                ratio: round(item.cache_read_tokens as f64 / total as f64 * 100.0, 1),
                cached: round(item.cache_read_tokens as f64 / 1_000_000_000.0, 1),
                uncached: round(item.input_tokens as f64 / 1_000_000_000.0, 1),
                total: round(total as f64 / 1_000_000_000.0, 1),
            })
        })
        .collect();
    entries.sort_by(|a, b| {
        b.ratio
            .partial_cmp(&a.ratio)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                b.cached
                    .partial_cmp(&a.cached)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });
    entries
}

/// `buildSessionCost`: dollars per session, cheapest first.
pub fn build_session_cost(
    rows: &[StatMetricRow],
    product: TokenProduct,
    window: &DateWindow,
) -> Vec<SessionCostEntry> {
    let mut entries: Vec<SessionCostEntry> = top_models_by_usage(rows, product, window)
        .into_iter()
        .filter_map(|item| {
            if item.sessions == 0 {
                return None;
            }
            let cost = round(
                microcents_to_dollars(item.total_cost_microcents as f64) / item.sessions as f64,
                4,
            );
            if cost == 0.0 {
                return None;
            }
            Some(SessionCostEntry {
                model: item.model.clone(),
                cost,
                tokens: (item.total_tokens as f64 / item.sessions as f64).round() as i64,
            })
        })
        .collect();
    entries.sort_by(|a, b| {
        a.cost
            .partial_cmp(&b.cost)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    entries
}

fn top_models_by_usage(
    rows: &[StatMetricRow],
    product: TokenProduct,
    window: &DateWindow,
) -> Vec<ModelAggregate> {
    let mut models = aggregate_by_model(&rows_for_product(rows, product, window.start, window.end));
    models.sort_by(|a, b| b.total_tokens.cmp(&a.total_tokens));
    models.truncate(METRIC_MODEL_LIMIT);
    models
}

/// `buildModelUsage`: per-bucket model usage points for the model detail page.
pub fn build_model_usage(
    rows: &[StatMetricRow],
    window: &DateWindow,
    range: UsageRange,
) -> Vec<ModelUsagePoint> {
    create_buckets(window, range)
        .into_iter()
        .map(|bucket| {
            let bucket_rows: Vec<StatMetricRow> = rows
                .iter()
                .filter(|row| row.period_start >= bucket.start && row.period_start < bucket.end)
                .cloned()
                .collect();
            let aggregate = combine_rows_for_model("", &bucket_rows);
            ModelUsagePoint {
                date: bucket.label,
                tokens: aggregate.total_tokens,
                users: aggregate.unique_users,
                sessions: aggregate.sessions,
                cost: round(
                    microcents_to_dollars(aggregate.total_cost_microcents as f64),
                    2,
                ),
            }
        })
        .collect()
}

/// `buildModelTokenMix`: input/output/reasoning/cached token mix.
pub fn build_model_token_mix(aggregate: &ModelAggregate) -> Vec<ModelMixEntry> {
    let items: Vec<(&'static str, i64)> = [
        ("Input", aggregate.input_tokens),
        ("Output", aggregate.output_tokens),
        ("Reasoning", aggregate.reasoning_tokens),
        ("Cached", aggregate.cache_read_tokens),
    ]
    .into_iter()
    .filter(|(_, tokens)| *tokens > 0)
    .collect();
    let total: i64 = items.iter().map(|(_, tokens)| tokens).sum();
    if total == 0 {
        return Vec::new();
    }
    items
        .into_iter()
        .map(|(label, tokens)| ModelMixEntry {
            label,
            tokens,
            share: round(tokens as f64 / total as f64 * 100.0, 1),
        })
        .collect()
}

/// `buildModelPeers`: ±5 peers around the model's rank (10 entries).
pub fn build_model_peers(
    peers: &[ModelAggregate],
    rank: i64,
    total_tokens: i64,
) -> Vec<ModelPeerEntry> {
    let start = 0_i64.max((rank - 5).min((peers.len() as i64 - 10).max(0)));
    peers
        .iter()
        .skip(start as usize)
        .take(10)
        .enumerate()
        .map(|(index, item)| ModelPeerEntry {
            model: item.model.clone(),
            provider: item.provider.clone(),
            author: format_provider(&item.provider),
            rank: start + index as i64 + 1,
            tokens: item.total_tokens,
            share: if total_tokens > 0 {
                round(item.total_tokens as f64 / total_tokens as f64 * 100.0, 2)
            } else {
                0.0
            },
            slug: model_slug(&item.model),
        })
        .collect()
}

/// `rowsForProduct`: window rows for one product cohort.
pub fn rows_for_product<'a>(
    rows: &'a [StatMetricRow],
    product: UsageProduct,
    start: i64,
    end: i64,
) -> Vec<&'a StatMetricRow> {
    let window_rows: Vec<&StatMetricRow> = rows
        .iter()
        .filter(|row| row.period_start >= start && row.period_start < end)
        .collect();
    if product == SITE_PRODUCT {
        return window_rows
            .into_iter()
            .filter(|row| row.tier == "Go" || row.tier == "Free")
            .collect();
    }
    if product != "All Users" {
        return window_rows
            .into_iter()
            .filter(|row| row.tier == product)
            .collect();
    }
    let all_rows: Vec<&StatMetricRow> = window_rows
        .iter()
        .copied()
        .filter(|row| row.tier == "all")
        .collect();
    if !all_rows.is_empty() {
        return all_rows;
    }
    window_rows
        .into_iter()
        .filter(|row| row.tier != "all")
        .collect()
}

/// `aggregateByModel`: sums rows keyed by `provider\0model` (insertion order).
pub fn aggregate_by_model(rows: &[&StatMetricRow]) -> Vec<ModelAggregate> {
    aggregate_by(rows, |row| model_key(&row.provider, &row.model))
}

/// `aggregateByModelName`: sums rows keyed by model name only.
pub fn aggregate_by_model_name(rows: &[&StatMetricRow]) -> Vec<ModelAggregate> {
    aggregate_by(rows, |row| row.model.clone())
}

fn aggregate_by(
    rows: &[&StatMetricRow],
    key: impl Fn(&StatMetricRow) -> String,
) -> Vec<ModelAggregate> {
    let mut result: Vec<(String, ModelAggregate)> = Vec::new();
    for item in rows {
        let row: &StatMetricRow = item;
        let key = key(row);
        if let Some(current) = result.iter_mut().find(|(k, _)| *k == key) {
            current.1 = combine_model_aggregate(Some(&current.1), row);
        } else {
            result.push((key, combine_model_aggregate(None, row)));
        }
    }
    result.into_iter().map(|(_, aggregate)| aggregate).collect()
}

/// `aggregateByProvider`: sums tokens keyed by provider name.
pub fn aggregate_by_provider(rows: &[&StatMetricRow]) -> Vec<ProviderAggregate> {
    let mut result: Vec<(String, i64)> = Vec::new();
    for item in rows {
        let row: &StatMetricRow = item;
        if let Some(current) = result
            .iter_mut()
            .find(|(provider, _)| *provider == row.provider)
        {
            current.1 += row.total_tokens;
        } else {
            result.push((row.provider.clone(), row.total_tokens));
        }
    }
    result
        .into_iter()
        .map(|(provider, tokens)| ProviderAggregate { provider, tokens })
        .collect()
}

/// `combineRowsForModel`: sums rows, forcing the model name.
pub fn combine_rows_for_model(model: &str, rows: &[StatMetricRow]) -> ModelAggregate {
    let mut aggregate: Option<ModelAggregate> = None;
    for row in rows {
        aggregate = Some(combine_model_aggregate(aggregate.as_ref(), row));
    }
    match aggregate {
        Some(mut value) => {
            if !model.is_empty() {
                value.model = model.to_string();
            }
            value
        }
        None => ModelAggregate {
            model: model.to_string(),
            provider: "unknown".to_string(),
            sessions: 0,
            unique_users: 0,
            input_tokens: 0,
            output_tokens: 0,
            reasoning_tokens: 0,
            cache_read_tokens: 0,
            total_tokens: 0,
            input_cost_microcents: 0,
            output_cost_microcents: 0,
            total_cost_microcents: 0,
        },
    }
}

fn combine_model_aggregate(
    current: Option<&ModelAggregate>,
    row: &StatMetricRow,
) -> ModelAggregate {
    ModelAggregate {
        model: row.model.clone(),
        provider: row.provider.clone(),
        sessions: current.map_or(0, |c| c.sessions) + row.sessions,
        unique_users: current.map_or(0, |c| c.unique_users) + row.unique_users,
        input_tokens: current.map_or(0, |c| c.input_tokens) + row.input_tokens,
        output_tokens: current.map_or(0, |c| c.output_tokens) + row.output_tokens,
        reasoning_tokens: current.map_or(0, |c| c.reasoning_tokens) + row.reasoning_tokens,
        cache_read_tokens: current.map_or(0, |c| c.cache_read_tokens) + row.cache_read_tokens,
        total_tokens: current.map_or(0, |c| c.total_tokens) + row.total_tokens,
        input_cost_microcents: current.map_or(0, |c| c.input_cost_microcents)
            + row.input_cost_microcents,
        output_cost_microcents: current.map_or(0, |c| c.output_cost_microcents)
            + row.output_cost_microcents,
        total_cost_microcents: current.map_or(0, |c| c.total_cost_microcents)
            + row.total_cost_microcents,
    }
}

/// `getWindow`: current + previous window for a usage range.
pub fn get_window(range: UsageRange, earliest: i64, latest: i64) -> DateWindow {
    let end = latest + DAY_MS;
    let start = (match range {
        "1D" => latest,
        "1W" => latest - 6 * DAY_MS,
        "2W" => latest - 13 * DAY_MS,
        "1M" => latest - 27 * DAY_MS,
        "2M" => latest - 55 * DAY_MS,
        "3M" => latest - 89 * DAY_MS,
        "YTD" => {
            let year = date_like::civil_from_days(latest.div_euclid(DAY_MS)).0;
            date_like::Date::utc(year, 1, 1).ms
        }
        _ => earliest,
    })
    .max(earliest);
    let duration = end - start;
    DateWindow {
        start,
        end,
        previous_start: start - duration,
        previous_end: start,
    }
}

/// `createBuckets`: equal-sized time buckets with range-dependent labels.
pub fn create_buckets(window: &DateWindow, range: UsageRange) -> Vec<Bucket> {
    let span = (window.end - window.start).max(DAY_MS);
    let count = match range {
        "1D" => 1,
        "1W" | "2W" | "1M" | "2M" | "3M" => ((span as f64 / DAY_MS as f64).ceil()) as i64,
        _ => ((span as f64 / DAY_MS as f64).ceil() as i64).clamp(1, 7),
    };
    let size = span as f64 / count as f64;
    (0..count)
        .map(|index| {
            let start = window.start + (index as f64 * size) as i64;
            let end = if index == count - 1 {
                window.end
            } else {
                window.start + ((index + 1) as f64 * size) as i64
            };
            Bucket {
                start,
                end,
                label: format_bucket_label(start, end, range),
            }
        })
        .collect()
}

fn format_bucket_label(start: i64, _end: i64, range: UsageRange) -> String {
    let date = date_like::Date::from_ms(start);
    if range == "YTD" {
        return MONTHS[(date.month - 1) as usize].to_string();
    }
    if range == "ALL" {
        let now_ms = now_ms();
        let now_year = date_like::civil_from_days(now_ms.div_euclid(DAY_MS)).0;
        return if date.year == now_year {
            MONTHS[(date.month - 1) as usize].to_string()
        } else {
            date.year.to_string()
        };
    }
    format_day(start)
}

fn format_day(value: i64) -> String {
    let date = date_like::Date::from_ms(value);
    format!("{} {}", MONTHS[(date.month - 1) as usize], date.day)
}

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

/// `formatProvider`: display names for known providers.
pub fn format_provider(provider: &str) -> String {
    let known: [(&str, &str); 16] = [
        ("anthropic", "Anthropic"),
        ("deepseek", "DeepSeek"),
        ("google", "Google"),
        ("minimax", "MiniMax"),
        ("meta", "Meta"),
        ("moonshot", "Moonshot"),
        ("moonshotai", "Moonshot"),
        ("nvidia", "NVIDIA"),
        ("opencode", "opencode"),
        ("openai", "OpenAI"),
        ("qwen", "Qwen"),
        ("tencent", "Tencent"),
        ("xai", "xAI"),
        ("xiaomi", "Xiaomi"),
        ("zhipu", "Zhipu"),
        ("zhipuai", "Zhipu"),
    ];
    let normalized: String = provider
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect();
    for (key, label) in known {
        if normalized == key {
            return label.to_string();
        }
    }
    provider
        .replace(['-', '_'], " ")
        .split(' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

/// `modelSlug`: lowercase kebab-case slug for a model name.
pub fn model_slug(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

fn model_key(provider: &str, model: &str) -> String {
    format!("{}\u{0}{}", provider, model)
}

/// `providerSlug`: aliased provider slug.
pub fn provider_slug(value: &str) -> String {
    let slug = model_slug(value);
    match slug.as_str() {
        "alibaba" => "qwen".to_string(),
        "moonshotai" => "moonshot".to_string(),
        "qwen" => "qwen".to_string(),
        "zhipuai" => "zhipu".to_string(),
        _ => slug,
    }
}

/// `providerMatches`: alias-aware provider comparison.
pub fn provider_matches(provider: &str, provider_param: &str) -> bool {
    provider_slug(provider) == provider_slug(provider_param)
}

/// `periodKey`: `YYYY-MM-DD` of an epoch-millisecond value (UTC).
pub fn period_key(value: i64) -> String {
    date_like::Date::from_ms(value).iso()
}

/// `periodKeyTime`: epoch ms (UTC midnight) of a `YYYY-MM-DD` key.
pub fn period_key_time(value: &str) -> Option<i64> {
    let year = value.get(0..4)?.parse::<i32>().ok()?;
    let month = value.get(5..7)?.parse::<u32>().ok()?;
    let day = value.get(8..10)?.parse::<u32>().ok()?;
    if value.len() < 10 {
        return None;
    }
    Some(date_like::Date::utc(year, month, day).ms)
}

/// `dateTime`: epoch ms of an ISO 8601 timestamp (UTC).
pub fn date_time(value: &str) -> Option<i64> {
    parse_iso_8601(value)
}

fn parse_iso_8601(value: &str) -> Option<i64> {
    // YYYY-MM-DDTHH:MM:SS(.mmm)?Z
    let year = value.get(0..4)?.parse::<i32>().ok()?;
    let month = value.get(5..7)?.parse::<u32>().ok()?;
    let day = value.get(8..10)?.parse::<u32>().ok()?;
    let rest = value.get(11..)?;
    let hour = rest.get(0..2)?.parse::<i64>().ok()?;
    let minute = rest.get(3..5)?.parse::<i64>().ok()?;
    let second = rest.get(6..8)?.parse::<i64>().ok()?;
    let millis = if rest.len() > 9 && rest.as_bytes()[8] == b'.' {
        rest.get(9..12)?.parse::<i64>().ok()?
    } else {
        0
    };
    if !rest.ends_with('Z') {
        return None;
    }
    Some(
        date_like::Date::utc(year, month, day).ms
            + hour * 3_600_000
            + minute * 60_000
            + second * 1000
            + millis,
    )
}

fn cost_per_million(cost_microcents: i64, tokens: i64) -> f64 {
    if tokens <= 0 || cost_microcents <= 0 {
        return 0.0;
    }
    round(
        microcents_to_dollars(cost_microcents as f64) / tokens as f64 * TOKEN_SCALE as f64,
        2,
    )
}

fn microcents_to_dollars(value: f64) -> f64 {
    value * DOLLARS_PER_MICROCENT
}

/// `percentChange`: rounded percentage change, 100 when gaining from zero.
pub fn percent_change(current: i64, previous: i64) -> i64 {
    if previous <= 0 {
        return if current > 0 { 100 } else { 0 };
    }
    ((current - previous) as f64 / previous as f64 * 100.0).round() as i64
}

/// `leaderboardChange`: null when previous is missing or current dominated it.
fn leaderboard_change(current: i64, previous: i64) -> Option<i64> {
    if current <= 0 {
        return Some(0);
    }
    if previous <= 0 || current >= previous * LEADERBOARD_CHANGE_MIN_MULTIPLE {
        return None;
    }
    Some(percent_change(current, previous))
}

/// PROVISIONAL descriptor of the home data services (Effect + Planetscale
/// runtime absent): `getStatsHomeData` loads daily model metrics, weekly
/// retention and country totals, then calls the pure builders above.
pub mod repo {
    use crate::domain::retention::is_missing_retention_table;

    pub const SERVICE_ID: &str = "@opencode/stats/home";
    pub const QUERY_CACHE_TTL_MS: i64 = 5 * 60 * 1000;
    pub const QUERY_CACHE_MAX_ENTRIES: usize = 256;
    /// `listModelDaily`: `model_stat` day rows for the zen dataset and site tiers.
    pub const MODEL_DAILY_QUERY: &str = "select period_key, updated_at, tier, provider, model, sessions, unique_users, input_tokens,
    output_tokens, reasoning_tokens, cache_read_tokens, total_tokens, input_cost_microcents, output_cost_microcents,
    total_cost_microcents from model_stat where grain = 'day' and dataset = 'zen' and client = 'all'
    and source = 'all' and tier in (?, ?, ?, ?) order by period_key";
    /// `listCountryTotals`: country token totals within a window (optionally per model).
    pub fn country_totals_query(scope: &str) -> String {
        format!(
            "select country, max(continent) as continent, sum(total_tokens) as total_tokens, max(updated_at) as updated_at
    from geo_stat where grain = 'day' and dataset = 'zen' and client = 'all' and source = 'all'
    and tier in (?, ?, ?, ?) {} and period_key >= ? and period_key < ? group by country",
            scope
        )
    }
    pub const COUNTRY_TOTALS_SCOPE_GLOBAL: &str = "and provider = 'all' and model = 'all'";
    pub const COUNTRY_TOTALS_SCOPE_MODEL: &str = "and model = ?";
    pub const COUNTRY_TOTALS_SCOPE_PROVIDER_MODEL: &str = "and provider = ? and model = ?";
    /// `listRetentionWeekly`: retention rows guarded by `isMissingRetentionTable`.
    pub const RETENTION_WEEKLY_QUERY: &str =
        "select cohort_date, updated_at, provider, model, eligible_users, retained_users
      from model_retention where dataset = 'zen' and tier = 'Go' order by cohort_date";
    pub const RETENTION_MISSING_COLUMN: &str = "Unknown column 'retained_users'";
    pub fn retention_available(cause: &str) -> bool {
        !is_missing_retention_table(cause)
    }
}
