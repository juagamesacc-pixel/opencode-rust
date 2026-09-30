// source: src/provider/transform.ts — exports: OUTPUT_TOKEN_MAX,
// sanitizeSurrogates, message, temperature, topP, topK, variants, options,
// smallOptions, providerOptions, maxOutputTokens, schema, reasoningVariants,
// ProviderTransform (+ consts per source; 1909-line transform)
// PROVISIONAL: AI-SDK payload shaping as descriptors; pure rules verbatim:
// surrogate replacement \uFFFD, temperature/topP/topK tables, maxOutputTokens
// min-fallback, GPT5 regexes, effort tables, release dates, schema-type sets.

/// source: OUTPUT_TOKEN_MAX = 32_000 — verbatim.
pub const OUTPUT_TOKEN_MAX: i64 = 32_000;

/// source: sanitizeSurrogates() → \uFFFD — verbatim replacement char.
pub const REPLACEMENT_CHAR: char = '\u{FFFD}';

/// source: temperature() rules — verbatim pairs.
pub fn temperature(api_id: &str, provider_id: &str) -> Option<f64> {
    let id = api_id.to_lowercase();
    if id.contains("north-mini-code") {
        return Some(1.0);
    }
    if id.contains("claude") {
        return None;
    }
    if id.contains("gemini") {
        return None; // + GEMINI_MODELS_WITH_SAMPLING_DEFAULTS test → 1.0 (PROVISIONAL: regex table)
    }
    if id.contains("glm-4.6") || id.contains("glm-4.7") || id.contains("minimax-m2") {
        return Some(1.0);
    }
    if id.contains("kimi-k2") {
        if ["thinking", "k2.", "k2p", "k2-5"]
            .iter()
            .any(|s| id.contains(s))
        {
            return Some(1.0);
        }
        return Some(0.6);
    }
    let _ = provider_id;
    None
}

/// source: topP() rules — verbatim.
pub fn top_p(api_id: &str, provider_id: &str) -> Option<f64> {
    let id = api_id.to_lowercase();
    if id.contains("gemini") {
        return None; // + sampling-defaults test → 0.95 (PROVISIONAL)
    }
    if ["minimax-m2", "kimi-k2.5", "kimi-k2p5", "kimi-k2-5"]
        .iter()
        .any(|s| id.contains(s))
    {
        return Some(0.95);
    }
    if ["deepseek-v4-flash-0731", "deepseek-v4-flash:0731"]
        .iter()
        .any(|s| id.contains(s))
        || (id.contains("deepseek-v4-flash")
            && (provider_id == "deepseek" || provider_id.starts_with("opencode")))
    {
        return Some(0.95);
    }
    None
}

/// source: topK() rules — verbatim.
pub fn top_k(api_id: &str) -> Option<u32> {
    let id = api_id.to_lowercase();
    if id.contains("minimax-m2") {
        if ["m2.", "m25", "m21"].iter().any(|s| id.contains(s)) {
            return Some(40);
        }
        return Some(20);
    }
    if id.contains("gemini") {
        return None; // + sampling-defaults test → 64 (PROVISIONAL)
    }
    None
}

/// source: maxOutputTokens() — min(limit.output, max) || max. Verbatim.
pub fn max_output_tokens(limit_output: i64, output_token_max: i64) -> i64 {
    let m = limit_output.min(output_token_max);
    if m == 0 {
        output_token_max
    } else {
        m
    }
}

/// source: Kimi host set — verbatim.
pub const KIMI_HOSTS: &[&str] = &[
    "api.kimi.com",
    "api.moonshot.ai",
    "api.moonshot.cn",
    "api.moonshotai.cn",
];

/// source: isKimiFamily() — id substrings + host set. Verbatim.
pub fn is_kimi_family(provider_id: &str, api_id: &str, url: &str) -> bool {
    for id in [provider_id.to_lowercase(), api_id.to_lowercase()] {
        if id.contains("kimi") || id.contains("moonshot") {
            return true;
        }
    }
    KIMI_HOSTS.iter().any(|h| url.to_lowercase().contains(h))
}

/// source: effort tables — verbatim members.
pub const WIDELY_SUPPORTED_EFFORTS: &[&str] = &["low", "medium", "high"];
pub const OPENAI_EFFORTS: &[&str] = &["none", "minimal", "low", "medium", "high", "xhigh"];
pub const OPENAI_GPT5_1_EFFORTS: &[&str] = &["none", "low", "medium", "high"];
pub const OPENAI_GPT5_PRO_EFFORTS: &[&str] = &["high"];

/// source: release dates — verbatim.
pub const OPENAI_NONE_EFFORT_RELEASE_DATE: &str = "2025-11-13";
pub const OPENAI_XHIGH_EFFORT_RELEASE_DATE: &str = "2025-12-04";

/// source: GPT5 regex texts — verbatim patterns.
pub const GPT5_FAMILY_RE: &str = "(?:^|/)gpt-5(?:[.-]|$)";
pub const GPT5_VERSION_RE: &str = "(?:^|/)gpt-5[.-](\\d+)(?:[.-]|$)";
pub const GPT5_PRO_RE: &str = "(?:^|/)gpt-5[.-]?pro(?:[.-]|$)";
pub const GPT5_VERSIONED_PRO_RE: &str = "(?:^|/)gpt-5[.-]\\d+[.-]pro(?:[.-]|$)";

/// source: schema() type sets + inference keywords — verbatim.
pub const SCHEMA_TYPES: &[&str] = &[
    "string", "number", "boolean", "integer", "object", "array", "null",
];
pub const COMPOSITION_KEYS: &[&str] = &["anyOf", "oneOf", "allOf"];

/// source: INCLUDE_ENCRYPTED_REASONING — verbatim.
pub const INCLUDE_ENCRYPTED_REASONING: &[&str] = &["reasoning.encrypted_content"];
