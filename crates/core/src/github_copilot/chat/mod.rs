//! Rust port of `packages/core/src/github-copilot/chat` barrel.
//! Source pin: v1.18.30 @ 3104c14 — 1:1 exact translation.

pub mod convert_to_openai_compatible_chat_messages;
pub mod get_response_metadata;
pub mod map_openai_compatible_finish_reason;
pub mod openai_compatible_api_types;
pub mod openai_compatible_chat_language_model;
pub mod openai_compatible_chat_options;
pub mod openai_compatible_metadata_extractor;
pub mod openai_compatible_prepare_tools;
