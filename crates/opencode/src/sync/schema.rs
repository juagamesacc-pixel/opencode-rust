// source: src/sync/schema.ts — exports: EventID
// PROVISIONAL pending crates/core (schema statics): brand + ascending ctor.

/// source: EventID — "evt"-prefixed brand, Schema.String.check(isStartsWith("evt")).
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct EventID(pub String);

impl EventID {
    /// source: EventID.ascending(id?) — Identifier.ascending("event", id), verbatim.
    pub fn ascending(given: Option<&str>) -> Result<Self, String> {
        Ok(Self(crate::id::id::ascending(
            crate::id::id::Prefix::Event,
            given,
        )?))
    }
}
