// source: packages/tui/src/util/signal.ts (51 lines, v1.18.30)
// 1:1 port — SolidJS signals become explicit state machines driven by the
// render loop; the debounced setter parks a tokio task like `setTimeout`.

#![allow(dead_code)]

use std::time::Duration;
use tokio::task::JoinHandle;

/// Fade timing verbatim: 160ms duration, 16ms steps.
pub const FADE_DURATION_MS: u64 = 160;
pub const FADE_STEP_MS: u64 = 16;

/// Mirrors `createDebouncedSignal` — latest value wins; the pending timer
/// is cancelled on every `set` (and on `cancel`, mirroring `onCleanup`).
pub struct Debounced<T: Clone + Send + 'static> {
    current: T,
    ms: u64,
    timer: Option<JoinHandle<()>>,
}

impl<T: Clone + Send + 'static> Debounced<T> {
    pub fn new(value: T, ms: u64) -> Self {
        Self {
            current: value,
            ms,
            timer: None,
        }
    }

    pub fn get(&self) -> &T {
        &self.current
    }

    pub fn set<F>(&mut self, next: T, apply: F)
    where
        F: FnOnce(T) + Send + 'static,
    {
        if let Some(handle) = self.timer.take() {
            handle.abort();
        }
        let ms = self.ms;
        self.timer = Some(tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(ms)).await;
            apply(next);
        }));
    }

    /// Mirrors the `onCleanup` timer clear.
    pub fn cancel(&mut self) {
        if let Some(handle) = self.timer.take() {
            handle.abort();
        }
    }
}

/// Mirrors `createFadeIn` — alpha state machine with the verbatim
/// smootherstep curve `p*p*(3-2p)` over 160ms.
#[derive(Debug, Clone)]
pub struct FadeIn {
    pub alpha: f64,
    revealed: bool,
    start_ms: Option<u64>,
}

impl FadeIn {
    pub fn new(visible: bool) -> Self {
        Self {
            alpha: if visible { 1.0 } else { 0.0 },
            revealed: visible,
            start_ms: None,
        }
    }

    /// Advance one frame; returns the current alpha.
    pub fn tick(&mut self, visible: bool, animate: bool, now_ms: u64) -> f64 {
        if !visible {
            self.alpha = 0.0;
            self.start_ms = None;
            return self.alpha;
        }
        if !animate || self.revealed {
            self.revealed = true;
            self.alpha = 1.0;
            self.start_ms = None;
            return self.alpha;
        }
        let start = match self.start_ms {
            Some(start) => start,
            None => {
                self.revealed = true;
                self.alpha = 0.0;
                self.start_ms = Some(now_ms);
                now_ms
            }
        };
        let progress = ((now_ms.saturating_sub(start)) as f64 / FADE_DURATION_MS as f64).min(1.0);
        self.alpha = progress * progress * (3.0 - 2.0 * progress);
        self.alpha
    }
}
