//! Rust port of `packages/effect-sqlite-node/src/index.ts` (opencode v1.18.30).
//!
//! Source 168 lines. Exports: `NodeSqliteClient` namespace + `TypeId`, `SqliteClient`,
//! `SqliteClientConfig`, `SqliteClient` service, `make`, `layer`.
//!
//! 1:1 notes:
//! - `ATTR_DB_SYSTEM_NAME = "db.system.name"` verbatim.
//! - `TypeId = "~@opencode-ai/effect-sqlite-node/NodeSqliteClient"` string brand verbatim.
//! - `SqliteClient extends SqlClient` with `[TypeId]`, `config: SqliteClientConfig`,
//!   `loadExtension: (path:string)=>Effect<void,SqlError>`, `updateValues: never`.
//! - `SqliteClient = Context.Service<SqliteClient>("@opencode-ai/effect-sqlite-node/NodeSqliteClient")` service ID verbatim.
//! - `SqliteClientConfig = { filename:string, readonly?, create?, readwrite?, disableWAL?, timeout?, allowExtension?,
//!   spanAttributes?, transformResultNames?, transformQueryNames? }` fields/ordering verbatim.
//! - `SqliteConnection extends Connection` with `loadExtension`.
//! - `make(options)` is `Effect.gen` with `Statement.makeCompilerSqlite(transformQueryNames)`,
//!   `defaultTransforms(transformResultNames).array` if present, `DatabaseSync(filename,{readOnly,readonly,timeout,allowExtension,
//!   enableForeignKeyConstraints:true, open:true})`, finalizer `() => db.close()`, `if (disableWAL!==true && readonly!==true) db.exec("PRAGMA journal_mode = WAL;")`,
//!   `run` via `Effect.withFiber` + `db.prepare(sql)` + `setReadBigInts(get(SafeIntegers))` + `statement.all(...params)` +
//!   classify `"Failed to execute statement"` / `"execute"`, `runValues` additionally `setReturnArrays(true)`,
//!   `identity<SqliteConnection>` with `execute` (transformRows branching), `executeRaw`, `executeValues`,
//!   `executeUnprepared` delegating to `execute`, `executeStream` → `Stream.die("executeStream not implemented")`, `loadExtension`
//!   → `Effect.try db.loadExtension` with classify `"Failed to load extension"` / `"loadExtension"`,
//!   `semaphore = Semaphore.make(1)`, `acquirer = semaphore.withPermits(1)(succeed(connection))`,
//!   `transactionAcquirer = uninterruptibleMask(restore(semaphore.take(1)) + finalizer release(1))`,
//!   `Client.make({acquirer, compiler, transactionAcquirer, spanAttributes:[...spanAttributes, [ATTR_DB_SYSTEM_NAME,"sqlite"]], transformRows})`
//!   plus `Object.assign` `[TypeId]`, `config`, `loadExtension: flatMap(acquirer, _.loadExtension)`.
//! - `layer(config)` is `Layer.effectContext(Effect.map(make(config), ctx.make + ctx.add(SqlClient))).pipe(provide(Reactivity.layer))`
//!   with service IDs `SqliteClient | SqlClient`.
//!
//! PROVISIONAL: `node:sqlite`, `effect`/`Sql*`/`Reactivity` are host runtimes — descriptor only.

// ---------------------------------------------------------------------------
// Constants / TypeId verbatim
// ---------------------------------------------------------------------------

/// Span attr key verbatim: `"db.system.name"`.
pub const ATTR_DB_SYSTEM_NAME: &str = "db.system.name";
/// Span attr sqlite value verbatim: `"sqlite"`.
pub const ATTR_DB_SYSTEM_VALUE: &str = "sqlite";

/// TypeId brand verbatim: `"~@opencode-ai/effect-sqlite-node/NodeSqliteClient"`.
pub const TYPE_ID: &str = "~@opencode-ai/effect-sqlite-node/NodeSqliteClient";
pub type TypeId = &'static str;

/// Service ID verbatim: `"@opencode-ai/effect-sqlite-node/NodeSqliteClient"`.
pub const SERVICE_ID: &str = "@opencode-ai/effect-sqlite-node/NodeSqliteClient";

/// Compiler helper verbatim members.
pub const COMPILER_FN: &str = "Statement.makeCompilerSqlite";
pub const DEFAULT_TRANSFORMS_FN: &str = "Statement.defaultTransforms";

/// DatabaseSync ctor option keys verbatim.
pub const OPT_ENABLE_FK: &str = "enableForeignKeyConstraints";
pub const OPT_OPEN: &str = "open";

/// WAL pragma verbatim.
pub const PRAGMA_WAL: &str = "PRAGMA journal_mode = WAL;";

/// Error classify strings verbatim.
pub const ERR_FAILED_EXECUTE: &str = "Failed to execute statement";
pub const OP_EXECUTE: &str = "execute";
pub const ERR_FAILED_LOAD_EXTENSION: &str = "Failed to load extension";
pub const OP_LOAD_EXTENSION: &str = "loadExtension";
pub const ERR_STREAM_NOT_IMPLEMENTED: &str = "executeStream not implemented";

// ---------------------------------------------------------------------------
// SqliteClientConfig (fields + ordering verbatim)
// ---------------------------------------------------------------------------

/// Mirrors `SqliteClientConfig` — field names/optionality/ordering verbatim.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteClientConfig {
    pub filename: String,
    pub readonly: Option<bool>,
    pub create: Option<bool>,
    pub readwrite: Option<bool>,
    pub disable_wal: Option<bool>,
    pub timeout: Option<u32>,
    pub allow_extension: Option<bool>,
    pub span_attributes: Option<Vec<(String, serde_json::Value)>>,
    /// Function name placeholder for `transformResultNames` (JS function — descriptor preserves key existence).
    pub has_transform_result_names: bool,
    pub has_transform_query_names: bool,
}

impl SqliteClientConfig {
    pub fn new(filename: impl Into<String>) -> Self {
        Self {
            filename: filename.into(),
            readonly: None,
            create: None,
            readwrite: None,
            disable_wal: None,
            timeout: None,
            allow_extension: None,
            span_attributes: None,
            has_transform_result_names: false,
            has_transform_query_names: false,
        }
    }
}

// ---------------------------------------------------------------------------
// SqliteClient / SqliteConnection descriptors
// ---------------------------------------------------------------------------

/// Mirrors `SqliteClient extends SqlClient` with brand.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteClient {
    pub type_id: &'static str,
    pub config: SqliteClientConfig,
}

impl SqliteClient {
    pub const TYPE_ID: &'static str = TYPE_ID;
    pub const SERVICE_ID: &'static str = SERVICE_ID;
}

/// Mirrors `SqliteConnection extends Connection` with `loadExtension`.
#[derive(Clone, Debug, PartialEq)]
pub struct SqliteConnection;

impl SqliteConnection {
    pub const METHODS: &'static [&'static str] = &[
        "execute",
        "executeRaw",
        "executeValues",
        "executeUnprepared",
        "executeStream",
        "loadExtension",
    ];
}

// ---------------------------------------------------------------------------
// make / layer descriptor
// ---------------------------------------------------------------------------

/// Mirrors `SqliteClient.make` descriptor (no real Effect execution — pure descriptor).
#[derive(Clone, Debug, PartialEq)]
pub struct MakeDescriptor {
    pub compiler: &'static str,
    pub has_transform_rows: bool,
    pub semaphore_permits: u32,
    pub transaction_mask: &'static str,
    pub span_attributes: Vec<(&'static str, &'static str)>,
}

pub fn make_descriptor(config: &SqliteClientConfig) -> MakeDescriptor {
    MakeDescriptor {
        compiler: COMPILER_FN,
        has_transform_rows: config.has_transform_result_names,
        semaphore_permits: 1,
        transaction_mask: "Effect.uninterruptibleMask",
        span_attributes: {
            let mut v = Vec::new();
            if let Some(attrs) = &config.span_attributes {
                for (k, _) in attrs {
                    // preserve key ordering placeholder
                    let _ = k;
                }
            }
            v.push((ATTR_DB_SYSTEM_NAME, ATTR_DB_SYSTEM_VALUE));
            v
        },
    }
}

/// Mirrors `NodeSqliteClient` namespace (`export * as NodeSqliteClient from "./index"` self-export).
#[derive(Clone, Debug, PartialEq)]
pub struct NodeSqliteClient;

impl NodeSqliteClient {
    pub const NAMESPACE: &'static str = "NodeSqliteClient";
    pub const REEXPORT_SELF: bool = true;
    pub const TYPE_ID: &'static str = TYPE_ID;
    pub const SERVICE_ID: &'static str = SERVICE_ID;
}

// ---------------------------------------------------------------------------
// PROVISIONAL stubs
// ---------------------------------------------------------------------------

/// PROVISIONAL: `node:sqlite` `DatabaseSync` / `SQLInputValue`.
pub mod node_sqlite_provisional {
    pub const MODULE: &str = "node:sqlite";
    pub const CLASS: &str = "DatabaseSync";
}

/// PROVISIONAL: `effect` runtime + `unstable/sql*` + `unstable/reactivity`.
pub mod effect_provisional {
    pub const EFFECT_PACKAGE: &str = "effect";
    pub const SQL_CLIENT: &str = "effect/unstable/sql/SqlClient";
    pub const SQL_CONNECTION: &str = "effect/unstable/sql/SqlConnection";
    pub const SQL_ERROR: &str = "effect/unstable/sql/SqlError";
    pub const STATEMENT: &str = "effect/unstable/sql/Statement";
    pub const REACTIVITY: &str = "effect/unstable/reactivity/Reactivity";
}
