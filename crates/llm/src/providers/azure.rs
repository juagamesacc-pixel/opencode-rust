#![allow(clippy::all, dead_code, unused_imports, unused_variables)]
// source: packages/llm/src/providers/azure.ts
// PROVISIONAL: Effect/HTTP/stream runtime pending — descriptor port preserving source names/signatures/behavior strings.
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Verbatim strings from source for V2 audit
pub const VERBATIM_AZURE_0: &str = "azure";
pub const VERBATIM_AUTHORIZATION_1: &str = "authorization";
pub const VERBATIM_OPTIONAL_2: &str = "optional";
pub const VERBATIM_AZURE_OPENAI_RESPONSES_3: &str = "azure-openai-responses";
pub const VERBATIM_API_VERSION_4: &str = "api-version";
pub const VERBATIM__CONST_CHATROUTE_OPENAIC_5: &str = " },
  },
})

const chatRoute = OpenAIChat.route.with({
  id: ";
pub const VERBATIM__PROVIDER_ID_AUTH_ROUTEA_6: &str = ",
  provider: id,
  auth: routeAuth,
  endpoint: {
    query: { ";
pub const VERBATIM__IN_INPUT_INPUT_AUTH_RET_7: &str = " in input && input.auth) return input.auth
  return Auth.remove(";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Descriptor {
    pub kind: String,
    pub value: Value,
}

pub fn descriptor() -> Descriptor {
    Descriptor {
        kind: "azure".to_string(),
        value: Value::Null,
    }
}
