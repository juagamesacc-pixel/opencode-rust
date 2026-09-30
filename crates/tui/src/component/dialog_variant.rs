// source: packages/tui/src/component/dialog-variant.tsx (39 lines, v1.18.30)
// 1:1 port — `Default` head option plus variant list; selection applies
// the variant (`None` resets to default) and clears.

#![allow(dead_code)]

use crate::context::local::LocalContext;
use crate::context::sync::SyncStore;
use crate::ui::dialog_select::{SelectOption, SelectState};

/// Build the variant select state.
pub fn variant_state(
    local: &LocalContext,
    sync: &SyncStore,
    args_model: Option<&str>,
    config_model: Option<&str>,
) -> SelectState {
    let mut options = vec![SelectOption {
        value: serde_json::Value::String("default".to_string()),
        title: "Default".to_string(),
        ..SelectOption::default()
    }];
    options.extend(
        local
            .variant_list(sync, args_model, config_model)
            .into_iter()
            .map(|variant| SelectOption {
                value: serde_json::Value::String(variant.clone()),
                title: variant,
                ..SelectOption::default()
            }),
    );
    let mut state = SelectState::new("Select variant", options);
    state.flat = true;
    state.current = local
        .variant_selected(sync, args_model, config_model)
        .map(serde_json::Value::String);
    state
}
