//! Rust port of `src/main/window-state.ts` (opencode v1.18.30).
//!
//! The source wraps every read in `try`/`catch` because the Electron
//! `WebContents` accessors can throw once the underlying contents are gone.
//! Rust methods cannot throw, so the `try`/`catch` is reproduced with
//! `std::panic::catch_unwind`: any failure (destroyed handle or panicking
//! accessor) yields `destroyedWindowURL`, exactly as in the source.
//!
//! Original file: `packages/desktop/src/main/window-state.ts`

use std::panic::{catch_unwind, AssertUnwindSafe};

pub const DESTROYED_WINDOW_URL: &str = "<destroyed>";

/// Mirrors `WebContentsURLState`.
pub trait WebContentsUrlState {
    fn is_destroyed(&self) -> bool;
    fn get_url(&self) -> String;
}

/// Mirrors `WindowURLState`.
pub trait WindowUrlState {
    type Contents: WebContentsUrlState;
    fn is_destroyed(&self) -> bool;
    fn web_contents(&self) -> &Self::Contents;
}

pub fn safe_web_contents_url(web_contents: &impl WebContentsUrlState) -> String {
    catch_unwind(AssertUnwindSafe(|| {
        if web_contents.is_destroyed() {
            return DESTROYED_WINDOW_URL.to_string();
        }
        web_contents.get_url()
    }))
    .unwrap_or_else(|_| DESTROYED_WINDOW_URL.to_string())
}

pub fn safe_window_url(win: &impl WindowUrlState) -> String {
    catch_unwind(AssertUnwindSafe(|| {
        if win.is_destroyed() {
            return DESTROYED_WINDOW_URL.to_string();
        }
        safe_web_contents_url(win.web_contents())
    }))
    .unwrap_or_else(|_| DESTROYED_WINDOW_URL.to_string())
}
