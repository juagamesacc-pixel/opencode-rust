// source: packages/stats/core/src/domain/inference.ts (1:1 port)

use crate::domain::geo::GeoStatAggregate;
use crate::domain::model::ModelStatAggregate;
use crate::domain::model_normalization::{
    stat_model, stat_provider, EXCLUDED_MODELS, FREE_MODELS, MODEL_AUTHOR_RULES,
    MODEL_NAME_ALIASES, RETIRED_STAT_PROVIDERS, STEALTH_MODELS,
};
use crate::domain::provider::ProviderStatAggregate;
use crate::domain::retention::RetentionStatAggregate;
use crate::domain::stat::{
    date_like, normalize_country, normalize_tier, period_key_for, start_of_iso_week,
    start_of_utc_day, StatBaseAggregate, StatGrain,
};
use std::collections::HashMap;

/// `StatDimension`: dimension of a statistic row.
pub type StatDimension = &'static str;

/// `StatsQuerySource`: R2 SQL source table + dataset override.
#[derive(Debug, Clone)]
pub struct StatsQuerySource {
    pub namespace: String,
    pub table: String,
    pub dataset: String,
}

/// `RetentionQuery`: one retention SQL query plus the cohort dates it covers.
#[derive(Debug, Clone, PartialEq)]
pub struct RetentionQuery {
    pub cohort_dates: Vec<String>,
    pub query: String,
}

const DAY_MS: i64 = 86_400_000;
const WEEK_MS: i64 = 7 * DAY_MS;
/// The typed production stream began before the legacy backfill's original end
/// boundary. Use one exclusive handoff so the overlapping rows are never
/// counted from both sources.
pub const LIVE_SOURCE_START: &str = "2026-08-11T10:57:48.186Z";

/// PROVISIONAL: production defaults come from SST `Resource.R2Sql` /
/// `Resource.StatsSyncConfig` at runtime; the values are not available in the
/// Rust port, so callers must supply a [`StatsQuerySource`] explicitly.
pub const PROVISIONAL_DEFAULT_NAMESPACE: &str = "opencode-stats";
pub const PROVISIONAL_DEFAULT_TABLE: &str = "generation_events";
pub const PROVISIONAL_DEFAULT_DATA_SET: &str = "zen";

/// `buildStatsQueries`: two queries per period (usage + geo) keep each R2 SQL
/// result under the 10,000-row limit and avoid combining the distinct
/// user/session aggregates with the high-cardinality geo dimensions.
pub fn build_stats_queries(
    period_start_ms: i64,
    period_end_ms: i64,
    input: Option<&StatsQuerySource>,
) -> Vec<String> {
    let source = input
        .map(|value| value.clone())
        .unwrap_or_else(|| StatsQuerySource {
            namespace: PROVISIONAL_DEFAULT_NAMESPACE.to_string(),
            table: PROVISIONAL_DEFAULT_TABLE.to_string(),
            dataset: PROVISIONAL_DEFAULT_DATA_SET.to_string(),
        });
    let mut periods = stat_periods(StatGrain::Week, period_start_ms, period_end_ms);
    periods.extend(stat_periods(StatGrain::Day, period_start_ms, period_end_ms));
    let mut queries = Vec::new();
    for period in &periods {
        queries.push(build_stats_query(period, &source, "usage"));
        queries.push(build_stats_query(period, &source, "geo"));
    }
    queries
}

/// `buildRetentionQueries`: zero or one retention query for the range.
pub fn build_retention_queries(
    period_start_ms: i64,
    period_end_ms: i64,
    input: Option<&StatsQuerySource>,
) -> Vec<RetentionQuery> {
    let source = input
        .map(|value| value.clone())
        .unwrap_or_else(|| StatsQuerySource {
            namespace: PROVISIONAL_DEFAULT_NAMESPACE.to_string(),
            table: PROVISIONAL_DEFAULT_TABLE.to_string(),
            dataset: PROVISIONAL_DEFAULT_DATA_SET.to_string(),
        });
    let periods = retention_periods(period_start_ms, period_end_ms);
    if periods.is_empty() {
        return Vec::new();
    }
    vec![RetentionQuery {
        cohort_dates: periods
            .iter()
            .map(|period| {
                iso_8601(period.start_ms)
                    .get(0..10)
                    .unwrap_or("")
                    .to_string()
            })
            .collect(),
        query: build_retention_query(&periods, &source),
    }]
}

#[derive(Debug, Clone)]
pub struct RetentionPeriod {
    pub start_ms: i64,
    pub end_ms: i64,
    pub return_start_ms: i64,
    pub return_end_ms: i64,
}

fn build_retention_query(periods: &[RetentionPeriod], source: &StatsQuerySource) -> String {
    let first = &periods[0];
    let last = periods.last().expect("non-empty periods");
    let scan_start_value = sql_string(&iso_8601(first.start_ms));
    let scan_end_value = sql_string(&iso_8601(last.return_end_ms));
    let ingest_end_value = sql_string(&iso_8601(last.return_end_ms + DAY_MS));
    let source_table = [source.namespace.clone(), source.table.clone()]
        .iter()
        .map(|part| sql_identifier(part))
        .collect::<Vec<String>>()
        .join(".");

    let mut activity_dates: Vec<i64> = Vec::new();
    for period in periods {
        activity_dates.push(period.start_ms);
        activity_dates.push(period.return_start_ms);
    }
    activity_dates.sort_unstable();
    activity_dates.dedup();
    let activity_week_sql = format!(
        "CASE\n{}      ELSE null\n    END",
        activity_dates
            .iter()
            .map(|date| format!(
                "      WHEN started_at >= {} AND started_at < {} THEN {}",
                sql_string(&iso_8601(*date)),
                sql_string(&iso_8601(*date + WEEK_MS)),
                sql_string(&iso_8601(*date).get(0..10).unwrap_or("").to_string())
            ))
            .collect::<Vec<String>>()
            .join("\n")
    );

    let cohort_dates = periods
        .iter()
        .map(|period| {
            sql_string(
                &iso_8601(period.start_ms)
                    .get(0..10)
                    .unwrap_or("")
                    .to_string(),
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    let return_dates = periods
        .iter()
        .map(|period| {
            sql_string(
                &iso_8601(period.return_start_ms)
                    .get(0..10)
                    .unwrap_or("")
                    .to_string(),
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    let return_cohort_sql = format!(
        "CASE activity_week\n{}\n    END",
        periods
            .iter()
            .map(|period| format!(
                "      WHEN {} THEN {}",
                sql_string(
                    &iso_8601(period.return_start_ms)
                        .get(0..10)
                        .unwrap_or("")
                        .to_string()
                ),
                sql_string(
                    &iso_8601(period.start_ms)
                        .get(0..10)
                        .unwrap_or("")
                        .to_string()
                )
            ))
            .collect::<Vec<String>>()
            .join("\n")
    );

    format!(
        "\nWITH normalized AS (
  SELECT
    {activity_week_sql} AS activity_week,
    {stat_model_sql} AS model,
    COALESCE(NULLIF(route_model, ''), '') AS provider_model,
    COALESCE(NULLIF(provider_id, ''), '') AS raw_provider,
    COALESCE(NULLIF(user_id, ''), NULLIF(workspace_id, ''), NULLIF(service_api_key_id, '')) AS user_key
  FROM {source_table}
  WHERE event_type = 'generation.completed'
    AND source IN ('inference', 'inference-legacy')
    AND (
      (source = 'inference-legacy' AND started_at < {live_source})
      OR (source = 'inference' AND started_at >= {live_source})
    )
    AND product = 'go'
    AND model_requested IS NOT NULL
    AND model_requested <> ''
    AND __ingest_ts >= {scan_start_value}
    AND __ingest_ts < {ingest_end_value}
    AND started_at >= {scan_start_value}
    AND started_at < {scan_end_value}
), filtered AS (
  SELECT
    activity_week,
    {stat_provider_sql} AS provider,
    model,
    user_key
  FROM normalized
  WHERE activity_week IS NOT NULL
    AND user_key <> ''
    AND lower(model) NOT IN ({excluded})
), model_usage AS (
  SELECT
    activity_week AS cohort_date,
    user_key,
    provider,
    model,
    COUNT(*) AS model_requests
  FROM filtered
  WHERE activity_week IN ({cohort_dates})
  GROUP BY activity_week, user_key, provider, model
), user_totals AS (
  SELECT
    cohort_date,
    user_key,
    SUM(model_requests) AS total_requests,
    MAX(model_requests) AS max_model_requests
  FROM model_usage
  GROUP BY cohort_date, user_key
), primary_models AS (
  SELECT model_usage.cohort_date, model_usage.user_key, model_usage.provider, model_usage.model
  FROM model_usage
  INNER JOIN user_totals ON model_usage.cohort_date = user_totals.cohort_date
    AND model_usage.user_key = user_totals.user_key
    AND model_usage.model_requests = user_totals.max_model_requests
  WHERE user_totals.total_requests >= 10
    AND CAST(model_usage.model_requests AS double) / NULLIF(user_totals.total_requests, 0) >= 0.8
), returned AS (
  SELECT
    {return_cohort_sql} AS cohort_date,
    user_key
  FROM filtered
  WHERE activity_week IN ({return_dates})
  GROUP BY {return_cohort_sql}, user_key
)
SELECT
  primary_models.cohort_date,
  {dataset} AS dataset,
  'Go' AS tier,
  primary_models.provider,
  primary_models.model,
  COUNT(*) AS eligible_users,
  SUM(CASE WHEN returned.user_key IS NULL THEN 0 ELSE 1 END) AS retained_users
FROM primary_models
LEFT JOIN returned ON primary_models.user_key = returned.user_key
  AND primary_models.cohort_date = returned.cohort_date
GROUP BY primary_models.cohort_date, primary_models.provider, primary_models.model
LIMIT 10000
",
        activity_week_sql = activity_week_sql,
        stat_model_sql = stat_model_sql("model_requested", "route_model"),
        source_table = source_table,
        live_source = sql_string(LIVE_SOURCE_START),
        scan_start_value = scan_start_value,
        ingest_end_value = ingest_end_value,
        scan_end_value = scan_end_value,
        stat_provider_sql = stat_provider_sql("model", "provider_model", "raw_provider"),
        excluded = excluded_models_sql(),
        cohort_dates = cohort_dates,
        return_cohort_sql = return_cohort_sql,
        return_dates = return_dates,
        dataset = sql_string(&source.dataset),
    )
}

#[derive(Debug, Clone)]
pub struct StatPeriod {
    pub grain: &'static str,
    pub key: String,
    pub start_ms: i64,
    pub end_ms: i64,
}

/// `StatPeriod` may carry `start_ms == end_ms` for exact handoff boundaries.
impl StatPeriod {
    pub fn is_empty(&self) -> bool {
        self.start_ms == self.end_ms
    }
}

fn build_stats_query(period: &StatPeriod, source: &StatsQuerySource, family: &str) -> String {
    let period_start_value = sql_string(&iso_8601(period.start_ms));
    let period_end_value = sql_string(&iso_8601(period.end_ms));
    let ingest_end_value = sql_string(&iso_8601(period.end_ms + DAY_MS));
    let source_table = [source.namespace.clone(), source.table.clone()]
        .iter()
        .map(|part| sql_identifier(part))
        .collect::<Vec<String>>()
        .join(".");
    let source_free_tier = free_tier_sql("model_tier", "model_requested");
    let (dimensions, grouping_sets) = if family == "usage" {
        (
            "CASE WHEN grouping(model) = 0 THEN 'model' ELSE 'provider' END AS dimension,
  tier,
  provider,
  CASE WHEN grouping(model) = 0 THEN model END AS model,
  CASE WHEN grouping(model) = 0 THEN COALESCE(MAX(NULLIF(provider_model, '')), '') END AS provider_model,
  null AS country,
  null AS continent".to_string(),
            "(tier, provider, model),
  (tier, provider)".to_string(),
        )
    } else {
        (
            "CASE WHEN grouping(model) = 0 THEN 'geo_model' ELSE 'geo' END AS dimension,
  tier,
  CASE WHEN grouping(model) = 0 THEN provider ELSE 'all' END AS provider,
  CASE WHEN grouping(model) = 0 THEN model ELSE 'all' END AS model,
  null AS provider_model,
  country,
  COALESCE(MAX(NULLIF(continent, '')), '') AS continent"
                .to_string(),
            "(tier, country),
  (tier, provider, model, country)"
                .to_string(),
        )
    };
    let distinct_columns = if family == "usage" {
        "approx_distinct(session) AS sessions,
    approx_distinct(user_key) AS unique_users"
            .to_string()
    } else {
        "0 AS sessions,
    0 AS unique_users"
            .to_string()
    };
    let aggregate_columns = format!(
        "
    {distinct_columns},
    COUNT(*) AS requests,
    COALESCE(SUM(tokens_input), 0) AS input_tokens,
    COALESCE(SUM(tokens_output), 0) AS output_tokens,
    COALESCE(SUM(tokens_reasoning), 0) AS reasoning_tokens,
    COALESCE(SUM(tokens_cache_read), 0) AS cache_read_tokens,
    COALESCE(SUM(tokens_total), 0) AS total_tokens,
    COALESCE(SUM(cost_input_microcents), 0) AS input_cost_microcents,
    COALESCE(SUM(cost_output_microcents), 0) AS output_cost_microcents,
    COALESCE(SUM(cost_total_microcents), 0) AS total_cost_microcents,
    AVG(duration_ms) AS avg_duration_ms,
    null AS p50_duration_ms,
    null AS p95_duration_ms,
    AVG(ttfb_ms) AS avg_ttfb_ms,
    null AS p50_ttfb_ms,
    null AS p95_ttfb_ms,
    AVG(output_tps) AS avg_output_tps,
    SUM(CASE WHEN outcome = 'succeeded' THEN 1 ELSE 0 END) AS success_count,
    SUM(CASE WHEN outcome = 'failed' THEN 1 ELSE 0 END) AS error_count,
    COUNT(*) AS sample_count"
    );

    format!(
        "\nWITH normalized AS (
  SELECT
    model_requested AS raw_model,
    COALESCE(NULLIF(lower(model_tier), ''), '') AS raw_tier,
    {stat_model_sql} AS model,
    COALESCE(NULLIF(route_model, ''), '') AS provider_model,
    COALESCE(NULLIF(provider_id, ''), '') AS raw_provider,
    UPPER(COALESCE(NULLIF(country, ''), 'ZZ')) AS country,
    COALESCE(NULLIF(continent, ''), '') AS continent,
    session_id AS session,
    COALESCE(NULLIF(workspace_id, ''), '') AS workspace,
    COALESCE(NULLIF(service_api_key_id, ''), '') AS api_key,
    COALESCE(NULLIF(user_id, ''), '') AS user_id,
    outcome,
    duration_ms,
    time_to_first_token_ms AS ttfb_ms,
    CASE
      WHEN first_token_at IS NULL OR last_token_at IS NULL THEN null
      ELSE date_part('epoch', last_token_at) - date_part('epoch', first_token_at)
    END AS output_seconds,
    tokens_input,
    tokens_output,
    tokens_reasoning,
    tokens_cache_read,
    tokens_cache_write,
    cost_input AS cost_input_microcents,
    cost_output AS cost_output_microcents,
    cost_total AS cost_total_microcents
  FROM {source_table}
  WHERE event_type = 'generation.completed'
    AND source IN ('inference', 'inference-legacy')
    AND (
      (source = 'inference-legacy' AND started_at < {live_source})
      OR (source = 'inference' AND started_at >= {live_source})
    )
    AND (product = 'go' OR ({source_free_tier}))
    AND model_requested IS NOT NULL
    AND model_requested <> ''
    AND __ingest_ts >= {period_start_value}
    AND __ingest_ts < {ingest_end_value}
    AND started_at >= {period_start_value}
    AND started_at < {period_end_value}
), filtered AS (
  SELECT
    CASE
      WHEN {free_tier_sql}
      THEN 'Free'
      ELSE 'Go'
    END AS tier,
    {stat_provider_sql} AS provider,
    provider_model,
    model,
    country,
    continent,
    session,
    COALESCE(NULLIF(user_id, ''), NULLIF(workspace, ''), NULLIF(api_key, '')) AS user_key,
    outcome,
    duration_ms,
    ttfb_ms,
    CASE
      WHEN output_seconds < 0.1 THEN null
      ELSE CAST(tokens_output AS double) / output_seconds
    END AS output_tps,
    tokens_input,
    tokens_output,
    tokens_reasoning,
    tokens_cache_read,
    COALESCE(tokens_cache_read, 0) + COALESCE(tokens_cache_write, 0) + COALESCE(tokens_input, 0) + COALESCE(tokens_output, 0) AS tokens_total,
    cost_input_microcents,
    cost_output_microcents,
    cost_total_microcents
  FROM normalized
  WHERE lower(model) NOT IN ({excluded})
)
SELECT
  {grain} AS grain,
  {period_key} AS period_key,
  {dataset} AS dataset,
  {dimensions},
  {aggregate_columns}
FROM filtered
GROUP BY GROUPING SETS (
  {grouping_sets}
)
LIMIT 10000
",
        stat_model_sql = stat_model_sql("model_requested", "route_model"),
        source_table = source_table,
        live_source = sql_string(LIVE_SOURCE_START),
        source_free_tier = source_free_tier,
        period_start_value = period_start_value,
        ingest_end_value = ingest_end_value,
        period_end_value = period_end_value,
        free_tier_sql = free_tier_sql("raw_tier", "raw_model"),
        stat_provider_sql = stat_provider_sql("model", "provider_model", "raw_provider"),
        excluded = excluded_models_sql(),
        grain = sql_string(period.grain),
        period_key = sql_string(&period.key),
        dataset = sql_string(&source.dataset),
        dimensions = dimensions,
        aggregate_columns = aggregate_columns,
        grouping_sets = grouping_sets,
    )
}

fn excluded_models_sql() -> String {
    EXCLUDED_MODELS
        .iter()
        .map(|model| sql_string(model))
        .collect::<Vec<String>>()
        .join(", ")
}

/// Value of one column in a raw R2 SQL result row (`Record<string, unknown>`).
#[derive(Debug, Clone)]
pub enum R2Value {
    Number(f64),
    Text(String),
    Null,
}

/// `R2SqlData`: one raw row returned by the R2 SQL ingestion query.
#[derive(Debug, Clone, Default)]
pub struct R2SqlData {
    cols: HashMap<String, R2Value>,
}

impl R2SqlData {
    pub fn new() -> Self {
        R2SqlData {
            cols: HashMap::new(),
        }
    }

    pub fn with_text(mut self, key: &str, value: &str) -> Self {
        self.cols
            .insert(key.to_string(), R2Value::Text(value.to_string()));
        self
    }

    pub fn with_number(mut self, key: &str, value: f64) -> Self {
        self.cols.insert(key.to_string(), R2Value::Number(value));
        self
    }

    /// Raw column value, `None` when the column is absent.
    pub fn col(&self, key: &str) -> Option<&R2Value> {
        self.cols.get(key)
    }

    /// TS `data[key] ?? ""`-style text access.
    pub fn text(&self, key: &str) -> String {
        match self.cols.get(key) {
            Some(R2Value::Text(value)) => value.clone(),
            Some(R2Value::Number(value)) => value.to_string(),
            _ => String::new(),
        }
    }

    /// `number`: `Number(data[key])`, non-finite coerced to 0.
    fn number(&self, key: &str) -> f64 {
        match self.cols.get(key) {
            Some(R2Value::Number(value)) => *value,
            Some(R2Value::Text(value)) => value.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
        }
    }

    /// `integer`: `Math.round(number(...))`.
    fn integer(&self, key: &str) -> i64 {
        self.number(key).round() as i64
    }

    /// `nullableNumber`: null when absent/empty, else the value rounded to 2.
    fn nullable_number(&self, key: &str) -> Option<f64> {
        match self.cols.get(key) {
            Some(R2Value::Text(value)) if value.is_empty() => None,
            None => None,
            _ => Some(round_2(self.number(key))),
        }
    }

    /// `nullableInteger`: null when absent/empty, else `Math.round(number(...))`.
    fn nullable_integer(&self, key: &str) -> Option<i64> {
        match self.cols.get(key) {
            Some(R2Value::Text(value)) if value.is_empty() => None,
            None => None,
            _ => Some(self.number(key).round() as i64),
        }
    }
}

/// `toModelAggregate`: canonical model rows from raw R2 rows.
pub fn to_model_aggregate(data: &R2SqlData) -> Vec<ModelStatAggregate> {
    let model = stat_model(
        Some(&data.text("model")),
        Some(&data.text("provider_model")),
    );
    let provider = stat_provider(
        Some(&model),
        Some(&data.text("provider_model")),
        Some(&data.text("provider")),
    );
    let provider = match provider {
        Some(provider) => provider,
        None => return Vec::new(),
    };
    to_stat_base_aggregate(data)
        .into_iter()
        .flat_map(|base| {
            vec![ModelStatAggregate {
                base,
                provider: provider.clone(),
                model: model.clone(),
                provider_model: data.text("provider_model"),
            }]
        })
        .collect()
}

/// `toProviderAggregate`: canonical provider rows from raw R2 rows.
pub fn to_provider_aggregate(data: &R2SqlData) -> Vec<ProviderStatAggregate> {
    let provider = stat_provider(
        Some(&data.text("model")),
        Some(&data.text("provider_model")),
        Some(&data.text("provider")),
    )
    .unwrap_or_else(|| "unknown".to_string());
    to_stat_base_aggregate(data)
        .into_iter()
        .map(|base| ProviderStatAggregate {
            base,
            provider: provider.clone(),
        })
        .collect()
}

/// `toGeoAggregate`: canonical geo rows from raw R2 rows.
pub fn to_geo_aggregate(data: &R2SqlData) -> Vec<GeoStatAggregate> {
    let provider = stat_provider(
        Some(&data.text("model")),
        Some(&data.text("provider_model")),
        Some(&data.text("provider")),
    )
    .unwrap_or_else(|| "all".to_string());
    let raw_model = data.text("model");
    let model = stat_model(
        Some(if raw_model.is_empty() {
            "all"
        } else {
            &raw_model
        }),
        Some(&data.text("provider_model")),
    );
    to_stat_base_aggregate(data)
        .into_iter()
        .map(|base| GeoStatAggregate {
            base,
            provider: provider.clone(),
            model: model.clone(),
            country: normalize_country(Some(&data.text("country"))),
            continent: data.text("continent"),
        })
        .collect()
}

/// `toRetentionAggregate`: retention cohort rows from raw R2 rows.
pub fn to_retention_aggregate(data: &R2SqlData) -> Vec<RetentionStatAggregate> {
    if !data.text("cohort_date").is_empty() && !data.text("model").is_empty() {
        let provider = stat_provider(
            Some(&data.text("model")),
            Some(""),
            Some(&data.text("provider")),
        )
        .unwrap_or_else(|| "unknown".to_string());
        let dataset = data.text("dataset");
        let dataset = if dataset.is_empty() {
            PROVISIONAL_DEFAULT_DATA_SET.to_string()
        } else {
            dataset
        };
        let tier = data.text("tier");
        let tier = if tier.is_empty() {
            "all".to_string()
        } else {
            tier
        };
        return vec![RetentionStatAggregate {
            cohort_date: data.text("cohort_date"),
            dataset,
            tier,
            provider,
            model: stat_model(Some(&data.text("model")), None),
            eligible_users: data.integer("eligible_users"),
            retained_users: data.integer("retained_users"),
        }];
    }
    Vec::new()
}

fn to_stat_base_aggregate(data: &R2SqlData) -> Vec<StatBaseAggregate> {
    let grain = data.text("grain");
    if grain != "day" && grain != "week" {
        return Vec::new();
    }
    if data.text("period_key").is_empty() {
        return Vec::new();
    }
    let dataset = data.text("dataset");
    let dataset = if dataset.is_empty() {
        PROVISIONAL_DEFAULT_DATA_SET.to_string()
    } else {
        dataset
    };
    let tier = data.text("tier");
    let tier = if tier.is_empty() {
        "unknown".to_string()
    } else {
        tier
    };
    vec![StatBaseAggregate {
        grain,
        period_key: data.text("period_key"),
        dataset,
        tier: normalize_tier(&tier),
        sessions: data.integer("sessions"),
        requests: data.integer("requests"),
        unique_users: data.integer("unique_users"),
        input_tokens: data.integer("input_tokens"),
        output_tokens: data.integer("output_tokens"),
        reasoning_tokens: data.integer("reasoning_tokens"),
        cache_read_tokens: data.integer("cache_read_tokens"),
        total_tokens: data.integer("total_tokens"),
        input_cost_microcents: data.integer("input_cost_microcents"),
        output_cost_microcents: data.integer("output_cost_microcents"),
        total_cost_microcents: data.integer("total_cost_microcents"),
        avg_duration_ms: data.nullable_number("avg_duration_ms"),
        p50_duration_ms: data.nullable_integer("p50_duration_ms"),
        p95_duration_ms: data.nullable_integer("p95_duration_ms"),
        avg_ttfb_ms: data.nullable_number("avg_ttfb_ms"),
        p50_ttfb_ms: data.nullable_integer("p50_ttfb_ms"),
        p95_ttfb_ms: data.nullable_integer("p95_ttfb_ms"),
        avg_output_tps: data.nullable_number("avg_output_tps"),
        success_count: data.integer("success_count"),
        error_count: data.integer("error_count"),
        sample_count: data.integer("sample_count"),
    }]
}

fn round_2(value: f64) -> f64 {
    let factor = 100.0;
    (value * factor).round() / factor
}

/// `sqlIdentifier`: double-quote, doubling embedded quotes.
pub fn sql_identifier(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

/// `sqlString`: single-quote, doubling embedded quotes.
pub fn sql_string(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

/// `statPeriods`: start-of-period buckets covering [start, end).
pub fn stat_periods(grain: StatGrain, period_start_ms: i64, period_end_ms: i64) -> Vec<StatPeriod> {
    let interval = if grain == StatGrain::Week {
        WEEK_MS
    } else {
        DAY_MS
    };
    let first = if grain == StatGrain::Week {
        start_of_iso_week(&date_like::Date::from_ms(period_start_ms))
    } else {
        start_of_utc_day(&date_like::Date::from_ms(period_start_ms))
    };
    let count = ceil_div(period_end_ms - first.ms, interval);
    (0..count)
        .map(|index| {
            let start_ms = first.ms + index * interval;
            StatPeriod {
                grain: grain.as_str(),
                key: period_key_for(grain, &date_like::Date::from_ms(start_ms)),
                start_ms,
                end_ms: (start_ms + interval).min(period_end_ms),
            }
        })
        .collect()
}

fn ceil_div(numerator: i64, denominator: i64) -> i64 {
    if numerator <= 0 {
        0
    } else {
        (numerator + denominator - 1) / denominator
    }
}

/// `retentionPeriods`: fully-observed weeks whose following week is also
/// inside the range (the return window must be complete).
fn retention_periods(period_start_ms: i64, period_end_ms: i64) -> Vec<RetentionPeriod> {
    let first = start_of_iso_week(&date_like::Date::from_ms(period_start_ms));
    let complete_end = start_of_iso_week(&date_like::Date::from_ms(period_end_ms));
    let count = ((complete_end.ms - first.ms) / WEEK_MS) - 1;
    let count = count.max(0);
    (0..count)
        .map(|index| {
            let start_ms = first.ms + index * WEEK_MS;
            let end_ms = start_ms + WEEK_MS;
            RetentionPeriod {
                start_ms,
                end_ms,
                return_start_ms: end_ms,
                return_end_ms: end_ms + WEEK_MS,
            }
        })
        .collect()
}

fn stat_model_sql(model: &str, provider_model: &str) -> String {
    let normalized = format!(
        "regexp_replace(CASE
      WHEN lower({model}) = 'big-pickle' THEN regexp_replace(NULLIF({provider_model}, ''), '^.*/', '')
      ELSE {model}
    END, '(-free|:free|:global)+$', '')"
    );
    let mut when_lines = String::new();
    for (from, to) in MODEL_NAME_ALIASES {
        when_lines.push_str(&format!(
            "      WHEN lower({normalized}) = {from} THEN {to}\n",
            from = sql_string(from),
            to = sql_string(to)
        ));
    }
    format!(
        "COALESCE(NULLIF(CASE
{when_lines}      ELSE {normalized}
    END, ''), 'unknown')"
    )
}

fn free_tier_sql(tier: &str, model: &str) -> String {
    let free_models = FREE_MODELS
        .iter()
        .map(|name| sql_string(name))
        .collect::<Vec<String>>()
        .join(", ");
    format!(
        "lower(COALESCE({tier}, '')) = 'free'
        OR lower({model}) IN ({free_models})
        OR lower({model}) LIKE '%-free'
        OR lower({model}) LIKE '%-free:global'"
    )
}

fn stat_provider_sql(model: &str, provider_model: &str, provider: &str) -> String {
    let stealth = STEALTH_MODELS
        .iter()
        .map(|name| sql_string(name))
        .collect::<Vec<String>>()
        .join(", ");
    let retired = RETIRED_STAT_PROVIDERS
        .iter()
        .map(|name| sql_string(name))
        .collect::<Vec<String>>()
        .join(", ");
    let mut rules_1 = String::new();
    let mut rules_2 = String::new();
    for (match_, author) in MODEL_AUTHOR_RULES {
        rules_1.push_str(&format!(
            "      WHEN strpos(lower({provider_model}), {match}) > 0 THEN {author}\n",
            match = sql_string(match_),
            author = sql_string(author)
        ));
        rules_2.push_str(&format!(
            "      WHEN strpos(lower({model}), {match}) > 0 THEN {author}\n",
            match = sql_string(match_),
            author = sql_string(author)
        ));
    }
    format!(
        "CASE
      WHEN lower({model}) IN ({stealth}) THEN 'unknown'
{rules_1}{rules_2}      WHEN {provider} <> '' AND lower({provider}) NOT IN ({retired}) THEN {provider}
      ELSE 'unknown'
    END"
    )
}

/// `iso_8601`: ECMAScript `Date.toISOString()` for epoch milliseconds.
pub fn iso_8601(ms: i64) -> String {
    let days = ms.div_euclid(86_400_000);
    let (year, month, day) = date_like::civil_from_days(days);
    let rem = ms.rem_euclid(86_400_000);
    let hour = rem / 3_600_000;
    let minute = (rem / 60_000) % 60;
    let second = (rem / 1000) % 60;
    let millis = rem % 1000;
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        year, month, day, hour, minute, second, millis
    )
}
