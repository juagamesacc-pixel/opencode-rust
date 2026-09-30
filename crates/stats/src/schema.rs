// source: packages/stats/core/src/database/schema.ts (1:1 port — Drizzle MySQL schema descriptors)

/// `Column`: descriptor for one MySQL column in the stats schema.
/// Mirrors the Drizzle column configuration used in the source
/// (`nullable` is the inverse of `.notNull()`, `auto_increment` of
/// `.autoincrement()`, `on_update` of `.onUpdateNow()`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub name: &'static str,
    pub kind: &'static str,
    pub nullable: bool,
    pub default: Option<&'static str>,
    pub auto_increment: bool,
    pub primary_key: bool,
    pub on_update_now: bool,
}

/// `Index`: descriptor for one MySQL index in the stats schema.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Index {
    pub name: &'static str,
    pub unique: bool,
    pub columns: Vec<&'static str>,
}

const fn col(
    name: &'static str,
    kind: &'static str,
    nullable: bool,
    default: Option<&'static str>,
    auto_increment: bool,
    primary_key: bool,
    on_update_now: bool,
) -> Column {
    Column {
        name,
        kind,
        nullable,
        default,
        auto_increment,
        primary_key,
        on_update_now,
    }
}

const fn ncol(name: &'static str, kind: &'static str) -> Column {
    col(name, kind, false, None, false, false, false)
}

/// `periodColumns`: shared grain/period/dataset/tier/client/source prefix.
const fn period_columns() -> [Column; 8] {
    [
        col("id", "bigint", false, None, true, true, false),
        ncol("grain", "varchar(16)"),
        ncol("period_key", "varchar(32)"),
        col(
            "dataset",
            "varchar(64)",
            false,
            Some("'all'"),
            false,
            false,
            false,
        ),
        col(
            "tier",
            "varchar(64)",
            false,
            Some("'all'"),
            false,
            false,
            false,
        ),
        col(
            "client",
            "varchar(64)",
            false,
            Some("'all'"),
            false,
            false,
            false,
        ),
        col(
            "source",
            "varchar(64)",
            false,
            Some("'all'"),
            false,
            false,
            false,
        ),
        ncol("provider", "varchar(128)"),
    ]
}

/// `metricColumns`: shared metric columns.
const fn metric_columns() -> [Column; 21] {
    [
        col("sessions", "bigint", false, Some("0"), false, false, false),
        col("requests", "bigint", false, Some("0"), false, false, false),
        col(
            "unique_users",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "input_tokens",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "output_tokens",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "reasoning_tokens",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "cache_read_tokens",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "total_tokens",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "input_cost_microcents",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "output_cost_microcents",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "total_cost_microcents",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "avg_duration_ms",
            "decimal(12,2)",
            true,
            None,
            false,
            false,
            false,
        ),
        col("p50_duration_ms", "int", true, None, false, false, false),
        col("p95_duration_ms", "int", true, None, false, false, false),
        col(
            "avg_ttfb_ms",
            "decimal(12,2)",
            true,
            None,
            false,
            false,
            false,
        ),
        col("p50_ttfb_ms", "int", true, None, false, false, false),
        col("p95_ttfb_ms", "int", true, None, false, false, false),
        col(
            "avg_output_tps",
            "decimal(12,4)",
            true,
            None,
            false,
            false,
            false,
        ),
        col(
            "success_count",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "error_count",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "sample_count",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
    ]
}

/// `marketShareColumns`: decimal market-share columns.
const fn market_share_columns() -> [Column; 3] {
    [
        col(
            "market_share_tokens",
            "decimal(10,6)",
            true,
            None,
            false,
            false,
            false,
        ),
        col(
            "market_share_requests",
            "decimal(10,6)",
            true,
            None,
            false,
            false,
            false,
        ),
        col(
            "market_share_sessions",
            "decimal(10,6)",
            true,
            None,
            false,
            false,
            false,
        ),
    ]
}

/// `timestampColumns`: created_at/updated_at pair.
const fn timestamp_columns() -> [Column; 2] {
    [
        col(
            "created_at",
            "datetime",
            false,
            Some("CURRENT_TIMESTAMP"),
            false,
            false,
            false,
        ),
        col(
            "updated_at",
            "datetime",
            false,
            Some("CURRENT_TIMESTAMP"),
            false,
            false,
            true,
        ),
    ]
}

/// `Table`: descriptor for one stats MySQL table.
#[derive(Debug, Clone)]
pub struct Table {
    pub name: &'static str,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
}

macro_rules! table {
    ($name:literal, $columns:expr, $indexes:expr) => {
        Table {
            name: $name,
            columns: $columns.to_vec(),
            indexes: $indexes.to_vec(),
        }
    };
}

fn idx(name: &'static str, unique: bool, columns: &'static [&'static str]) -> Index {
    Index {
        name,
        unique,
        columns: columns.to_vec(),
    }
}

/// `modelStat`: `model_stat` table (period + provider/model dimensions +
/// metrics + rank columns).
pub fn model_stat() -> Table {
    let mut columns = period_columns().to_vec();
    columns.push(ncol("model", "varchar(256)"));
    columns.push(col(
        "provider_model",
        "varchar(256)",
        false,
        Some("''"),
        false,
        false,
        false,
    ));
    columns.extend(metric_columns());
    columns.push(col(
        "rank_by_tokens",
        "int",
        true,
        None,
        false,
        false,
        false,
    ));
    columns.push(col(
        "rank_by_requests",
        "int",
        true,
        None,
        false,
        false,
        false,
    ));
    columns.push(col("rank_by_cost", "int", true, None, false, false, false));
    columns.extend(timestamp_columns());
    table!(
        "model_stat",
        columns,
        [
            idx(
                "uniq_model_period",
                true,
                &[
                    "grain",
                    "period_key",
                    "dataset",
                    "tier",
                    "client",
                    "source",
                    "provider",
                    "model",
                ],
            ),
            idx(
                "idx_leaderboard_tokens",
                false,
                &["grain", "period_key", "dataset", "tier", "total_tokens"],
            ),
            idx("idx_model", false, &["model", "grain", "period_key"]),
        ]
    )
}

/// `providerStat`: `provider_stat` table.
pub fn provider_stat() -> Table {
    let mut columns = period_columns().to_vec();
    columns.extend(metric_columns());
    columns.extend(market_share_columns());
    columns.push(col(
        "rank_by_tokens",
        "int",
        true,
        None,
        false,
        false,
        false,
    ));
    columns.push(col(
        "rank_by_requests",
        "int",
        true,
        None,
        false,
        false,
        false,
    ));
    columns.push(col(
        "rank_by_sessions",
        "int",
        true,
        None,
        false,
        false,
        false,
    ));
    columns.push(col("rank_by_cost", "int", true, None, false, false, false));
    columns.extend(timestamp_columns());
    table!(
        "provider_stat",
        columns,
        [
            idx(
                "uniq_provider_period",
                true,
                &[
                    "grain",
                    "period_key",
                    "dataset",
                    "tier",
                    "client",
                    "source",
                    "provider"
                ],
            ),
            idx(
                "idx_provider_leaderboard_tokens",
                false,
                &["grain", "period_key", "dataset", "tier", "total_tokens"],
            ),
            idx(
                "idx_provider_market_share",
                false,
                &[
                    "grain",
                    "period_key",
                    "dataset",
                    "tier",
                    "market_share_tokens"
                ],
            ),
            idx(
                "idx_provider_rank",
                false,
                &["grain", "period_key", "dataset", "tier", "rank_by_tokens"],
            ),
            idx("idx_provider", false, &["provider", "grain", "period_key"]),
        ]
    )
}

/// `geoStat`: `geo_stat` table.
pub fn geo_stat() -> Table {
    let mut columns = period_columns().to_vec();
    // Source overrides the shared `provider` column with a "all" default.
    columns[7] = col(
        "provider",
        "varchar(128)",
        false,
        Some("'all'"),
        false,
        false,
        false,
    );
    columns.push(col(
        "model",
        "varchar(256)",
        false,
        Some("'all'"),
        false,
        false,
        false,
    ));
    columns.push(ncol("country", "char(2)"));
    columns.push(col(
        "continent",
        "varchar(8)",
        false,
        Some("''"),
        false,
        false,
        false,
    ));
    columns.extend(metric_columns());
    columns.extend(market_share_columns());
    columns.push(col(
        "rank_by_tokens",
        "int",
        true,
        None,
        false,
        false,
        false,
    ));
    columns.push(col(
        "rank_by_requests",
        "int",
        true,
        None,
        false,
        false,
        false,
    ));
    columns.push(col(
        "rank_by_sessions",
        "int",
        true,
        None,
        false,
        false,
        false,
    ));
    columns.push(col("rank_by_cost", "int", true, None, false, false, false));
    columns.extend(timestamp_columns());
    table!(
        "geo_stat",
        columns,
        [
            idx(
                "uniq_country_period",
                true,
                &[
                    "grain",
                    "period_key",
                    "dataset",
                    "tier",
                    "client",
                    "source",
                    "provider",
                    "model",
                    "country",
                ],
            ),
            idx(
                "idx_country_map_tokens",
                false,
                &["grain", "period_key", "dataset", "tier", "total_tokens"],
            ),
            idx(
                "idx_country_rank",
                false,
                &["grain", "period_key", "dataset", "tier", "rank_by_tokens"],
            ),
            idx("idx_country", false, &["country", "grain", "period_key"]),
            idx(
                "idx_continent",
                false,
                &["continent", "grain", "period_key"]
            ),
            idx(
                "idx_country_model",
                false,
                &["model", "country", "grain", "period_key"]
            ),
            idx(
                "idx_country_model_range",
                false,
                &[
                    "model",
                    "provider",
                    "grain",
                    "dataset",
                    "client",
                    "source",
                    "tier",
                    "period_key"
                ],
            ),
        ]
    )
}

/// `modelRetention`: `model_retention` table (auto-increment id + cohort rows).
pub fn model_retention() -> Table {
    let columns: Vec<Column> = vec![
        col("id", "bigint", false, None, true, true, false),
        ncol("cohort_date", "char(10)"),
        col(
            "dataset",
            "varchar(64)",
            false,
            Some("'all'"),
            false,
            false,
            false,
        ),
        col(
            "tier",
            "varchar(64)",
            false,
            Some("'all'"),
            false,
            false,
            false,
        ),
        ncol("provider", "varchar(128)"),
        ncol("model", "varchar(256)"),
        col(
            "eligible_users",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "retained_users",
            "bigint",
            false,
            Some("0"),
            false,
            false,
            false,
        ),
        col(
            "created_at",
            "datetime",
            false,
            Some("CURRENT_TIMESTAMP"),
            false,
            false,
            false,
        ),
        col(
            "updated_at",
            "datetime",
            false,
            Some("CURRENT_TIMESTAMP"),
            false,
            false,
            true,
        ),
    ];
    table!(
        "model_retention",
        columns,
        [
            idx(
                "uniq_model_retention_cohort",
                true,
                &["cohort_date", "dataset", "tier", "provider", "model"],
            ),
            idx(
                "idx_model_retention_recent",
                false,
                &["dataset", "tier", "cohort_date"]
            ),
            idx(
                "idx_model_retention_model",
                false,
                &["model", "cohort_date"]
            ),
        ]
    )
}

/// `tables`: all stats schema tables.
pub fn tables() -> Vec<Table> {
    vec![model_stat(), provider_stat(), geo_stat(), model_retention()]
}
