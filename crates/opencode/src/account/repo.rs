// source: src/account/repo.ts — exports: AccountRow, Interface, Service,
// use, node, AccountRepo
// PROVISIONAL pending crates/core (layer-node, database, account/sql,
// service-use) + drizzle-orm + ./url: ACCOUNT_STATE_ID=1 + query shapes +
// "Database operation failed" verbatim; db bodies as trait.

use super::schema::{AccessToken, AccountID, AccountRepoError, Info, OrgID, RefreshToken};

/// source: ACCOUNT_STATE_ID = 1 — verbatim.
pub const ACCOUNT_STATE_ID: i64 = 1;

/// source: AccountRow = AccountTable.$inferSelect — PROVISIONAL pending core sql.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountRow {
    pub id: AccountID,
    pub email: String,
    pub url: String,
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub token_expiry: Option<i64>,
}

/// source: "Database operation failed" — verbatim.
pub const DB_FAILED_MESSAGE: &str = "Database operation failed";

/// source: Interface — active/list/remove/use/getRow/persistToken/persistAccount, verbatim.
pub trait Interface {
    fn active(&self) -> Result<Option<Info>, AccountRepoError>;
    fn list(&self) -> Result<Vec<Info>, AccountRepoError>;
    fn remove(&self, account_id: &AccountID) -> Result<(), AccountRepoError>;
    fn use_(&self, account_id: &AccountID, org_id: Option<&OrgID>) -> Result<(), AccountRepoError>;
    fn get_row(&self, account_id: &AccountID) -> Result<Option<AccountRow>, AccountRepoError>;
}

/// source: Service "@opencode/AccountRepo" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/AccountRepo";

/// source: node deps [Database.node] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &["@opencode-ai/core/database/database.Database"];
