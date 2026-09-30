// source: packages/stats/core/src/database.ts (1:1 port)
//
// The source builds on Effect (`Config`, `Layer`, `Context`) and the
// PlanetScale serverless client. Those runtimes do not exist in the Rust
// port; the connection/drizzle layer is represented by the settings and
// error types below, preserving the exact env-var names, defaults and
// service identifiers.

/// `DATABASE_URL` default from `Resource.StatsDatabase.url` is not available
/// in the port; callers must supply it explicitly via the environment.
pub const DATABASE_URL_ENV: &str = "DATABASE_URL";
pub const DATABASE_MIGRATIONS_DIR_ENV: &str = "DATABASE_MIGRATIONS_DIR";
pub const DEFAULT_MIGRATIONS_DIR: &str = "./migrations";

/// `DatabaseSettings`: validated database configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseSettings {
    pub url: String,
    pub migrations_dir: String,
}

/// `DatabaseSettingsError`: configuration decode failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseSettingsError {
    MissingUrl,
    EmptyUrl,
    MissingMigrationsDir,
    EmptyMigrationsDir,
}

/// `decodeDatabaseSettings`: `Schema.decodeUnknownSync(DatabaseSettings)`
/// (non-empty strings).
pub fn decode_database_settings(
    url: Option<&str>,
    migrations_dir: Option<&str>,
) -> Result<DatabaseSettings, DatabaseSettingsError> {
    let url = url.ok_or(DatabaseSettingsError::MissingUrl)?;
    if url.is_empty() {
        return Err(DatabaseSettingsError::EmptyUrl);
    }
    let migrations_dir = match migrations_dir {
        Some(value) if !value.is_empty() => value.to_string(),
        _ => return Err(DatabaseSettingsError::MissingMigrationsDir),
    };
    Ok(DatabaseSettings {
        url: url.to_string(),
        migrations_dir,
    })
}

/// `settingsFromEnv`: `Config.all` with defaults (migrations dir only; the
/// database URL is required in the port because `Resource.StatsDatabase.url`
/// is unavailable).
pub fn settings_from_env() -> Result<DatabaseSettings, DatabaseSettingsError> {
    let url = std::env::var(DATABASE_URL_ENV).ok();
    let migrations_dir = std::env::var(DATABASE_MIGRATIONS_DIR_ENV)
        .ok()
        .filter(|v| !v.is_empty());
    let migrations_dir = migrations_dir.unwrap_or_else(|| DEFAULT_MIGRATIONS_DIR.to_string());
    decode_database_settings(url.as_deref(), Some(&migrations_dir))
}

/// `DatabaseConfig` service identifier.
pub const DATABASE_CONFIG_SERVICE_ID: &str = "@opencode/stats/DatabaseConfig";
/// `DrizzleClient` service identifier.
pub const DRIZZLE_CLIENT_SERVICE_ID: &str = "@opencode/stats/DrizzleClient";

/// `DatabaseError`: wraps underlying driver failures.
#[derive(Debug)]
pub struct DatabaseError {
    pub message: String,
    pub cause: String,
}

impl DatabaseError {
    pub fn make(cause: impl std::fmt::Display) -> DatabaseError {
        DatabaseError {
            message: "Database operation failed".to_string(),
            cause: cause.to_string(),
        }
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.message, self.cause)
    }
}

impl std::error::Error for DatabaseError {}

/// `MigrationError`: migration application failure.
#[derive(Debug)]
pub struct MigrationError {
    pub message: String,
    pub cause: Option<String>,
    pub exit_code: Option<i32>,
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MigrationError {}

/// `MigrationOutcome`: `migrate` returns Ok(()) when migrations applied;
/// Err(MigrationError) when the driver failed or reported a non-zero exit.
pub fn migration_failed_from_exit(exit_code: i32) -> String {
    format!("Failed to initialize database migrations: {exit_code}")
}
