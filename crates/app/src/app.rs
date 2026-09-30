//! Rust port of `packages/app/src/app.tsx` (opencode v1.18.30).
//!
//! Source 667 lines: SolidJS app shell (`AppBaseProviders`, `AppInterface`,
//! `SessionRoute`, `TargetServerRoute`, provider tree, router). No Rust
//! equivalent for SolidJS reactivity/router in the workspace — ported as
//! render-descriptor structs + explicit state-machine fns with verbatim
//! route/prop names. Rendering itself is PROVISIONAL.
//! Original file: `packages/app/src/app.tsx`

#![allow(dead_code)]

// PROVISIONAL: pending solid-js/solid-router/tanstack-query/sentry/effect — mirrors `packages/app/src/app.tsx`.
/// Mirrors the `AppBaseProviders` props (`locale`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppBaseProvidersProps {
    pub locale: String,
}

/// Mirrors the `AppInterface` props.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppInterfaceProps {
    pub default_server: String,
    pub canonical_local_server: String,
    pub servers: Vec<ServerDescriptor>,
    pub disable_health_check: bool,
}

/// Minimal descriptor for `ServerConnection.Http` values passed to `AppInterface`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerDescriptor {
    pub kind: String,
    pub url: String,
    pub has_auth: bool,
}

impl AppInterfaceProps {
    pub fn new(
        default_server: &str,
        canonical_local_server: &str,
        servers: Vec<ServerDescriptor>,
    ) -> Self {
        Self {
            default_server: default_server.to_string(),
            canonical_local_server: canonical_local_server.to_string(),
            servers,
            disable_health_check: false,
        }
    }
}

/// Mirrors the session-route param shapes (`params.id`, `draftId`/`prompt` search).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SessionRouteParams {
    pub id: Option<String>,
    pub draft_id: Option<String>,
    pub prompt: Option<String>,
    pub new_layout_designs: bool,
    pub tabs_ready: bool,
    pub directory: Option<String>,
}

/// Mirrors the `SessionRoute` new-layout redirect decision.
/// Returns the redirect href when the legacy `/:id` route must map to the
/// canonical server session href; `None` means render `SessionPage`.
pub fn session_route_redirect(params: &SessionRouteParams, canonical_href: &str) -> Option<String> {
    if params.id.is_some() && params.new_layout_designs && params.tabs_ready {
        return Some(canonical_href.to_string());
    }
    None
}

/// Mirrors the `SessionRoute` new-draft creation guard.
pub fn should_create_new_draft(params: &SessionRouteParams) -> bool {
    if !params.new_layout_designs {
        return false;
    }
    if params.id.is_some() || params.draft_id.is_some() {
        return false;
    }
    if !params.tabs_ready || params.directory.is_none() {
        return false;
    }
    true
}

/// Mirrors the provider-tree composition order (verbatim module names).
pub const APP_PROVIDERS: &[&str] = &[
    "CommandProvider",
    "CommentsProvider",
    "FileProvider",
    "ServerSDKProvider",
    "ServerSyncProvider",
    "GlobalProvider",
    "HighlightsProvider",
    "LanguageProvider",
    "LayoutProvider",
    "ModelsProvider",
    "NotificationProvider",
    "PermissionProvider",
    "PromptProvider",
    "ServerProvider",
    "SettingsProvider",
    "TabsProvider",
    "SDKProvider",
    "WslServersProvider",
];
