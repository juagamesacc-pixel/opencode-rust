// source: packages/tui/src/component/dialog-console-org.tsx (135 lines, v1.18.30)
// 1:1 port — org options (active-first, account+org sort, `email  host`
// categories), loading/empty rows, error view copy (`Could not load
// orgs`), and the switch flow (active → clear; else switchOrg + dispose
// + `Switched to {org}` toast + clear).

#![allow(dead_code)]

use serde_json::Value;

use crate::ui::dialog_select::SelectOption;

/// One console org entry.
#[derive(Debug, Clone)]
pub struct ConsoleOrg {
    pub org_id: String,
    pub org_name: String,
    pub account_id: String,
    pub account_email: String,
    pub account_url: String,
    pub active: bool,
}

impl ConsoleOrg {
    pub fn from_value(value: &Value) -> Option<Self> {
        Some(Self {
            org_id: value.get("orgID")?.as_str()?.to_string(),
            org_name: value.get("orgName")?.as_str().unwrap_or("").to_string(),
            account_id: value.get("accountID")?.as_str()?.to_string(),
            account_email: value
                .get("accountEmail")?
                .as_str()
                .unwrap_or("")
                .to_string(),
            account_url: value.get("accountUrl")?.as_str().unwrap_or("").to_string(),
            active: value
                .get("active")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
        })
    }
}

/// Mirrors `accountHost` (invalid URLs pass through).
pub fn account_host(url: &str) -> String {
    match url.split("://").nth(1) {
        Some(rest) => rest.split('/').next().unwrap_or(url).to_string(),
        None => url.to_string(),
    }
}

/// Mirrors `accountLabel` (`email  host`, two spaces).
pub fn account_label(email: &str, url: &str) -> String {
    format!("{email}  {}", account_host(url))
}

/// Sort orgs (active first, then account label, then org name).
pub fn sort_orgs(orgs: &mut [ConsoleOrg]) {
    orgs.sort_by(|a, b| {
        (!a.active)
            .cmp(&(!b.active))
            .then_with(|| {
                account_label(&a.account_email, &a.account_url)
                    .cmp(&account_label(&b.account_email, &b.account_url))
            })
            .then_with(|| a.org_name.cmp(&b.org_name))
    });
}

/// Build org select options (loading/empty rows verbatim).
pub fn org_options(orgs: Option<&[ConsoleOrg]>) -> Vec<SelectOption> {
    match orgs {
        None => vec![SelectOption {
            title: "Loading orgs…".to_string(),
            value: Value::String("loading".to_string()),
            ..SelectOption::default()
        }],
        Some(list) => {
            if list.is_empty() {
                return vec![SelectOption {
                    title: "No orgs found".to_string(),
                    value: Value::String("empty".to_string()),
                    ..SelectOption::default()
                }];
            }
            let mut sorted = list.to_vec();
            sort_orgs(&mut sorted);
            sorted
                .into_iter()
                .map(|item| SelectOption {
                    title: item.org_name.clone(),
                    category: Some(account_label(&item.account_email, &item.account_url)),
                    category_view: Some(format!(
                        "{} {}",
                        item.account_email,
                        account_host(&item.account_url)
                    )),
                    value: serde_json::json!({
                        "accountID": item.account_id,
                        "orgID": item.org_id,
                        "orgName": item.org_name,
                        "active": item.active,
                    }),
                    ..SelectOption::default()
                })
                .collect()
        }
    }
}

/// Switch-org call params (mirrors `switchOrg({ accountID, orgID })`).
pub fn switch_org_params(org: &ConsoleOrg) -> Value {
    serde_json::json!({ "accountID": org.account_id, "orgID": org.org_id })
}

/// Switched toast message verbatim (`Switched to {org}`).
pub fn switched_message(org_name: &str) -> String {
    format!("Switched to {org_name}")
}

/// Error view copy verbatim.
pub const ORG_LOAD_ERROR_TITLE: &str = "Could not load orgs";
