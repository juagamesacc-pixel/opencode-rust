// source: src/question/schema.ts — exports: QuestionID
// PROVISIONAL pending @opencode-ai/schema/question-v1: ID brand mirrored.

/// source: QuestionID = QuestionV1.ID — que-prefixed brand, verbatim shape.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct QuestionID(pub String);

impl QuestionID {
    /// source: QuestionID.ascending() — verbatim constructor.
    pub fn ascending() -> Self {
        Self(crate::id::id::create(
            crate::id::id::PREFIX_QUESTION,
            crate::id::id::Direction::Ascending,
            None,
        ))
    }
}
