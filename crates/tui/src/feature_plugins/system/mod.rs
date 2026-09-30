// source: packages/tui/src/feature_plugins/system/ — barrel (mod.rs)
#![allow(dead_code)]
pub mod diff_viewer;
pub mod diff_viewer_file_tree;
pub mod diff_viewer_file_tree_utils;
pub mod diff_viewer_ui;
pub mod notifications;
pub mod plugins;
pub mod which_key;

// Re-exports (source order)
pub use diff_viewer::*;
pub use diff_viewer_file_tree::*;
pub use diff_viewer_file_tree_utils::*;
pub use diff_viewer_ui::*;
pub use notifications::*;
pub use plugins::*;
pub use which_key::*;
