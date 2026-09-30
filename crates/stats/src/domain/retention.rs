// source: packages/stats/core/src/domain/retention.ts (1:1 port)

/// `RetentionStatRow`: retention-cohort ingestion row for the `model_retention` table.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RetentionStatRow {
    pub cohort_date: String,
    pub dataset: String,
    pub tier: String,
    pub provider: String,
    pub model: String,
    pub eligible_users: i64,
    pub retained_users: i64,
}

/// `rowsFromAggregates`: maps aggregate cohorts into retention rows.
pub fn rows_from_aggregates(aggregates: &[RetentionStatAggregate]) -> Vec<RetentionStatRow> {
    aggregates.iter().map(row_to_row).collect()
}

fn row_to_row(data: &RetentionStatAggregate) -> RetentionStatRow {
    RetentionStatRow {
        cohort_date: data.cohort_date.clone(),
        dataset: data.dataset.clone(),
        tier: data.tier.clone(),
        provider: data.provider.clone(),
        model: data.model.clone(),
        eligible_users: data.eligible_users,
        retained_users: data.retained_users,
    }
}

/// Source aggregate shape for [`row_to_row`].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RetentionStatAggregate {
    pub cohort_date: String,
    pub dataset: String,
    pub tier: String,
    pub provider: String,
    pub model: String,
    pub eligible_users: i64,
    pub retained_users: i64,
}

/// `isMissingRetentionTable`: recognizes the "missing column" error that
/// indicates the retention table does not exist yet in the target database.
pub fn is_missing_retention_table(cause: &str) -> bool {
    cause.contains("Unknown column 'retained_users'")
}

/// `UPSERT_CHUNK_SIZE`: retention rows are upserted in chunks of this size.
pub const UPSERT_CHUNK_SIZE: usize = 500;

/// PROVISIONAL descriptor of `RetentionStatRepo` (Effect/Planetscale runtime absent):
/// `available` probes the database for the `retained_users` column, `replace`
/// upserts the given rows in 500-row chunks with
/// `retained_users = VALUES(retained_users)` on duplicate key.
pub mod repo {
    pub const SERVICE_ID: &str = "@opencode/stats/RetentionStatRepo";
    pub const DUPLICATE_KEY_UPDATE_COLUMN: &str = "retained_users";
    pub const MISSING_COLUMN_TEXT: &str = "Unknown column 'retained_users'";
}
