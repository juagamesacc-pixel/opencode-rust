// source: core/src/drizzle/types.ts
//! 1:1 port of the shared column builders. Source pin: v1.18.30 @3104c14.
//!
//! The source returns `drizzle-orm/mysql-core` builder objects; here each builder
//! resolves to the exact DDL fragment the column contributes, so the emitted SQL
//! matches drizzle-kit's output byte for byte.

/// `ulid(name)` — `varchar(name, { length: 30 })`.
pub fn ulid(name: &str) -> String {
    format!("`{}` varchar(30)", name)
}

/// `workspaceColumns.id` — `ulid("id").notNull()`.
pub fn workspace_columns_id() -> String {
    format!("{} NOT NULL", ulid("id"))
}

/// `workspaceColumns.workspaceID` — `ulid("workspace_id").notNull()`.
pub fn workspace_columns_workspace_id() -> String {
    format!("{} NOT NULL", ulid("workspace_id"))
}

/// `id()` — `ulid("id").notNull()`.
pub fn id() -> String {
    format!("{} NOT NULL", ulid("id"))
}

/// `utc(name)` — `timestamp(name, { fsp: 3 })`.
pub fn utc(name: &str) -> String {
    format!("`{}` timestamp(3)", name)
}

/// `currency(name)` — `bigint(name, { mode: "number" })`.
pub fn currency(name: &str) -> String {
    format!("`{}` bigint", name)
}

/// `timestamps.timeCreated` — `utc("time_created").notNull().defaultNow()`.
pub fn timestamps_time_created() -> String {
    format!("{} NOT NULL DEFAULT (now())", utc("time_created"))
}

/// `timestamps.timeUpdated` — `utc("time_updated").notNull().default(
/// sql`CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)`)`.
pub fn timestamps_time_updated() -> String {
    format!(
        "{} NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)",
        utc("time_updated")
    )
}

/// `timestamps.timeDeleted` — nullable `utc("time_deleted")`.
pub fn timestamps_time_deleted() -> String {
    utc("time_deleted")
}
