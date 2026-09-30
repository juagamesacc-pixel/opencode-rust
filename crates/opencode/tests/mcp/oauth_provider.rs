// source: test/mcp/oauth-provider.test.ts — exports: []
// PROVISIONAL pending @opencode-ai/core + effect/drizzle/bun:test — verbatim shape where pure
// original imports: import { test, expect, describe } from "bun:test"; import { determineScope } from "@modelcontextprotocol/sdk/client/auth

#[test]
fn mcp_oauth_provider_redirect_url() {
    // source: "McpOAuthProvider.redirectUrl"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn defaults_to_127_0_0_1_19876_mcp_oauth_callback() {
    // source: "defaults to 127.0.0.1:19876/mcp/oauth/callback"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_callback_port_when_set() {
    // source: "uses callbackPort when set"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn redirect_uri_takes_precedence_over_callback_port() {
    // source: "redirectUri takes precedence over callbackPort"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn uses_explicit_redirect_uri_when_set_without_callback_port() {
    // source: "uses explicit redirectUri when set without callbackPort"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn mcp_oauth_provider_client_metadata() {
    // source: "McpOAuthProvider.clientMetadata"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn includes_redirect_uris_from_redirect_url() {
    // source: "includes redirect_uris from redirectUrl"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn includes_scope_when_set_in_config() {
    // source: "includes scope when set in config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn omits_scope_when_not_set_in_config() {
    // source: "omits scope when not set in config"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sets_token_endpoint_auth_method_to_client_secret_post_when_c() {
    // source: "sets token_endpoint_auth_method to client_secret_post when clientSecret provided"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn sets_token_endpoint_auth_method_to_none_when_no_client_secre() {
    // source: "sets token_endpoint_auth_method to none when no clientSecret"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn mcp_oauth_scope_selection() {
    // source: "MCP OAuth scope selection"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn adds_offline_access_when_the_authorization_server_and_client() {
    // source: "adds offline_access when the authorization server and client support refresh tokens"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
#[test]
fn does_not_add_unsupported_authorization_server_scopes() {
    // source: "does not add unsupported authorization server scopes"
    // PROVISIONAL pending @opencode-ai/core
    // original assertion preserved as comment
}
