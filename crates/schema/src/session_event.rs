//! 1:1 port of packages/schema/src/session-event.ts
#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// NOTE(target uncertain, plan §4): crate::session_id::SessionID, crate::session_message::{ID, UnknownError}, crate::model::Ref, crate::prompt::Prompt, crate::session_delivery::Delivery, crate::schema_primitives::{DateTimeUtcFromMillis, NonNegativeInt, RelativePath}, crate::location::Ref, crate::revert::State, crate::llm::{ProviderMetadata, ToolContent}.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Source {
    pub start: i64,
    pub end: i64,
    pub text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Base {
    pub timestamp: crate::schema_primitives::DateTimeUtcFromMillis,
    pub sessionID: crate::session_id::SessionID,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PromptFields {
    pub timestamp: crate::schema_primitives::DateTimeUtcFromMillis,
    pub sessionID: crate::session_id::SessionID,
    pub messageID: crate::session_message::ID,
    pub prompt: crate::prompt::Prompt,
    pub delivery: crate::session_delivery::Delivery,
}
pub use crate::session_message::UnknownError;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentSwitched {
    #[serde(flatten)]
    pub base: Base,
    pub messageID: crate::session_message::ID,
    pub agent: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelSwitched {
    #[serde(flatten)]
    pub base: Base,
    pub messageID: crate::session_message::ID,
    pub model: crate::model::Ref,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Moved {
    #[serde(flatten)]
    pub base: Base,
    pub location: crate::location::Ref,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdirectory: Option<crate::schema_primitives::RelativePath>,
}
pub type Prompted = PromptFields;
pub type PromptAdmitted = PromptFields;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextUpdated {
    #[serde(flatten)]
    pub base: Base,
    pub messageID: crate::session_message::ID,
    pub text: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Synthetic {
    #[serde(flatten)]
    pub base: Base,
    pub messageID: crate::session_message::ID,
    pub text: String,
}
pub mod shell {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Started {
        #[serde(flatten)]
        pub base: super::Base,
        pub messageID: crate::session_message::ID,
        pub callID: String,
        pub command: String,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Ended {
        #[serde(flatten)]
        pub base: super::Base,
        pub callID: String,
        pub output: String,
    }
}
pub mod step {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Started {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub agent: String,
        pub model: crate::model::Ref,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<String>,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Tokens {
        pub input: f64,
        pub output: f64,
        pub reasoning: f64,
        pub cache: crate::session_event::AssistantTokensCacheShim,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Ended {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub finish: String,
        pub cost: f64,
        pub tokens: Tokens,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub snapshot: Option<String>,
        pub files: Vec<crate::schema_primitives::RelativePath>,
    }
    impl Ended {
        pub const TYPE: &'static str = "session.next.step.ended";
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Failed {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub error: super::UnknownError,
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssistantTokensCacheShim {
    pub read: f64,
    pub write: f64,
}
pub mod text {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Started {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub textID: String,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Delta {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub textID: String,
        pub delta: String,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Ended {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub textID: String,
        pub text: String,
    }
}
pub mod reasoning {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Started {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub reasoningID: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub providerMetadata: Option<crate::llm::ProviderMetadata>,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Delta {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub reasoningID: String,
        pub delta: String,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Ended {
        #[serde(flatten)]
        pub base: super::Base,
        pub assistantMessageID: crate::session_message::ID,
        pub reasoningID: String,
        pub text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub providerMetadata: Option<crate::llm::ProviderMetadata>,
    }
}
pub mod tool {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ToolBase {
        pub timestamp: crate::schema_primitives::DateTimeUtcFromMillis,
        pub sessionID: crate::session_id::SessionID,
        pub assistantMessageID: crate::session_message::ID,
        pub callID: String,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct ProviderDetail {
        pub executed: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub metadata: Option<crate::llm::ProviderMetadata>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resultMetadata: Option<crate::llm::ProviderMetadata>,
    }
    pub mod input {
        use serde::{Deserialize, Serialize};
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Started {
            #[serde(flatten)]
            pub base: super::ToolBase,
            pub name: String,
        }
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Delta {
            #[serde(flatten)]
            pub base: super::ToolBase,
            pub delta: String,
        }
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct Ended {
            #[serde(flatten)]
            pub base: super::ToolBase,
            pub text: String,
        }
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Called {
        #[serde(flatten)]
        pub base: ToolBase,
        pub tool: String,
        pub input: HashMap<String, serde_json::Value>,
        pub provider: ProviderDetail,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Progress {
        #[serde(flatten)]
        pub base: ToolBase,
        pub structured: HashMap<String, serde_json::Value>,
        pub content: Vec<crate::llm::ToolContent>,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Success {
        #[serde(flatten)]
        pub base: ToolBase,
        pub structured: HashMap<String, serde_json::Value>,
        pub content: Vec<crate::llm::ToolContent>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub outputPaths: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub result: Option<serde_json::Value>,
        pub provider: ProviderDetail,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Failed {
        #[serde(flatten)]
        pub base: ToolBase,
        pub error: super::UnknownError,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub result: Option<serde_json::Value>,
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetryError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statusCode: Option<f64>,
    pub isRetryable: bool,
    pub responseHeaders: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responseBody: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Retried {
    #[serde(flatten)]
    pub base: Base,
    pub attempt: f64,
    pub error: RetryError,
}
pub mod compaction {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Started {
        #[serde(flatten)]
        pub base: super::Base,
        pub messageID: crate::session_message::ID,
        pub reason: String,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Delta {
        #[serde(flatten)]
        pub base: super::Base,
        pub messageID: crate::session_message::ID,
        pub text: String,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Ended {
        #[serde(flatten)]
        pub base: super::Base,
        pub messageID: crate::session_message::ID,
        pub reason: String,
        pub text: String,
    }
}
pub mod revert {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Staged {
        #[serde(flatten)]
        pub base: super::Base,
        pub revert: crate::revert::State,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Cleared {
        #[serde(flatten)]
        pub base: super::Base,
    }
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Committed {
        #[serde(flatten)]
        pub base: super::Base,
        pub messageID: crate::session_message::ID,
    }
}
pub use revert as RevertEvent;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Durable {
    #[serde(rename = "session.next.agent.switched")]
    AgentSwitched(AgentSwitched),
    #[serde(rename = "session.next.model.switched")]
    ModelSwitched(ModelSwitched),
    #[serde(rename = "session.next.moved")]
    Moved(Moved),
    #[serde(rename = "session.next.prompted")]
    Prompted(Prompted),
    #[serde(rename = "session.next.prompt.admitted")]
    PromptAdmitted(PromptAdmitted),
    #[serde(rename = "session.next.context.updated")]
    ContextUpdated(ContextUpdated),
    #[serde(rename = "session.next.synthetic")]
    Synthetic(Synthetic),
    #[serde(rename = "session.next.shell.started")]
    ShellStarted(shell::Started),
    #[serde(rename = "session.next.shell.ended")]
    ShellEnded(shell::Ended),
    #[serde(rename = "session.next.step.started")]
    StepStarted(step::Started),
    #[serde(rename = "session.next.step.ended")]
    StepEnded(step::Ended),
    #[serde(rename = "session.next.step.failed")]
    StepFailed(step::Failed),
    #[serde(rename = "session.next.text.started")]
    TextStarted(text::Started),
    #[serde(rename = "session.next.text.ended")]
    TextEnded(text::Ended),
    #[serde(rename = "session.next.tool.input.started")]
    ToolInputStarted(tool::input::Started),
    #[serde(rename = "session.next.tool.input.ended")]
    ToolInputEnded(tool::input::Ended),
    #[serde(rename = "session.next.tool.called")]
    ToolCalled(tool::Called),
    #[serde(rename = "session.next.tool.progress")]
    ToolProgress(tool::Progress),
    #[serde(rename = "session.next.tool.success")]
    ToolSuccess(tool::Success),
    #[serde(rename = "session.next.tool.failed")]
    ToolFailed(tool::Failed),
    #[serde(rename = "session.next.reasoning.started")]
    ReasoningStarted(reasoning::Started),
    #[serde(rename = "session.next.reasoning.ended")]
    ReasoningEnded(reasoning::Ended),
    #[serde(rename = "session.next.retried")]
    Retried(Retried),
    #[serde(rename = "session.next.compaction.started")]
    CompactionStarted(compaction::Started),
    #[serde(rename = "session.next.compaction.ended")]
    CompactionEnded(compaction::Ended),
    #[serde(rename = "session.next.revert.staged")]
    RevertStaged(revert::Staged),
    #[serde(rename = "session.next.revert.cleared")]
    RevertCleared(revert::Cleared),
    #[serde(rename = "session.next.revert.committed")]
    RevertCommitted(revert::Committed),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum All {
    #[serde(rename = "session.next.agent.switched")]
    AgentSwitched(AgentSwitched),
    #[serde(rename = "session.next.model.switched")]
    ModelSwitched(ModelSwitched),
    #[serde(rename = "session.next.moved")]
    Moved(Moved),
    #[serde(rename = "session.next.prompted")]
    Prompted(Prompted),
    #[serde(rename = "session.next.prompt.admitted")]
    PromptAdmitted(PromptAdmitted),
    #[serde(rename = "session.next.context.updated")]
    ContextUpdated(ContextUpdated),
    #[serde(rename = "session.next.synthetic")]
    Synthetic(Synthetic),
    #[serde(rename = "session.next.shell.started")]
    ShellStarted(shell::Started),
    #[serde(rename = "session.next.shell.ended")]
    ShellEnded(shell::Ended),
    #[serde(rename = "session.next.step.started")]
    StepStarted(step::Started),
    #[serde(rename = "session.next.step.ended")]
    StepEnded(step::Ended),
    #[serde(rename = "session.next.step.failed")]
    StepFailed(step::Failed),
    #[serde(rename = "session.next.text.started")]
    TextStarted(text::Started),
    #[serde(rename = "session.next.text.delta")]
    TextDelta(text::Delta),
    #[serde(rename = "session.next.text.ended")]
    TextEnded(text::Ended),
    #[serde(rename = "session.next.reasoning.started")]
    ReasoningStarted(reasoning::Started),
    #[serde(rename = "session.next.reasoning.delta")]
    ReasoningDelta(reasoning::Delta),
    #[serde(rename = "session.next.reasoning.ended")]
    ReasoningEnded(reasoning::Ended),
    #[serde(rename = "session.next.tool.input.started")]
    ToolInputStarted(tool::input::Started),
    #[serde(rename = "session.next.tool.input.delta")]
    ToolInputDelta(tool::input::Delta),
    #[serde(rename = "session.next.tool.input.ended")]
    ToolInputEnded(tool::input::Ended),
    #[serde(rename = "session.next.tool.called")]
    ToolCalled(tool::Called),
    #[serde(rename = "session.next.tool.progress")]
    ToolProgress(tool::Progress),
    #[serde(rename = "session.next.tool.success")]
    ToolSuccess(tool::Success),
    #[serde(rename = "session.next.tool.failed")]
    ToolFailed(tool::Failed),
    #[serde(rename = "session.next.retried")]
    Retried(Retried),
    #[serde(rename = "session.next.compaction.started")]
    CompactionStarted(compaction::Started),
    #[serde(rename = "session.next.compaction.delta")]
    CompactionDelta(compaction::Delta),
    #[serde(rename = "session.next.compaction.ended")]
    CompactionEnded(compaction::Ended),
    #[serde(rename = "session.next.revert.staged")]
    RevertStaged(revert::Staged),
    #[serde(rename = "session.next.revert.cleared")]
    RevertCleared(revert::Cleared),
    #[serde(rename = "session.next.revert.committed")]
    RevertCommitted(revert::Committed),
}
pub type DurableEvent = Durable;
pub type Event = All;
pub type Type = String;

impl All {
    pub const Definitions: &[&'static str] = &[
        "session.next.agent.switched",
        "session.next.model.switched",
        "session.next.moved",
        "session.next.prompted",
        "session.next.prompt.admitted",
        "session.next.context.updated",
        "session.next.synthetic",
        "session.next.shell.started",
        "session.next.shell.ended",
        "session.next.step.started",
        "session.next.step.ended",
        "session.next.step.failed",
        "session.next.text.started",
        "session.next.text.delta",
        "session.next.text.ended",
        "session.next.reasoning.started",
        "session.next.reasoning.delta",
        "session.next.reasoning.ended",
        "session.next.tool.input.started",
        "session.next.tool.input.delta",
        "session.next.tool.input.ended",
        "session.next.tool.called",
        "session.next.tool.progress",
        "session.next.tool.success",
        "session.next.tool.failed",
        "session.next.retried",
        "session.next.compaction.started",
        "session.next.compaction.delta",
        "session.next.compaction.ended",
        "session.next.revert.staged",
        "session.next.revert.cleared",
        "session.next.revert.committed",
    ];
    pub const DurableDefinitions: &[crate::event_manifest::DurableEntry] = &[
        crate::event_manifest::DurableEntry {
            r#type: "session.next.agent.switched",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.model.switched",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.moved",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.prompted",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.prompt.admitted",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.context.updated",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.synthetic",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.shell.started",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.shell.ended",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.step.started",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.step.ended",
            aggregate: "sessionID",
            version: 2,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.step.failed",
            aggregate: "sessionID",
            version: 2,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.text.started",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.text.ended",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.tool.input.started",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.tool.input.ended",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.tool.called",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.tool.progress",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.tool.success",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.tool.failed",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.reasoning.started",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.reasoning.ended",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.retried",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.compaction.started",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.compaction.ended",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.revert.staged",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.revert.cleared",
            aggregate: "sessionID",
            version: 1,
        },
        crate::event_manifest::DurableEntry {
            r#type: "session.next.revert.committed",
            aggregate: "sessionID",
            version: 1,
        },
    ];
}
pub const Definitions: &[&str] = All::Definitions;
pub const DurableDefinitions: &[crate::event_manifest::DurableEntry] = All::DurableDefinitions;
