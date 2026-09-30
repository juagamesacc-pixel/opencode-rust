// source: src/sqlite-core/effect/db.ts — exports: SQLiteEffectDatabase (Effect-backed drizzle DB, extends BaseSQLiteDatabase, provides query builders)
//! PROVISIONAL pending drizzle-orm sqlite-core dialect + Effect query HKT

#![allow(dead_code)]
pub struct SQLiteEffectDatabase;
impl SQLiteEffectDatabase {
    pub const ENTITY_KIND: &'static str = "SQLiteEffectDatabase";
}
