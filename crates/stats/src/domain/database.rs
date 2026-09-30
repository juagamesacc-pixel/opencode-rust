// source: packages/stats/core/src/database.ts (1:1 port)
// Mounted at `domain::database` per crates/stats/tests/parity.rs.
//
// Effect `Config`/`Layer`/`Context` machinery, the PlanetScale client, the
// drizzle ORM, and SST `Resource` bindings have no in-workspace Rust
// equivalent: runtime construction is PROVISIONAL. Names, env-var keys,
// defaults, and service IDs are mirrored exactly.

/// Env var carrying the database URL (`Config.nonEmptyString("DATABASE_URL")`).
pub const DATABASE_URL_ENV: &str = "DATABASE_URL";
/// Env var overriding the migrations directory.
pub const DATABASE_MIGRATIONS_DIR_ENV: &str = "DATABASE_MIGRATIONS_DIR";
/// Default migrations directory (`Config.withDefault("./migrations")`).
pub const DEFAULT_MIGRATIONS_DIR: &str = "./migrations";
/// Effect service ID of `DatabaseConfig` (`"@opencode/stats/DatabaseConfig"`).
pub const DATABASE_CONFIG_SERVICE_ID: &str = "@opencode/stats/DatabaseConfig";
/// Effect service ID of `DrizzleClient` (`"@opencode/stats/DrizzleClient"`).
pub const DRIZZLE_CLIENT_SERVICE_ID: &str = "@opencode/stats/DrizzleClient";

/// `DatabaseSettings` (database.ts:12): decoded `{ url, migrationsDir }`.
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseSettings {
    pub url: String,
    pub migrations_dir: String,
}

/// `DatabaseError` (database.ts:47): `"Database operation failed"`.
#[derive(Debug, Clone, PartialEq)]
pub struct DatabaseError {
    pub message: &'static str,
}

impl DatabaseError {
    pub fn make() -> Self {
        DatabaseError {
            message: "Database operation failed",
        }
    }
}

/// `MigrationError` (database.ts:62): migration failure with message + cause.
#[derive(Debug, Clone, PartialEq)]
pub struct MigrationError {
    pub message: String,
}

/// PROVISIONAL(database.ts:34-45,71-94): `DatabaseConfig.layer`,
/// `DrizzleClient.layer`, `makeDrizzle`, `migrate`, and the merged `layer`
/// need the Effect runtime, the PlanetScale client, drizzle-orm, and SST
/// resources — none of which exist in this workspace.
pub fn migrate() -> Result<(), MigrationError> {
    unimplemented!("PROVISIONAL(packages/stats/core/src/database.ts): Effect/drizzle/SST runtime")
}
