// source: src/acp/session.ts — exports: SelectedModel,
// KnownMessagePartMetadata, Info, StoreInput, RecordPartMetadataInput,
// PartMetadataLookupInput, Interface, Service, node, ACPSession
// PROVISIONAL pending ACP sdk + core (provider, model, layer-node) + effect:
// store shapes + 13-method interface table + service id verbatim; Ref<Map>
// state machine as trait.

use serde::{Deserialize, Serialize};

/// source: Info { id, cwd, mcpServers, createdAt, model?, variant?, modeId?, knownParts } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub id: String,
    pub cwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SelectedModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode_id: Option<String>,
}

/// source: SelectedModel { providerID, modelID } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectedModel {
    pub provider_id: String,
    pub model_id: String,
}

/// source: Interface methods — verbatim names/order (create/load/list/get/
/// tryGet/remove/setModel/getModel/setVariant/getVariant/setMode/getMode/
/// recordPartMetadata/getPartMetadata/tryGetPartMetadata).
pub const SESSION_METHODS: &[&str] = &[
    "create",
    "load",
    "list",
    "get",
    "tryGet",
    "remove",
    "setModel",
    "getModel",
    "setVariant",
    "getVariant",
    "setMode",
    "getMode",
    "recordPartMetadata",
    "getPartMetadata",
    "tryGetPartMetadata",
];

/// source: Service "@opencode/ACP/Session" — verbatim service id.
pub const SERVICE_ID: &str = "@opencode/ACP/Session";

/// source: node deps [] — verbatim.
pub const NODE_SERVICE: &str = SERVICE_ID;
pub const NODE_DEPS: &[&str] = &[];
