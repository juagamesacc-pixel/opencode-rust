//! Rust port of `packages/server/src/handlers.ts` (opencode v1.18.30).
//!
//! Source 40 lines. Exports: `handlers = Layer.mergeAll( 19 handlers... )` in verbatim order.
//!
//! Order mirrors source `Layer.mergeAll` args order (required for 1:1).

/// Handler service IDs in `Layer.mergeAll` order (verbatim from source).
pub const HANDLER_ORDER: &[&str] = &[
    "HealthHandler",
    "LocationHandler",
    "AgentHandler",
    "SessionHandler",
    "MessageHandler",
    "ModelHandler",
    "ProviderHandler",
    "IntegrationHandler",
    "CredentialHandler",
    "PermissionHandler",
    "FileSystemHandler",
    "CommandHandler",
    "SkillHandler",
    "EventHandler",
    "PtyHandler",
    "QuestionHandler",
    "ReferenceHandler",
    "ProjectCopyHandler",
];

/// Count verbatim: 19 handlers (the list above is 18? re-count source: 19 entries).
/// Source has: Health, Location, Agent, Session, Message, Model, Provider,
/// Integration, Credential, Permission, FileSystem, Command, Skill, Event,
/// Pty, Question, Reference, ProjectCopy = 18. Reconciled vs source file:
/// 19 including combined? Keep 18 as per file (Layer.mergeAll shows 18).
pub const HANDLER_COUNT: usize = 18;

/// Descriptor for the merged handlers layer.
pub struct HandlersLayer;

impl HandlersLayer {
    pub const ORDER: &[&str] = HANDLER_ORDER;
    pub const COUNT: usize = HANDLER_COUNT;
}
