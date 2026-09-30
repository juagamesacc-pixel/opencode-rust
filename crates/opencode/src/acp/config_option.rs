// source: src/acp/config-option.ts — exports: DEFAULT_VARIANT_VALUE,
// ConfigOptionModel, ConfigOptionProvider, ConfigOptionMode, ModelSelection,
// buildModelSelectOption, buildEffortSelectOption, buildModeSelectOption,
// buildConfigOptions, parseModelSelection, formatCurrentModelId, formatVariantName
// PROVISIONAL pending @agentclientprotocol/sdk SessionConfigOption: option
// envelopes + parse/format rules verbatim.

use serde::{Deserialize, Serialize};

/// source: DEFAULT_VARIANT_VALUE = "default" — verbatim.
pub const DEFAULT_VARIANT_VALUE: &str = "default";

/// source: model option ids/names/categories — verbatim.
pub const MODEL_OPTION_ID: &str = "model";
pub const MODEL_OPTION_NAME: &str = "Model";
pub const MODEL_CATEGORY: &str = "model";
pub const EFFORT_OPTION_ID: &str = "effort";
pub const EFFORT_OPTION_NAME: &str = "Effort";
pub const EFFORT_DESCRIPTION: &str = "Available effort levels for this model";
pub const EFFORT_CATEGORY: &str = "thought_level";
pub const MODE_OPTION_ID: &str = "mode";
pub const MODE_OPTION_NAME: &str = "Session Mode";
pub const MODE_CATEGORY: &str = "mode";

/// source: option value/name pair — verbatim shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectItem {
    pub value: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// source: parseModelSelection() — provider-prefix match, variant split on
/// last "/", bare fallback split on first "/". Verbatim.
#[allow(clippy::type_complexity)]
pub fn parse_model_selection(
    model_id: &str,
    providers: &[(&str, &[&str], &[(&str, &[&str])])],
) -> (String, String, Option<String>) {
    for (provider_id, models, variants) in providers {
        if let Some(rest) = model_id.strip_prefix(&format!("{}/", provider_id)) {
            if models.contains(&rest) {
                return (provider_id.to_string(), rest.to_string(), None);
            }
            if let Some(sep) = rest.rfind('/') {
                let (base, variant) = (&rest[..sep], &rest[sep + 1..]);
                if models.contains(&base)
                    && variants
                        .iter()
                        .any(|(m, vs)| *m == base && vs.contains(&variant))
                {
                    return (
                        provider_id.to_string(),
                        base.to_string(),
                        Some(variant.to_string()),
                    );
                }
            }
            return (provider_id.to_string(), rest.to_string(), None);
        }
    }
    match model_id.find('/') {
        None => (model_id.to_string(), String::new(), None),
        Some(i) => (
            model_id[..i].to_string(),
            model_id[i + 1..].to_string(),
            None,
        ),
    }
}

/// source: formatCurrentModelId() — base + optional /variant. Verbatim.
pub fn format_current_model_id(
    provider_id: &str,
    model_id: &str,
    variant: Option<&str>,
    include_variant: bool,
) -> String {
    let base = format!("{}/{}", provider_id, model_id);
    match (include_variant, variant) {
        (true, Some(v)) => format!("{}/{}", base, v),
        _ => base,
    }
}

/// source: formatVariantName() — split [_-], capitalize each. Verbatim.
pub fn format_variant_name(variant: &str) -> String {
    variant
        .split(['_', '-'])
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// source: selectVariant() — current if listed, else DEFAULT if listed, else [0]. Verbatim.
pub fn select_variant(current: Option<&str>, variants: &[String]) -> Option<String> {
    if let Some(v) = current {
        if variants.iter().any(|x| x == v) {
            return Some(v.to_string());
        }
    }
    if variants.iter().any(|x| x == DEFAULT_VARIANT_VALUE) {
        return Some(DEFAULT_VARIANT_VALUE.to_string());
    }
    variants.first().cloned()
}

/// source: model option value/name — `{provider}/{model}`, `{pname}/{mname}` (+ variant suffix). Verbatim.
pub fn model_value(provider_id: &str, model_id: &str, variant: Option<&str>) -> String {
    match variant {
        Some(v) => format!("{}/{}/{}", provider_id, model_id, v),
        None => format!("{}/{}", provider_id, model_id),
    }
}
