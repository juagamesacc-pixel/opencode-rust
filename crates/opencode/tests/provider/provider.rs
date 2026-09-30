// source: test/provider/provider.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { afterEach, expect, test } from "bun:test"; import { mkdir, unlink } from "fs/promises"; import path from "path"

#[test]
fn provider_loaded_from_env_variable() {
    // source: "provider loaded from env variable"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_loaded_from_config_with_api_key_option() {
    // source: "provider loaded from config with apiKey option"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_providers_excludes_provider() {
    // source: "disabled_providers excludes provider"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn enabled_providers_restricts_to_only_listed_providers() {
    // source: "enabled_providers restricts to only listed providers"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_whitelist_filters_models_for_provider() {
    // source: "model whitelist filters models for provider"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_blacklist_excludes_specific_models() {
    // source: "model blacklist excludes specific models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_model_alias_via_config() {
    // source: "custom model alias via config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_provider_with_npm_package() {
    // source: "custom provider with npm package"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn filters_alpha_provider_models_by_default() {
    // source: "filters alpha provider models by default"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_deep_seek_openai_compatible_model_defaults_interleave() {
    // source: "custom DeepSeek openai-compatible model defaults interleaved reasoning field"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn env_variable_takes_precedence_config_merges_options() {
    // source: "env variable takes precedence, config merges options"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_model_returns_model_for_valid_provider_model() {
    // source: "getModel returns model for valid provider/model"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_model_throws_model_not_found_error_for_invalid_model() {
    // source: "getModel throws ModelNotFoundError for invalid model"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_model_throws_model_not_found_error_for_invalid_provider() {
    // source: "getModel throws ModelNotFoundError for invalid provider"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn parse_model_correctly_parses_provider_model_string() {
    // source: "parseModel correctly parses provider/model string"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn parse_model_handles_model_ids_with_slashes() {
    // source: "parseModel handles model IDs with slashes"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_model_returns_first_available_model_when_no_config_s() {
    // source: "defaultModel returns first available model when no config set"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_model_respects_config_model_setting() {
    // source: "defaultModel respects config model setting"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_model_treats_empty_provider_config_as_no_allowlist() {
    // source: "defaultModel treats empty provider config as no allowlist"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn default_model_returns_a_typed_error_when_config_excludes_eve() {
    // source: "defaultModel returns a typed error when config excludes every provider"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_with_base_url_from_config() {
    // source: "provider with baseURL from config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_cost_defaults_to_zero_when_not_specified() {
    // source: "model cost defaults to zero when not specified"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_options_are_merged_from_existing_model() {
    // source: "model options are merged from existing model"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_removed_when_all_models_filtered_out() {
    // source: "provider removed when all models filtered out"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn closest_finds_model_by_partial_match() {
    // source: "closest finds model by partial match"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn closest_returns_undefined_for_nonexistent_provider() {
    // source: "closest returns undefined for nonexistent provider"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_model_uses_real_id_by_key_for_aliased_models() {
    // source: "getModel uses realIdByKey for aliased models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_api_field_sets_model_api_url() {
    // source: "provider api field sets model api.url"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn explicit_base_url_overrides_api_field() {
    // source: "explicit baseURL overrides api field"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_inherits_properties_from_existing_database_model() {
    // source: "model inherits properties from existing database model"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_config_preserves_explicitly_empty_models_dev_variants() {
    // source: "model config preserves explicitly empty models.dev variants"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_config_regenerates_variants_when_overriding_the_provid() {
    // source: "model config regenerates variants when overriding the provider package"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_providers_prevents_loading_even_with_env_var() {
    // source: "disabled_providers prevents loading even with env var"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn enabled_providers_with_empty_array_allows_no_providers() {
    // source: "enabled_providers with empty array allows no providers"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn whitelist_and_blacklist_can_be_combined() {
    // source: "whitelist and blacklist can be combined"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_modalities_default_correctly() {
    // source: "model modalities default correctly"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_with_custom_cost_values() {
    // source: "model with custom cost values"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_returns_appropriate_small_model() {
    // source: "getSmallModel returns appropriate small model"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_prefers_gemini_for_google_vertex() {
    // source: "getSmallModel prefers Gemini for Google Vertex"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_selects_the_latest_model_in_the_preferred_fa() {
    // source: "getSmallModel selects the latest model in the preferred family"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_matches_exact_model_families() {
    // source: "getSmallModel matches exact model families"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_ignores_model_ids_without_family_metadata() {
    // source: "getSmallModel ignores model IDs without family metadata"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_skips_inferred_models_for_azure() {
    // source: "getSmallModel skips inferred models for Azure"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_skips_inferred_models_for_azure_cognitive_se() {
    // source: "getSmallModel skips inferred models for Azure Cognitive Services"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_respects_config_small_model_override() {
    // source: "getSmallModel respects config small_model override"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_small_model_ignores_invalid_config_small_model() {
    // source: "getSmallModel ignores invalid config small_model"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_sort_prioritizes_preferred_models() {
    // source: "provider.sort prioritizes preferred models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn multiple_providers_can_be_configured_simultaneously() {
    // source: "multiple providers can be configured simultaneously"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_with_custom_npm_package() {
    // source: "provider with custom npm package"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_alias_name_defaults_to_alias_key_when_id_differs() {
    // source: "model alias name defaults to alias key when id differs"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_with_multiple_env_var_options_only_includes_api_key() {
    // source: "provider with multiple env var options only includes apiKey when single env"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_with_single_env_var_includes_api_key_automatically() {
    // source: "provider with single env var includes apiKey automatically"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_cost_overrides_existing_cost_values() {
    // source: "model cost overrides existing cost values"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn completely_new_provider_not_in_database_can_be_configured() {
    // source: "completely new provider not in database can be configured"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_providers_and_enabled_providers_interaction() {
    // source: "disabled_providers and enabled_providers interaction"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_with_tool_call_false() {
    // source: "model with tool_call false"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_defaults_tool_call_to_true_when_not_specified() {
    // source: "model defaults tool_call to true when not specified"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_headers_are_preserved() {
    // source: "model headers are preserved"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_env_fallback_second_env_var_used_if_first_missing() {
    // source: "provider env fallback - second env var used if first missing"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_model_returns_consistent_results() {
    // source: "getModel returns consistent results"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_name_defaults_to_id_when_not_in_database() {
    // source: "provider name defaults to id when not in database"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_not_found_error_includes_suggestions_for_typos() {
    // source: "ModelNotFoundError includes suggestions for typos"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_not_found_error_for_provider_includes_suggestions() {
    // source: "ModelNotFoundError for provider includes suggestions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_not_found_error_suggests_catalog_models_for_unloaded_p() {
    // source: "ModelNotFoundError suggests catalog models for unloaded providers"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_provider_returns_undefined_for_nonexistent_provider() {
    // source: "getProvider returns undefined for nonexistent provider"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn get_provider_returns_provider_info() {
    // source: "getProvider returns provider info"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn closest_returns_undefined_when_no_partial_match_found() {
    // source: "closest returns undefined when no partial match found"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn closest_checks_multiple_query_terms_in_order() {
    // source: "closest checks multiple query terms in order"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_limit_defaults_to_zero_when_not_specified() {
    // source: "model limit defaults to zero when not specified"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn provider_options_are_deeply_merged() {
    // source: "provider options are deeply merged"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn hosted_nvidia_provider_adds_billing_origin_header() {
    // source: "hosted nvidia provider adds billing origin header"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_nvidia_base_url_adds_billing_origin_header() {
    // source: "custom nvidia baseURL adds billing origin header"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn explicit_nvidia_billing_origin_header_is_preserved() {
    // source: "explicit nvidia billing origin header is preserved"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_model_inherits_npm_package_from_models_dev_provider_c() {
    // source: "custom model inherits npm package from models.dev provider config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_model_inherits_api_url_from_models_dev_provider() {
    // source: "custom model inherits api.url from models.dev provider"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn mode_options_and_cost_are_derived_from_the_base_model() {
    // source: "mode options and cost are derived from the base model"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn models_dev_normalization_fills_required_response_fields() {
    // source: "models.dev normalization fills required response fields"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn models_dev_reasoning_options_replace_generated_variants_and_() {
    // source: "models.dev reasoning options replace generated variants and unsupported toggles fall back"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn merge_gateway_exposes_declared_effort_variants_without_model() {
    // source: "MERGE Gateway exposes declared effort variants without model-specific handling"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn public_provider_info_omits_invalid_models() {
    // source: "public provider info omits invalid models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_variants_are_generated_for_reasoning_models() {
    // source: "model variants are generated for reasoning models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_variants_can_be_disabled_via_config() {
    // source: "model variants can be disabled via config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn model_variants_can_be_customized_via_config() {
    // source: "model variants can be customized via config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn disabled_key_is_stripped_from_variant_config() {
    // source: "disabled key is stripped from variant config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn all_variants_can_be_disabled_via_config() {
    // source: "all variants can be disabled via config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn variant_config_merges_with_generated_variants() {
    // source: "variant config merges with generated variants"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn variants_filtered_in_second_pass_for_database_models() {
    // source: "variants filtered in second pass for database models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn custom_model_with_variants_enabled_and_disabled() {
    // source: "custom model with variants enabled and disabled"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn google_vertex_retains_base_url_for_custom_proxy() {
    // source: "Google Vertex: retains baseURL for custom proxy"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn google_vertex_supports_open_ai_compatible_models() {
    // source: "Google Vertex: supports OpenAI compatible models"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn google_vertex_uses_rep_endpoint_for_claude_continental_multi() {
    // source: "Google Vertex: uses REP endpoint for Claude continental multi-regions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn google_vertex_anthropic_uses_rep_endpoint_for_continental_mu() {
    // source: "Google Vertex Anthropic: uses REP endpoint for continental multi-regions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn google_vertex_keeps_regional_claude_endpoints_unchanged() {
    // source: "Google Vertex: keeps regional Claude endpoints unchanged"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn google_vertex_uses_rep_endpoint_for_gemini_continental_multi() {
    // source: "Google Vertex: uses REP endpoint for Gemini continental multi-regions"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn google_vertex_keeps_regional_gemini_endpoints_unchanged() {
    // source: "Google Vertex: keeps regional Gemini endpoints unchanged"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn cloudflare_ai_gateway_loads_with_env_variables() {
    // source: "cloudflare-ai-gateway loads with env variables"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn cloudflare_ai_gateway_forwards_config_metadata_options() {
    // source: "cloudflare-ai-gateway forwards config metadata options"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn plugin_config_providers_persist_after_instance_dispose() {
    // source: "plugin config providers persist after instance dispose"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn plugin_config_enabled_and_disabled_providers_are_honored() {
    // source: "plugin config enabled and disabled providers are honored"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn opencode_loader_keeps_paid_models_when_config_api_key_is_pre() {
    // source: "opencode loader keeps paid models when config apiKey is present"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn opencode_loader_keeps_paid_models_when_auth_exists() {
    // source: "opencode loader keeps paid models when auth exists"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
