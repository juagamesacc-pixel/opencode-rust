// source: packages/tui/src/component/prompt/ — barrel (mod.rs)
#![allow(dead_code)]
pub mod autocomplete;
pub mod cwd;
pub mod frecency;
pub mod history;
pub mod index;
pub mod local_attachment;
pub mod r#move;
pub mod stash;
pub mod workspace;

// Re-exports (source order)
pub use autocomplete::*;
pub use cwd::*;
pub use frecency::*;
pub use history::*;
pub use index::*;
pub use local_attachment::*;
pub use r#move::*;
pub use stash::*;
pub use workspace::*;
