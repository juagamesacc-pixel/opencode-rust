// source: packages/stats/core/src/domain/model-normalization.ts (1:1 port)

/// `MODEL_AUTHOR_RULES`: model-name substring -> author mapping (ordered).
pub const MODEL_AUTHOR_RULES: [(&str, &str); 13] = [
    ("claude", "anthropic"),
    ("gemini", "google"),
    ("deepseek", "deepseek"),
    ("glm", "zhipu"),
    ("gpt", "openai"),
    ("grok", "xai"),
    ("hy3", "tencent"),
    ("kimi", "moonshot"),
    ("mimo", "xiaomi"),
    ("minimax", "minimax"),
    ("muse-spark", "meta"),
    ("nemotron", "nvidia"),
    ("qwen", "qwen"),
];

/// `EXCLUDED_MODELS`: models excluded from statistics entirely.
pub const EXCLUDED_MODELS: [&str; 1] = ["alpha-gpt-next"];

/// `STEALTH_MODELS`: models mapped to the `unknown` provider.
pub const STEALTH_MODELS: [&str; 1] = ["omen-alpha"];

/// `FREE_MODELS`: models always treated as the Free tier.
pub const FREE_MODELS: [&str; 3] = ["gpt-5-nano", "grok-code", "big-pickle"];

/// `MODEL_NAME_ALIASES`: retired model names mapped onto their successors.
pub const MODEL_NAME_ALIASES: [(&str, &str); 5] = [
    ("deepseek-v4-flash-0731", "deepseek-v4-flash"),
    (
        "deepseek-v4-flash-dsv4-flash-final-rnaovd",
        "deepseek-v4-flash",
    ),
    ("ox-alpha", "glm-5.3-flash"),
    ("x-preview-f", "glm-5.3-flash"),
    ("xiaomi/mimo-v2.5", "mimo-v2.5"),
];

/// `RETIRED_STAT_MODELS`: all alias keys plus `big-pickle`.
pub const RETIRED_STAT_MODELS: [&str; 6] = [
    "big-pickle",
    "deepseek-v4-flash-0731",
    "deepseek-v4-flash-dsv4-flash-final-rnaovd",
    "ox-alpha",
    "x-preview-f",
    "xiaomi/mimo-v2.5",
];

/// `RETIRED_STAT_PROVIDERS`: providers excluded from provider dimensions.
pub const RETIRED_STAT_PROVIDERS: [&str; 1] = ["opencode"];

/// `normalizeInferenceModel`: lowercase, strips `-free`/`:free`/`:global` suffixes.
pub fn normalize_inference_model(value: Option<&str>) -> String {
    let lower = value.unwrap_or("unknown").to_lowercase();
    let trimmed = strip_suffixes(&lower);
    if trimmed.is_empty() {
        "unknown".to_string()
    } else {
        trimmed
    }
}

fn strip_suffixes(value: &str) -> String {
    let mut out = value.to_string();
    loop {
        let before = out.clone();
        for suffix in ["-free", ":free", ":global"] {
            if let Some(stripped) = out.strip_suffix(suffix) {
                out = stripped.to_string();
            }
        }
        if out == before {
            return out;
        }
    }
}

/// `modelAuthor`: author from the rule table, `unknown` when unmatched,
/// `None` for excluded models.
pub fn model_author(value: Option<&str>) -> Option<String> {
    let model = normalize_inference_model(value).to_lowercase();
    if EXCLUDED_MODELS.contains(&model.as_str()) {
        return None;
    }
    Some(
        MODEL_AUTHOR_RULES
            .iter()
            .find(|(match_, _)| model.contains(*match_))
            .map(|(_, author)| author.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
    )
}

/// `statModel`: resolves the canonical statistic model name (aliases applied).
pub fn stat_model(model: Option<&str>, provider_model: Option<&str>) -> String {
    let normalized = normalize_inference_model(model);
    let resolved = if normalized == "big-pickle" {
        let last = provider_model
            .unwrap_or("")
            .split('/')
            .next_back()
            .unwrap_or("");
        normalize_inference_model(Some(last))
    } else {
        normalized
    };
    MODEL_NAME_ALIASES
        .iter()
        .find(|(from, _)| resolved.to_lowercase() == *from)
        .map(|(_, to)| to.to_string())
        .unwrap_or(resolved)
}

/// `statProvider`: canonical provider for a statistic row.
pub fn stat_provider(
    model: Option<&str>,
    provider_model: Option<&str>,
    provider: Option<&str>,
) -> Option<String> {
    let normalized = stat_model(model, provider_model);
    if STEALTH_MODELS.contains(&normalized.as_str()) {
        return Some("unknown".to_string());
    }

    let model_author_value = model_author(Some(&normalized));
    let model_author_value = match model_author_value {
        Some(value) => value,
        None => return None,
    };

    let provider_model_author = model_author(provider_model);
    if let Some(author) = &provider_model_author {
        if author != "unknown" {
            return Some(author.clone());
        }
    }
    if model_author_value != "unknown" {
        return Some(model_author_value);
    }
    if let Some(provider) = provider {
        if !RETIRED_STAT_PROVIDERS.contains(&provider.to_lowercase().as_str()) {
            return Some(provider.to_string());
        }
    }
    Some(model_author_value)
}
