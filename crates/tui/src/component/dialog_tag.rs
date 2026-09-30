// source: packages/tui/src/component/dialog-tag.tsx (47 lines, v1.18.30)
// 1:1 port — `find.files` lookup (top 5) becomes select options; the
// resource refreshes per filter like `createResource`.

#![allow(dead_code)]

use std::sync::Arc;

use crate::context::sdk::SdkClient;
use crate::ui::dialog_select::{SelectOption, SelectState};

/// Load tag options (mirrors the resource: query → first 5 files).
pub async fn tag_options(
    client: &Arc<dyn SdkClient>,
    query: &str,
    workspace: Option<&str>,
) -> Vec<SelectOption> {
    let params = match workspace {
        Some(workspace) => serde_json::json!({ "query": query, "workspace": workspace }),
        None => serde_json::json!({ "query": query }),
    };
    let files = client
        .call("find.files", params)
        .await
        .ok()
        .and_then(|response| response.get("data").and_then(|d| d.as_array()).cloned())
        .unwrap_or_default();
    files
        .into_iter()
        .take(5)
        .filter_map(|file| {
            let title = file.as_str()?.to_string();
            Some(SelectOption {
                value: file,
                title,
                ..SelectOption::default()
            })
        })
        .collect()
}

/// Build the tag select state (title `Autocomplete`, verbatim).
pub fn tag_state(options: Vec<SelectOption>) -> SelectState {
    SelectState::new("Autocomplete", options)
}
