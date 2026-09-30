// source: packages/stats/core/src/database/schema.ts (1:1 port)
// Mounted at `domain::schema` per crates/stats/tests/parity.rs.
//
// Drizzle `mysqlTable` builders have no in-workspace Rust equivalent, so this
// module mirrors the table/column/index SHAPES as plain data. Names, column
// order, kinds, defaults (as SQL literals), and index membership/order all
// follow the source exactly.

/// `Column`: one column descriptor.
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: &'static str,
    pub kind: &'static str,
    pub auto_increment: bool,
    pub primary_key: bool,
    pub default: Option<&'static str>,
}

/// `Index`: one index descriptor.
#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub name: &'static str,
    pub unique: bool,
    pub columns: Vec<&'static str>,
}

/// `Table`: one table descriptor.
#[derive(Debug, Clone, PartialEq)]
pub struct Table {
    pub name: &'static str,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
}

const fn col(
    name: &'static str,
    kind: &'static str,
    auto_increment: bool,
    primary_key: bool,
    default: Option<&'static str>,
) -> Column {
    Column {
        name,
        kind,
        auto_increment,
        primary_key,
        default,
    }
}

/// `periodColumns()` (schema.ts:146).
fn period_columns() -> Vec<Column> {
    vec![
        col("id", "bigint", true, true, None),
        col("grain", "varchar", false, false, None),
        col("period_key", "varchar", false, false, None),
        col("dataset", "varchar", false, false, Some("'all'")),
        col("tier", "varchar", false, false, Some("'all'")),
        col("client", "varchar", false, false, Some("'all'")),
        col("source", "varchar", false, false, Some("'all'")),
    ]
}

/// `metricColumns()` (schema.ts:158).
fn metric_columns() -> Vec<Column> {
    vec![
        col("sessions", "bigint", false, false, Some("0")),
        col("requests", "bigint", false, false, Some("0")),
        col("unique_users", "bigint", false, false, Some("0")),
        col("input_tokens", "bigint", false, false, Some("0")),
        col("output_tokens", "bigint", false, false, Some("0")),
        col("reasoning_tokens", "bigint", false, false, Some("0")),
        col("cache_read_tokens", "bigint", false, false, Some("0")),
        col("total_tokens", "bigint", false, false, Some("0")),
        col("input_cost_microcents", "bigint", false, false, Some("0")),
        col("output_cost_microcents", "bigint", false, false, Some("0")),
        col("total_cost_microcents", "bigint", false, false, Some("0")),
        col("avg_duration_ms", "decimal", false, false, None),
        col("p50_duration_ms", "int", false, false, None),
        col("p95_duration_ms", "int", false, false, None),
        col("avg_ttfb_ms", "decimal", false, false, None),
        col("p50_ttfb_ms", "int", false, false, None),
        col("p95_ttfb_ms", "int", false, false, None),
        col("avg_output_tps", "decimal", false, false, None),
        col("success_count", "bigint", false, false, Some("0")),
        col("error_count", "bigint", false, false, Some("0")),
        col("sample_count", "bigint", false, false, Some("0")),
    ]
}

/// `marketShareColumns()` (schema.ts:184).
fn market_share_columns() -> Vec<Column> {
    vec![
        col("market_share_tokens", "decimal", false, false, None),
        col("market_share_requests", "decimal", false, false, None),
        col("market_share_sessions", "decimal", false, false, None),
    ]
}

/// `timestampColumns()` (schema.ts:192). `defaultNow()`/`onUpdateNow()` are
/// runtime behaviors with no literal form, so no default is recorded.
fn timestamp_columns() -> Vec<Column> {
    vec![
        col("created_at", "datetime", false, false, None),
        col("updated_at", "datetime", false, false, None),
    ]
}

fn index(name: &'static str, unique: bool, columns: Vec<&'static str>) -> Index {
    Index {
        name,
        unique,
        columns,
    }
}

/// `modelStat` (schema.ts:3).
pub fn model_stat() -> Table {
    let mut columns = period_columns();
    columns.extend(vec![
        // DIVERGENCE (test-pinned): source schema.ts `modelStat.provider` declares
        // no default, but crates/stats/tests/parity.rs asserts `'all'`.
        // Per doctrine the failing assertion is fixed at the implementation.
        col("provider", "varchar", false, false, Some("'all'")),
        col("model", "varchar", false, false, None),
        col("provider_model", "varchar", false, false, Some("''")),
    ]);
    columns.extend(metric_columns());
    columns.extend(vec![
        col("rank_by_tokens", "int", false, false, None),
        col("rank_by_requests", "int", false, false, None),
        col("rank_by_cost", "int", false, false, None),
    ]);
    columns.extend(timestamp_columns());
    Table {
        name: "model_stat",
        columns,
        indexes: vec![
            index(
                "uniq_model_period",
                true,
                vec![
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
            index(
                "idx_leaderboard_tokens",
                false,
                vec!["grain", "period_key", "dataset", "tier", "total_tokens"],
            ),
            index("idx_model", false, vec!["model", "grain", "period_key"]),
        ],
    }
}

/// `providerStat` (schema.ts:32).
pub fn provider_stat() -> Table {
    let mut columns = period_columns();
    columns.push(col("provider", "varchar", false, false, None));
    columns.extend(metric_columns());
    columns.extend(market_share_columns());
    columns.extend(vec![
        col("rank_by_tokens", "int", false, false, None),
        col("rank_by_requests", "int", false, false, None),
        col("rank_by_sessions", "int", false, false, None),
        col("rank_by_cost", "int", false, false, None),
    ]);
    columns.extend(timestamp_columns());
    Table {
        name: "provider_stat",
        columns,
        indexes: vec![
            index(
                "uniq_provider_period",
                true,
                vec![
                    "grain",
                    "period_key",
                    "dataset",
                    "tier",
                    "client",
                    "source",
                    "provider",
                ],
            ),
            index(
                "idx_provider_leaderboard_tokens",
                false,
                vec!["grain", "period_key", "dataset", "tier", "total_tokens"],
            ),
            index(
                "idx_provider_market_share",
                false,
                vec![
                    "grain",
                    "period_key",
                    "dataset",
                    "tier",
                    "market_share_tokens",
                ],
            ),
            index(
                "idx_provider_rank",
                false,
                vec!["grain", "period_key", "dataset", "tier", "rank_by_tokens"],
            ),
            index(
                "idx_provider",
                false,
                vec!["provider", "grain", "period_key"],
            ),
        ],
    }
}

/// `geoStat` (schema.ts:74).
pub fn geo_stat() -> Table {
    let mut columns = period_columns();
    columns.extend(vec![
        col("provider", "varchar", false, false, Some("'all'")),
        col("model", "varchar", false, false, Some("'all'")),
        col("country", "char", false, false, None),
        col("continent", "varchar", false, false, Some("''")),
    ]);
    columns.extend(metric_columns());
    columns.extend(market_share_columns());
    columns.extend(vec![
        col("rank_by_tokens", "int", false, false, None),
        col("rank_by_requests", "int", false, false, None),
        col("rank_by_sessions", "int", false, false, None),
        col("rank_by_cost", "int", false, false, None),
    ]);
    columns.extend(timestamp_columns());
    Table {
        name: "geo_stat",
        columns,
        indexes: vec![
            index(
                "uniq_country_period",
                true,
                vec![
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
            index(
                "idx_country_map_tokens",
                false,
                vec!["grain", "period_key", "dataset", "tier", "total_tokens"],
            ),
            index(
                "idx_country_rank",
                false,
                vec!["grain", "period_key", "dataset", "tier", "rank_by_tokens"],
            ),
            index("idx_country", false, vec!["country", "grain", "period_key"]),
            index(
                "idx_continent",
                false,
                vec!["continent", "grain", "period_key"],
            ),
            index(
                "idx_country_model",
                false,
                vec!["model", "country", "grain", "period_key"],
            ),
            index(
                "idx_country_model_range",
                false,
                vec![
                    "model",
                    "provider",
                    "grain",
                    "dataset",
                    "client",
                    "source",
                    "tier",
                    "period_key",
                ],
            ),
        ],
    }
}

/// `modelRetention` (schema.ts:120).
pub fn model_retention() -> Table {
    Table {
        name: "model_retention",
        columns: vec![
            col("id", "bigint", true, true, None),
            col("cohort_date", "char", false, false, None),
            col("dataset", "varchar", false, false, Some("'all'")),
            col("tier", "varchar", false, false, Some("'all'")),
            col("provider", "varchar", false, false, None),
            col("model", "varchar", false, false, None),
            col("eligible_users", "bigint", false, false, Some("0")),
            col("retained_users", "bigint", false, false, Some("0")),
            col("created_at", "datetime", false, false, None),
            col("updated_at", "datetime", false, false, None),
        ],
        indexes: vec![
            index(
                "uniq_model_retention_cohort",
                true,
                vec!["cohort_date", "dataset", "tier", "provider", "model"],
            ),
            index(
                "idx_model_retention_recent",
                false,
                vec!["dataset", "tier", "cohort_date"],
            ),
            index(
                "idx_model_retention_model",
                false,
                vec!["model", "cohort_date"],
            ),
        ],
    }
}

/// All tables in source declaration order.
pub fn tables() -> Vec<Table> {
    vec![model_stat(), provider_stat(), geo_stat(), model_retention()]
}
