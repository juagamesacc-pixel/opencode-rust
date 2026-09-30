// source: packages/stats/core/src/domain/provider.ts (1:1 port)

use super::stat::{
    collapse_rows, rank_rows_with_market_share, synthesize_all_tier_rows, StatBaseAggregate,
    StatBaseRow,
};
use crate::impl_from_stat_base;
use crate::impl_stat_base_row;

/// `ProviderStatRow`: ingestion row for the `provider_stat` table.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ProviderStatRow {
    pub grain: String,
    pub period_key: String,
    pub dataset: Option<String>,
    pub tier: Option<String>,
    pub client: Option<String>,
    pub source: Option<String>,
    pub provider: String,
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

impl ProviderStatRow {
    pub fn dimension_key(&self) -> String {
        self.provider.clone()
    }
}

impl_stat_base_row!(ProviderStatRow, market);
impl_from_stat_base!(ProviderStatRow);

/// `ProviderStatAggregate`: rolled-up aggregate row before ranking.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderStatAggregate {
    pub base: StatBaseAggregate,
    pub provider: String,
}

/// `ProviderStatMetric`: daily site-facing metric row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProviderStatMetric {
    pub period_key: String,
    pub updated_at: String,
    pub tier: String,
    pub provider: String,
    pub total_tokens: i64,
}

/// `rowsFromAggregates`: collapses per-grain aggregates, synthesizes the
/// `tier = "all"` rows and ranks with market share per period scope key.
pub fn rows_from_aggregates(aggregates: &[ProviderStatAggregate]) -> Vec<ProviderStatRow> {
    let mut rows: Vec<ProviderStatRow> = Vec::new();
    for grain in ["week", "day"] {
        let grain_rows: Vec<ProviderStatRow> = aggregates
            .iter()
            .filter(|item| item.base.grain == grain)
            .map(|item| {
                let base = StatBaseRow::from_aggregate(&item.base);
                let mut out = ProviderStatRow::from(&base);
                out.provider = item.provider.clone();
                out
            })
            .collect();
        let collapsed = collapse_rows(&grain_rows);
        rows.extend(synthesize_all_tier_rows(&collapsed));
    }
    rank_rows_with_market_share(&rows, super::stat::stat_period_key)
}

/// PROVISIONAL descriptor of `ProviderStatRepo` (Effect/Planetscale runtime absent):
/// lists daily site-tier metrics, lists rows for one period scope, upserts in
/// 500-row chunks and deletes rows for retired providers.
pub mod repo {
    pub const SERVICE_ID: &str = "@opencode/stats/ProviderStatRepo";
}
