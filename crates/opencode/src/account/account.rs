// source: src/account/account.ts — exports: AccountID, AccountError,
// AccountRepoError, AccountServiceError, AccountTransportError, AccessToken,
// RefreshToken, DeviceCode, UserCode, Info, Org, OrgID, Login, PollSuccess,
// PollPending, PollSlow, PollExpired, PollDenied, PollError, PollResult,
// AccountOrgs, ActiveOrg, Interface, Service, use, node, Account
// PROVISIONAL pending crates/core (layer-node, app-node-platform,
// service-use, database) + ./repo + ./url + @/util/*: client_id, refresh
// threshold, endpoint paths, grant types, messages verbatim.

use super::schema::{
    AccessToken, AccountError, AccountID, DeviceCode, Info, Login, Org, PollResult, UserCode,
};

/// source: clientId = "opencode-cli" — verbatim.
pub const CLIENT_ID: &str = "opencode-cli";
/// source: eagerRefreshThreshold = 5 minutes — verbatim.
pub const EAGER_REFRESH_THRESHOLD_MS: i64 = 5 * 60 * 1000;

/// source: isTokenFresh — expiry != null && expiry > now + threshold. Verbatim.
pub fn is_token_fresh(token_expiry: Option<i64>, now: i64) -> bool {
    matches!(token_expiry, Some(e) if e > now + EAGER_REFRESH_THRESHOLD_MS)
}

/// source: default mapAccountServiceError message — verbatim.
pub const DEFAULT_SERVICE_MESSAGE: &str = "Account service operation failed";
/// source: "HTTP request failed" — verbatim.
pub const HTTP_FAILED_MESSAGE: &str = "HTTP request failed";
/// source: "Failed to decode response" — verbatim.
pub const DECODE_FAILED_MESSAGE: &str = "Failed to decode response";
/// source: "Account not found during token refresh" — verbatim.
pub const NOT_FOUND_REFRESH_MESSAGE: &str = "Account not found during token refresh";
/// source: "expected HTTP(S)" — verbatim.
pub const EXPECTED_HTTP_MESSAGE: &str = "expected HTTP(S)";
/// source: "Invalid device verification URL" — verbatim.
pub const INVALID_VERIFY_URL_MESSAGE: &str = "Invalid device verification URL";
/// source: device grant type — verbatim.
pub const GRANT_DEVICE_CODE: &str = "urn:ietf:params:oauth:grant-type:device_code";
/// source: refresh grant type — verbatim.
pub const GRANT_REFRESH: &str = "refresh_token";

/// source: endpoint paths — verbatim.
pub const PATH_TOKEN: &str = "/auth/device/token";
pub const PATH_CODE: &str = "/auth/device/code";
pub const PATH_ORGS: &str = "/api/orgs";
pub const PATH_USER: &str = "/api/user";
pub const PATH_CONFIG: &str = "/api/config";

/// source: "x-org-id" header — verbatim.
pub const ORG_HEADER: &str = "x-org-id";

/// source: AccountOrgs — verbatim.
#[derive(Debug, Clone)]
pub struct AccountOrgs {
    pub account: Info,
    pub orgs: Vec<Org>,
}

/// source: ActiveOrg — verbatim.
#[derive(Debug, Clone)]
pub struct ActiveOrg {
    pub account: Info,
    pub org: Org,
}

/// source: device auth response fields — verbatim.
#[derive(Debug, Clone)]
pub struct DeviceAuth {
    pub device_code: DeviceCode,
    pub user_code: UserCode,
    pub verification_uri_complete: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// source: Interface — active/activeOrg/list/orgsByAccount/remove/use/orgs/
/// config/token/login/poll, verbatim.
pub trait Interface {
    fn list(&self) -> Result<Vec<Info>, AccountError>;
    fn remove(&self, account_id: &AccountID) -> Result<(), AccountError>;
    fn orgs(&self, account_id: &AccountID) -> Result<Vec<Org>, AccountError>;
    fn token(&self, account_id: &AccountID) -> Result<Option<AccessToken>, AccountError>;
    fn poll(&self, input: &Login) -> Result<PollResult, AccountError>;
}

/// source: Service "@opencode/Account" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/Account";

/// source: node deps [AccountRepo.node, httpClient] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[
    "@/account/repo.AccountRepo",
    "@opencode-ai/core/effect/app-node-platform.httpClient",
];
