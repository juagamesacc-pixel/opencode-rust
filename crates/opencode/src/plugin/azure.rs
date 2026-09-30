// source: src/plugin/azure.ts — exports: AzureAuthPlugin, createAzureAuthHooks
// PROVISIONAL pending plugin Hooks + core (which, installation/version) +
// @/auth + @/util/process: scopes, refresh buffer, CLI argv, scope rule,
// prompts, methods, instructions, expiry math, messages verbatim.

/// source: scopes — verbatim.
pub const AZURE_COGNITIVE_SCOPE: &str = "https://cognitiveservices.azure.com/.default";
pub const AZURE_FOUNDRY_SCOPE: &str = "https://ai.azure.com/.default";
/// source: AZURE_TOKEN_REFRESH_BUFFER = 60_000 — verbatim.
pub const AZURE_TOKEN_REFRESH_BUFFER_MS: i64 = 60_000;

/// source: CLI argv — verbatim.
pub fn azure_token_argv(scope: &str) -> Vec<String> {
    vec![
        "account".into(),
        "get-access-token".into(),
        "--scope".into(),
        scope.into(),
        "--output".into(),
        "json".into(),
    ]
}

/// source: "Azure CLI returned an invalid token expiration" — verbatim.
pub const INVALID_EXPIRY_MESSAGE: &str = "Azure CLI returned an invalid token expiration";

/// source: scopeForRequest() — foundry host + non-/models path rule. Verbatim.
pub fn scope_for_request(host: &str, path: &str) -> &'static str {
    if host.ends_with(".services.ai.azure.com") && !path.starts_with("/models") {
        return AZURE_FOUNDRY_SCOPE;
    }
    AZURE_COGNITIVE_SCOPE
}

/// source: prompts — resourceName text prompt when env missing. Verbatim strings.
pub const RESOURCE_KEY: &str = "resourceName";
pub const RESOURCE_MESSAGE: &str = "Enter Azure Resource Name";
pub const RESOURCE_PLACEHOLDER: &str = "e.g. my-models";
pub const RESOURCE_ENV: &str = "AZURE_RESOURCE_NAME";

/// source: provider/methods/labels/instructions — verbatim.
pub const AZURE_PROVIDER: &str = "azure";
pub const OAUTH_LABEL: &str = "Microsoft Entra ID (Azure CLI)";
pub const API_LABEL: &str = "API key";
pub const OAUTH_INSTRUCTIONS: &str = "Sign in with `az login` before continuing.";
/// source: "Azure Resource Name is required" — verbatim.
pub const RESOURCE_REQUIRED_MESSAGE: &str = "Azure Resource Name is required";
/// source: expiry +365d — verbatim.
pub const OAUTH_EXPIRY_DAYS: i64 = 365;
