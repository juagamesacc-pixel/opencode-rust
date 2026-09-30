// source: src/account/schema.ts — exports: AccountID, OrgID, AccessToken,
// RefreshToken, DeviceCode, UserCode, Info, Org, AccountRepoError,
// AccountServiceError, AccountTransportError, AccountError, Login,
// PollSuccess, PollPending, PollSlow, PollExpired, PollDenied, PollError, PollResult

use serde::{Deserialize, Serialize};

macro_rules! brand {
    ($name:ident) => {
        /// source: string brand — verbatim tag.
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);
    };
}

brand!(AccountID);
brand!(OrgID);
brand!(AccessToken);
brand!(RefreshToken);
brand!(DeviceCode);
brand!(UserCode);

/// source: Info ("Account" { id, email, url, active_org_id }) — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub id: AccountID,
    pub email: String,
    pub url: String,
    pub active_org_id: Option<OrgID>,
}

/// source: Org { id, name } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Org {
    pub id: OrgID,
    pub name: String,
}

/// source: AccountRepoError — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRepoError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

/// source: AccountServiceError — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountServiceError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

/// source: AccountTransportError { method, url, description?, cause? } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTransportError {
    pub method: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

impl AccountTransportError {
    /// source: message getter — verbatim 4-line template.
    pub fn message(&self) -> String {
        [
            Some(format!("Could not reach {} {}.", self.method, self.url)),
            Some("This failed before the server returned an HTTP response.".to_string()),
            self.description.clone(),
            Some("Check your network, proxy, or VPN configuration and try again.".to_string()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join("\n")
    }
}

/// source: AccountError union — verbatim members.
#[derive(Debug, Clone)]
pub enum AccountError {
    Repo(AccountRepoError),
    Service(AccountServiceError),
    Transport(AccountTransportError),
}

/// source: Login { code, user, url, server, expiry, interval } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Login {
    pub code: DeviceCode,
    pub user: UserCode,
    pub url: String,
    pub server: String,
    pub expiry: u64,
    pub interval: u64,
}

/// source: PollSuccess { email } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollSuccess {
    pub email: String,
}

/// source: PollPending — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollPending {}

/// source: PollSlow — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollSlow {}

/// source: PollExpired — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollExpired {}

/// source: PollDenied — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollDenied {}

/// source: PollError { cause } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollError {
    pub cause: String,
}

/// source: PollResult union — verbatim members/order.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PollResult {
    PollSuccess(PollSuccess),
    PollPending(PollPending),
    PollSlow(PollSlow),
    PollExpired(PollExpired),
    PollDenied(PollDenied),
    PollError(PollError),
}

/// source: DeviceTokenError.toPollResult() mapping — verbatim error strings.
pub fn device_error_to_poll(error: &str) -> PollResult {
    match error {
        "authorization_pending" => PollResult::PollPending(PollPending {}),
        "slow_down" => PollResult::PollSlow(PollSlow {}),
        "expired_token" => PollResult::PollExpired(PollExpired {}),
        "access_denied" => PollResult::PollDenied(PollDenied {}),
        other => PollResult::PollError(PollError {
            cause: other.to_string(),
        }),
    }
}
