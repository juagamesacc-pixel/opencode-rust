//! Rust port of `src/audio.d.ts` (opencode v1.18.30).
//!
//! The declaration file teaches the bundler that `*.mp3` imports (and
//! specifically `@opencode-ai/ui/audio/*.mp3`) resolve to file paths. There
//! is no runtime behavior to port; this module records the declared patterns
//! so the sound asset references in [`crate::attention`] stay traceable.
//!
//! Original file: `packages/tui/src/audio.d.ts`

/// Mirrors `declare module "*.mp3"`.
pub const MP3_MODULE_PATTERN: &str = "*.mp3";

/// Mirrors `declare module "@opencode-ai/ui/audio/*.mp3"`.
pub const UI_AUDIO_MODULE_PATTERN: &str = "@opencode-ai/ui/audio/*.mp3";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declared_patterns_match_source() {
        assert_eq!(MP3_MODULE_PATTERN, "*.mp3");
        assert_eq!(UI_AUDIO_MODULE_PATTERN, "@opencode-ai/ui/audio/*.mp3");
    }
}
