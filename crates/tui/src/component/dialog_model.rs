// source: packages/tui/src/component/dialog-model.tsx (197 lines, v1.18.30)
// 1:1 port — favorites/recents/provider sections, opencode `-nano`
// exclusion, `Free` footers, provider-name sort with opencode first,
// `sortModelOptions` ordering, needle search, and the variant follow-up
// (`NextAction::ShowVariant`) after `model.set`.

#![allow(dead_code)]

use serde_json::Value;

use crate::context::local::{LocalContext, ModelId};
use crate::context::sync::SyncStore;
use crate::ui::dialog_select::{
    fuzzy_score, SelectAction, SelectDisabled, SelectOption, SelectSide, SelectState,
};

/// Follow-up after a model pick (mirrors the variant branch in `onSelect`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelNext {
    Clear,
    ShowVariant,
}

/// One provider entry for the model dialog.
#[derive(Debug, Clone)]
pub struct ModelProviderInfo {
    pub id: String,
    pub name: String,
    pub models: Vec<ModelInfo>,
}

/// One model entry for the model dialog.
#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub id: String,
    pub provider_id: String,
    pub name: String,
    pub release_date: String,
    pub status: Option<String>,
    pub free: bool,
}

/// Collect provider/model infos from the sync store.
pub fn model_providers(sync: &SyncStore) -> Vec<ModelProviderInfo> {
    sync.provider
        .iter()
        .map(|provider| {
            let id = provider
                .get("id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let name = provider
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&id)
                .to_string();
            let mut models: Vec<ModelInfo> = provider
                .get("models")
                .and_then(|m| m.as_object())
                .map(|map| {
                    map.iter()
                        .map(|(model_id, info)| ModelInfo {
                            id: model_id.clone(),
                            provider_id: id.clone(),
                            name: info
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or(model_id)
                                .to_string(),
                            release_date: info
                                .get("release_date")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            status: info
                                .get("status")
                                .and_then(|v| v.as_str())
                                .map(str::to_string),
                            free: info
                                .get("cost")
                                .and_then(|c| c.get("input"))
                                .and_then(|v| v.as_f64())
                                == Some(0.0),
                        })
                        .collect()
                })
                .unwrap_or_default();
            models.sort_by(|a, b| a.id.cmp(&b.id));
            ModelProviderInfo { id, name, models }
        })
        .collect()
}

/// Mirrors `sortModelOptions` — free models last unless newest-first.
pub fn sort_model_options(options: &mut [ModelListOption], newest_first: bool) {
    options.sort_by(|a, b| {
        if newest_first {
            b.release_date
                .cmp(&a.release_date)
                .then_with(|| a.title.cmp(&b.title))
        } else {
            (a.footer.as_deref() != Some("Free"))
                .cmp(&(b.footer.as_deref() != Some("Free")))
                .then_with(|| b.release_date.cmp(&a.release_date))
                .then_with(|| a.title.cmp(&b.title))
        }
    });
}

/// Working option with sort metadata.
#[derive(Debug, Clone)]
pub struct ModelListOption {
    pub value: ModelId,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub disabled: bool,
    pub footer: Option<String>,
    pub release_date: String,
}

/// Build the full option list (sections + needle search, verbatim order).
pub fn model_options(
    local: &LocalContext,
    providers: &[ModelProviderInfo],
    connected: bool,
    provider_filter: Option<&str>,
    popular: Vec<SelectOption>,
    needle: &str,
) -> Vec<ModelListOption> {
    let needle = needle.trim();
    let show_sections = connected && provider_filter.is_none() && needle.is_empty();
    let favorites = if connected {
        local.model.favorite.clone()
    } else {
        Vec::new()
    };
    let recents = local.model.recent.clone();

    let to_options = |items: &[ModelId], category: &str| -> Vec<ModelListOption> {
        if !show_sections {
            return Vec::new();
        }
        items
            .iter()
            .filter_map(|item| {
                let provider = providers.iter().find(|p| p.id == item.provider_id)?;
                let model = provider.models.iter().find(|m| m.id == item.model_id)?;
                Some(ModelListOption {
                    value: ModelId {
                        provider_id: provider.id.clone(),
                        model_id: model.id.clone(),
                    },
                    title: if model.name.is_empty() {
                        item.model_id.clone()
                    } else {
                        model.name.clone()
                    },
                    description: Some(provider.name.clone()),
                    category: Some(category.to_string()),
                    disabled: provider.id == "opencode" && model.id.contains("-nano"),
                    footer: if model.free && provider.id == "opencode" {
                        Some("Free".to_string())
                    } else {
                        None
                    },
                    release_date: model.release_date.clone(),
                })
            })
            .collect()
    };

    let favorite_options = to_options(&favorites, "Favorites");
    let recent_options = to_options(
        &recents
            .iter()
            .filter(|item| !favorites.iter().any(|fav| fav == *item))
            .cloned()
            .collect::<Vec<_>>(),
        "Recent",
    );

    let mut provider_options: Vec<ModelListOption> = {
        let mut sorted = providers.to_vec();
        sorted.sort_by(|a, b| {
            (a.id != "opencode")
                .cmp(&(b.id != "opencode"))
                .then_with(|| a.name.cmp(&b.name))
        });
        let mut out = Vec::new();
        for provider in &sorted {
            let mut models: Vec<ModelListOption> = provider
                .models
                .iter()
                .filter(|info| info.status.as_deref() != Some("deprecated"))
                .filter(|info| {
                    provider_filter
                        .map(|f| info.provider_id == f)
                        .unwrap_or(true)
                })
                .map(|info| ModelListOption {
                    value: ModelId {
                        provider_id: provider.id.clone(),
                        model_id: info.id.clone(),
                    },
                    title: if info.name.is_empty() {
                        info.id.clone()
                    } else {
                        info.name.clone()
                    },
                    description: if favorites
                        .iter()
                        .any(|item| item.provider_id == provider.id && item.model_id == info.id)
                    {
                        Some("(Favorite)".to_string())
                    } else {
                        None
                    },
                    category: if connected {
                        Some(provider.name.clone())
                    } else {
                        None
                    },
                    disabled: provider.id == "opencode" && info.id.contains("-nano"),
                    footer: if info.free && provider.id == "opencode" {
                        Some("Free".to_string())
                    } else {
                        None
                    },
                    release_date: info.release_date.clone(),
                })
                .filter(|option| {
                    if !show_sections {
                        return true;
                    }
                    if favorites.iter().any(|item| {
                        item.provider_id == option.value.provider_id
                            && item.model_id == option.value.model_id
                    }) {
                        return false;
                    }
                    if recents.iter().any(|item| {
                        item.provider_id == option.value.provider_id
                            && item.model_id == option.value.model_id
                    }) {
                        return false;
                    }
                    true
                })
                .collect();
            sort_model_options(&mut models, provider_filter.is_some());
            out.extend(models);
        }
        out
    };

    let popular_options: Vec<ModelListOption> = if connected {
        Vec::new()
    } else {
        popular
            .into_iter()
            .take(6)
            .map(|option| ModelListOption {
                value: ModelId {
                    provider_id: String::new(),
                    model_id: option.title.clone(),
                },
                title: option.title,
                description: option.description,
                category: Some("Popular providers".to_string()),
                disabled: false,
                footer: option.footer,
                release_date: String::new(),
            })
            .collect()
    };

    if !needle.is_empty() {
        let mut searched: Vec<ModelListOption> = provider_options
            .into_iter()
            .filter(|option| {
                fuzzy_score(
                    &needle.to_lowercase(),
                    &option.title.to_lowercase(),
                    option.category.as_deref(),
                )
                .is_some()
            })
            .collect();
        sort_model_options(&mut searched, false);
        let mut popular_searched: Vec<ModelListOption> = popular_options
            .into_iter()
            .filter(|option| {
                fuzzy_score(&needle.to_lowercase(), &option.title.to_lowercase(), None).is_some()
            })
            .collect();
        searched.append(&mut popular_searched);
        return searched;
    }

    let mut out = favorite_options;
    out.extend(recent_options);
    out.append(&mut provider_options);
    out.extend(popular_options);
    out
}

/// Build the model select state (title, flat, skipFilter, current).
pub fn model_state(
    title: &str,
    options: Vec<ModelListOption>,
    current: Option<&ModelId>,
    connected: bool,
    provider_action_title: &str,
) -> SelectState {
    let select_options = options
        .into_iter()
        .map(|option| SelectOption {
            title: option.title,
            description: option.description,
            category: option.category,
            disabled: option.disabled,
            footer: option.footer,
            value: Value::String(format!(
                "{}/{}",
                option.value.provider_id, option.value.model_id
            )),
            ..SelectOption::default()
        })
        .collect();
    let mut state = SelectState::new(title, select_options);
    state.flat = true;
    state.skip_filter = true;
    state.current = current.map(|m| Value::String(format!("{}/{}", m.provider_id, m.model_id)));
    state.actions = vec![
        SelectAction {
            command: "model.dialog.provider".to_string(),
            title: provider_action_title.to_string(),
            side: SelectSide::Left,
            hidden: false,
            disabled: crate::ui::dialog_select::SelectDisabled::Flag(false),
            on_trigger: Box::new(|_| {}),
        },
        SelectAction {
            command: "model.dialog.favorite".to_string(),
            title: "Favorite".to_string(),
            side: SelectSide::Left,
            hidden: !connected,
            disabled: crate::ui::dialog_select::SelectDisabled::Flag(false),
            on_trigger: Box::new(|_| {}),
        },
    ];
    state
}

/// Mirror of the dialog's `onSelect` follow-up (variant step or clear).
pub fn model_next(
    local: &LocalContext,
    sync: &SyncStore,
    args_model: Option<&str>,
    config_model: Option<&str>,
) -> ModelNext {
    let list = local.variant_list(sync, args_model, config_model);
    let current = local.variant_selected(sync, args_model, config_model);
    if current.as_deref() == Some("default")
        || current.as_ref().map(|c| list.contains(c)).unwrap_or(false)
    {
        return ModelNext::Clear;
    }
    if !list.is_empty() {
        return ModelNext::ShowVariant;
    }
    ModelNext::Clear
}
