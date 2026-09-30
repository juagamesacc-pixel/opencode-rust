// source: packages/stats/core/src/domain/geo.ts (1:1 port)

use super::stat::{
    collapse_rows, rank_rows_with_market_share, stat_period_key, synthesize_all_tier_rows,
    StatBaseAggregate, StatBaseRow,
};
use crate::impl_from_stat_base;
use crate::impl_stat_base_row;

/// `GeoStatRow`: ingestion row for the `geo_stat` table.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct GeoStatRow {
    pub grain: String,
    pub period_key: String,
    pub dataset: Option<String>,
    pub tier: Option<String>,
    pub client: Option<String>,
    pub source: Option<String>,
    pub provider: String,
    pub model: String,
    pub country: String,
    pub continent: String,
    pub sessions: Option<i64>,
    pub requests: Option<i64>,
    pub unique_users: Option<i64>,
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub reasoning_tokens: Option<i64>,
    pub cache_read_tokens: Option<i64>,
    pub total_tokens: Option<i64>,
    pub input_cost_microcents: Option<i64>,
    pub output_cost_microcents: Option<i64>,
    pub total_cost_microcents: Option<i64>,
    pub avg_duration_ms: Option<f64>,
    pub p50_duration_ms: Option<i64>,
    pub p95_duration_ms: Option<i64>,
    pub avg_ttfb_ms: Option<f64>,
    pub p50_ttfb_ms: Option<i64>,
    pub p95_ttfb_ms: Option<i64>,
    pub avg_output_tps: Option<f64>,
    pub success_count: Option<i64>,
    pub error_count: Option<i64>,
    pub sample_count: Option<i64>,
    pub market_share_tokens: Option<f64>,
    pub market_share_requests: Option<f64>,
    pub market_share_sessions: Option<f64>,
    pub rank_by_tokens: Option<i64>,
    pub rank_by_requests: Option<i64>,
    pub rank_by_sessions: Option<i64>,
    pub rank_by_cost: Option<i64>,
}

impl GeoStatRow {
    /// `dimensionKey`: provider + model + country.
    pub fn dimension_key(&self) -> String {
        [
            self.provider.clone(),
            self.model.clone(),
            self.country.clone(),
        ]
        .join("\u{0}")
    }
}

impl_stat_base_row!(GeoStatRow, market);
impl_from_stat_base!(GeoStatRow);

/// `GeoStatAggregate`: rolled-up aggregate row before ranking.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoStatAggregate {
    pub base: StatBaseAggregate,
    pub provider: String,
    pub model: String,
    pub country: String,
    pub continent: String,
}

/// `GeoStatMetric`: daily site-facing metric row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoStatMetric {
    pub period_key: String,
    pub updated_at: String,
    pub tier: String,
    pub provider: String,
    pub model: String,
    pub country: String,
    pub continent: String,
    pub total_tokens: i64,
}

/// `rowsFromAggregates`: collapses per-grain aggregates, synthesizes the
/// `tier = "all"` rows and ranks with market share grouped by
/// `marketShareKey` (period scope + provider + model).
pub fn rows_from_aggregates(aggregates: &[GeoStatAggregate]) -> Vec<GeoStatRow> {
    let mut rows: Vec<GeoStatRow> = Vec::new();
    for grain in ["week", "day"] {
        let grain_rows: Vec<GeoStatRow> = aggregates
            .iter()
            .filter(|item| item.base.grain == grain)
            .map(to_row)
            .collect();
        let collapsed = collapse_rows(&grain_rows);
        rows.extend(synthesize_all_tier_rows(&collapsed));
    }
    rank_rows_with_market_share(&rows, market_share_key)
}

fn to_row(data: &GeoStatAggregate) -> GeoStatRow {
    let base = StatBaseRow::from_aggregate(&data.base);
    let mut out = GeoStatRow::from(&base);
    out.provider = data.provider.clone();
    out.model = data.model.clone();
    out.country = data.country.clone();
    out.continent = data.continent.clone();
    out
}

/// `marketShareKey`: period scope + provider + model.
fn market_share_key(row: &GeoStatRow) -> String {
    [
        stat_period_key(row),
        row.provider.clone(),
        row.model.clone(),
    ]
    .join("\u{0}")
}

/// PROVISIONAL descriptor of `GeoStatRepo` (Effect/Planetscale runtime absent):
/// lists daily site-tier metrics, lists rows for one period scope, upserts in
/// 500-row chunks (falling back to rows without `unique_users`) and deletes
/// rows for retired model/provider dimensions.
pub mod repo {
    pub const SERVICE_ID: &str = "@opencode/stats/GeoStatRepo";
}
