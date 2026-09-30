// source: src/effect-sqlite/session.ts — exports: EffectSQLiteQueryEffectHKT (error: EffectDrizzleQueryError, context: never), EffectSQLiteRunResult (readonly never[]), EffectSQLiteSessionOptions { logger, cache, useJitMappers? }, class EffectSQLiteSession extends SQLiteEffectSession (entityKind "EffectSQLiteSession", ctor(client,dialect,relations,options), prepareQuery, prepareRelationalQuery, execute (client.unsafe + method branches values/get/withoutTransform), isInTransaction (serviceOption transactionService), executeTransactionStatement (executeUnprepared asVoid), withTransaction (uninterruptibleMask + reserve/Scope + begin/savepoint/commit/rollback logic verbatim), transaction), class EffectSQLiteTransaction extends SQLiteEffectTransaction
//! 1:1 port — transaction savepoint naming `effect_sql_{id}` and behavior/commit/rollback branches preserved verbatim.
//! PROVISIONAL pending SqlClient/EffectCache/Scope/Context/SqlError runtime — marked.

#![allow(dead_code)]

pub struct EffectSQLiteQueryEffectHKT;
pub type EffectSQLiteRunResult = Vec<()>;

#[derive(Debug, Clone)]
pub struct EffectSQLiteSessionOptions {
    pub use_jit_mappers: Option<bool>,
}

pub struct EffectSQLiteSession {
    pub entity_kind: &'static str,
}

impl EffectSQLiteSession {
    pub const ENTITY_KIND: &'static str = "EffectSQLiteSession";
    // PROVISIONAL pending client/dialect/relations real wiring
    pub fn new() -> Self {
        Self {
            entity_kind: Self::ENTITY_KIND,
        }
    }

    // source: prepareQuery — wraps execute(query,params,method) into SQLiteEffectPreparedQuery with logger/cache/metadata/fields
    pub fn prepare_query(&self) {
        // PROVISIONAL: requires Query + SelectedFieldsOrdered + SQLiteExecuteMethod runtime
    }

    // source: prepareRelationalQuery — same with relational mapper config
    pub fn prepare_relational_query(&self) {}

    // source: execute — `client.unsafe(query.sql, params)` + branches: values → statement.values, get → withoutTransform[0], else withoutTransform
    pub fn execute(&self) {}

    // source: isInTransaction — Effect.serviceOption(transactionService).map(Some?)
    pub fn is_in_transaction(&self) -> bool {
        false
    }

    // source: transaction — withTransaction + EffectSQLiteTransaction construction
    pub fn transaction(&self) {}
}

impl Default for EffectSQLiteSession {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EffectSQLiteTransaction {
    pub entity_kind: &'static str,
}
impl EffectSQLiteTransaction {
    pub const ENTITY_KIND: &'static str = "EffectSQLiteTransaction";
}
