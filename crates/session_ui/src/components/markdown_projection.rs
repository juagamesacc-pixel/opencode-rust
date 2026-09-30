// source: packages/session-ui/src/components/markdown-projection.ts
// 1:1 port — pure projection helpers, fully ported as real Rust.

use crate::components::markdown_stream::{Block, BlockMode, Projection};

/// 1:1 port of `completedProjection(text)`.
pub fn completed_projection(text: &str) -> Projection {
    Projection {
        text: text.to_string(),
        blocks: vec![Block {
            raw: text.to_string(),
            src: text.to_string(),
            mode: BlockMode::Full,
            language: None,
            complete: None,
        }],
    }
}

/// 1:1 port of `canReusePendingBlock(current, next)`.
/// `current` is `Pick<Block, "mode" | "raw"> | undefined` in TS.
pub fn can_reuse_pending_block(current: Option<&Block>, next: &Block) -> bool {
    let Some(current) = current else {
        return false;
    };
    if current.mode != next.mode {
        return false;
    }
    if next.mode == BlockMode::Code || next.mode == BlockMode::Live {
        return next.raw.starts_with(&current.raw);
    }
    current.raw == next.raw
}
