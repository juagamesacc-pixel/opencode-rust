// source: packages/stats/core/src/domain/model.ts (1:1 port)

use super::stat::{
    collapse_rows, stat_period_key, synthesize_all_tier_rows, StatBaseAggregate, StatBaseRow,
    StatBaseRowLike, DATA_SITE_TIERS,
};
use crate::impl_from_stat_base;
use crate::impl_stat_base_row;

/// `ModelStatRow`: ingestion row for the `model_stat` table.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ModelStatRow {
    pub grain: String,
    pub period_key: String,
    pub dataset: Option<String>,
    pub tier: Option<String>,
    pub client: Option<String>,
    pub source: Option<String>,
    pub provider: String,
    pub model: String,
    pub provider_model: String,
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
    pub rank_by_tokens: Option<i64>,
    pub rank_by_requests: Option<i64>,
    pub rank_by_cost: Option<i64>,
}

impl ModelStatRow {
    pub fn dimension_key(&self) -> String {
        [self.provider.clone(), self.model.clone()].join("\u{0}")
    }
}

impl_stat_base_row!(ModelStatRow, ranks);
impl_from_stat_base!(ModelStatRow);

/// `ModelStatAggregate`: rolled-up aggregate row before ranking.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ModelStatAggregate {
    pub base: StatBaseAggregate,
    pub provider: String,
    pub model: String,
    pub provider_model: String,
}

/// `ModelStatMetric`: daily site-facing metric row.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ModelStatMetric {
    pub period_key: String,
    pub updated_at: String,
    pub tier: String,
    pub provider: String,
    pub model: String,
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

/// `rowsFromAggregates`: collapses per-grain aggregates, synthesizes the
/// `tier = "all"` rows and ranks within each period scope key.
pub fn rows_from_aggregates(aggregates: &[ModelStatAggregate]) -> Vec<ModelStatRow> {
    let mut rows: Vec<ModelStatRow> = Vec::new();
    for grain in ["week", "day"] {
        let grain_rows: Vec<ModelStatRow> = aggregates
            .iter()
            .filter(|item| item.base.grain == grain)
            .map(to_row)
            .collect();
        let collapsed = collapse_rows(&grain_rows);
        rows.extend(synthesize_all_tier_rows(&collapsed));
    }
    rank_all(&rows)
}

fn to_row(data: &ModelStatAggregate) -> ModelStatRow {
    let base = StatBaseRow::from_aggregate(&data.base);
    let mut out = ModelStatRow::from(&base);
    out.provider = data.provider.clone();
    out.model = data.model.clone();
    out.provider_model = data.provider_model.clone();
    out
}

/// `rankRows`: groups by period scope key and assigns token/request/cost ranks.
fn rank_all(rows: &[ModelStatRow]) -> Vec<ModelStatRow> {
    let mut out = rows.to_vec();
    let mut groups: Vec<(String, Vec<usize>)> = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let key = stat_period_key(row);
        if let Some(group) = groups.iter_mut().find(|(k, _)| *k == key) {
            group.1.push(index);
        } else {
            groups.push((key, vec![index]));
        }
    }
    for (_, indices) in groups {
        let token_ranks = super::stat::rank_by_group(&indices, |i| rows[*i].total_tokens());
        let request_ranks = super::stat::rank_by_group(&indices, |i| rows[*i].requests());
        let cost_ranks = super::stat::rank_by_group(&indices, |i| rows[*i].total_cost_microcents());
        for &i in &indices {
            out[i].rank_by_tokens = super::stat::rank_lookup(&token_ranks, i);
            out[i].rank_by_requests = super::stat::rank_lookup(&request_ranks, i);
            out[i].rank_by_cost = super::stat::rank_lookup(&cost_ranks, i);
        }
    }
    out
}

/// `modelDailyScope`: daily site-tier rows (used by the repo list query).
pub fn model_daily_scope() -> Vec<String> {
    DATA_SITE_TIERS
        .iter()
        .map(|tier| tier.to_string())
        .collect()
}

/// PROVISIONAL descriptor of `ModelStatRepo` (Effect/Planetscale runtime absent):
/// the service lists daily metrics, reports the last sync timestamp, upserts
/// rows in 500-row chunks (without `unique_users` when the table lacks the
/// column) and deletes rows for retired model/provider dimensions.
pub mod repo {
    pub const SERVICE_ID: &str = "@opencode/stats/ModelStatRepo";
    pub const UPSERT_CHUNK_SIZE: usize = super::super::stat::UPSERT_CHUNK_SIZE;
    #[allow(dead_code)]
    pub fn list_daily_scope() -> Vec<String> {
        super::model_daily_scope()
    }
}
