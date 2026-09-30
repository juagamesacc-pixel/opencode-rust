//! Rust port of `src/main/unresponsive.ts` (opencode v1.18.30).
//!
//! Portable logic is ported 1:1: the `active()` guard, the sample tally
//! (insertion-ordered like the source's JS `Map`, so the stable descending
//! sort breaks ties in first-seen order), the `stopAndFlush` return value, and
//! the `renderer unresponsive samples` report text.
//!
//! The two `setTimeout` calls and the `closed` subscription stay the source's
//! control flow, but firing them needs a JS-style timer plus CDP, so the
//! [`UnresponsiveSampler`] surface here is the pull-based core that
//! [`schedule`]/[`start`] drive:
//! [`UnresponsiveSampler::collect_sample`] is `collect()` and
//! [`UnresponsiveSampler::stop_and_flush`] is `stopAndFlush()`.
//!
//! PROVISIONAL(packages/desktop/src/main/unresponsive.ts):
//! `win.webContents.mainFrame.collectJavaScriptCallStack()` (CDP),
//! `webContents.isDevToolsOpened()`, the `sampleTimer`/`stopTimer` scheduling
//! and the `win.on("closed", stopAndFlush)` subscription.
//!
//! Original file: `packages/desktop/src/main/unresponsive.ts`

use crate::main::window_state::WindowUrlState;

/// `const sampleInterval = 1000`
pub const SAMPLE_INTERVAL_MS: u64 = 1000;
/// `const samplePeriod = 15000`
pub const SAMPLE_PERIOD_MS: u64 = 15000;

/// The window surface `createUnresponsiveSampler` uses.
pub trait UnresponsiveWindow: WindowUrlState {
    /// PROVISIONAL: `win.webContents.isDevToolsOpened()`.
    fn is_dev_tools_opened(&self) -> bool;
    /// `await win.webContents.mainFrame.collectJavaScriptCallStack()`.
    ///
    /// PROVISIONAL: CDP has no in-workspace Rust binding. `Err` is the
    /// `.catch()` arm, which logs and reports "no stack".
    fn collect_javascript_call_stack(&self) -> Result<String, String>;
}

/// The report body `stopAndFlush` logs.
pub fn format_message(name: &str, url: &str, entries: &[(String, usize)]) -> String {
    let total: usize = entries.iter().map(|entry| entry.1).sum();
    let mut lines = vec![
        "renderer unresponsive samples".to_string(),
        format!("Window: {name}"),
        format!("URL: {url}"),
    ];
    lines.extend(
        entries
            .iter()
            .map(|entry| format!("<{}> {}", entry.1, entry.0)),
    );
    lines.push(format!("Total Samples: {total}"));
    lines.join("\n")
}

/// `createUnresponsiveSampler(win, name)`'s state and its portable methods.
pub struct UnresponsiveSampler<W: UnresponsiveWindow> {
    win: W,
    name: String,
    sampling: bool,
    /// `new Map<string, number>()` — a `Vec` keeps JS `Map` insertion order so
    /// the stable sort in `stop_and_flush` breaks ties the same way.
    samples: Vec<(String, usize)>,
}

impl<W: UnresponsiveWindow> UnresponsiveSampler<W> {
    /// `createUnresponsiveSampler(win, name)`.
    ///
    /// The source subscribes `win.on("closed", stopAndFlush)` here;
    /// PROVISIONAL, see the module docs.
    pub fn new(win: W, name: String) -> Self {
        Self {
            win,
            name,
            sampling: false,
            samples: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_sampling(&self) -> bool {
        self.sampling
    }

    /// `const active = () => sampling && !win.isDestroyed() && !win.webContents.isDestroyed()`
    pub fn active(&self) -> bool {
        self.sampling && !self.win.is_destroyed() && !self.win.web_contents().is_destroyed()
    }

    /// The `.catch()` half of `collect()`.
    ///
    /// PROVISIONAL(packages/desktop/src/main/logging.ts):
    /// `writeLog("window", "failed to collect unresponsive sample", { window: name, error }, "error")`.
    pub fn report_collect_failure(&self, error: &str) {
        let _ = (self.name.as_str(), error);
    }

    /// `const collect = async () => { … }` — the stack tally.
    ///
    /// The source re-checks `active()` after the `await` and then reschedules;
    /// the reschedule is the caller's (`SAMPLE_INTERVAL_MS`).
    pub fn collect_sample(&mut self, stack: Option<String>) {
        if !self.active() {
            return;
        }
        let Some(stack) = stack else {
            return;
        };
        match self.samples.iter_mut().find(|entry| entry.0 == stack) {
            Some(entry) => entry.1 += 1,
            None => self.samples.push((stack, 1)),
        }
    }

    /// `const stopAndFlush = () => { … }`
    ///
    /// Returns the `wasSampling` value the source returns, and the report body
    /// it logged (`None` when there was nothing to flush).
    pub fn stop_and_flush(&mut self) -> (bool, Option<String>) {
        let was_sampling = self.sampling;
        self.sampling = false;
        if self.samples.is_empty() {
            return (was_sampling, None);
        }
        // `[...samples.entries()].sort((a, b) => b[1] - a[1])`
        self.samples.sort_by(|a, b| b.1.cmp(&a.1));
        let message = format_message(
            &self.name,
            &crate::main::window_state::safe_window_url(&self.win),
            &self.samples,
        );
        // PROVISIONAL(packages/desktop/src/main/logging.ts):
        // `writeLog("window", message, undefined, "error")`.
        self.samples.clear();
        (was_sampling, Some(message))
    }

    /// `const start = () => { … }`
    ///
    /// Returns `true` when sampling started, matching the source's early
    /// `return`. The `schedule()` and `stopTimer` calls are PROVISIONAL.
    pub fn start(&mut self) -> bool {
        if self.sampling
            || self.win.is_destroyed()
            || self.win.web_contents().is_destroyed()
            || self.win.is_dev_tools_opened()
        {
            return false;
        }
        self.sampling = true;
        self.samples.clear();
        // PROVISIONAL: `schedule()` arms `sampleTimer` and
        // `stopTimer = setTimeout(stopAndFlush, samplePeriod)`.
        true
    }
}

#[cfg(test)]
mod tests {
    // The source ships no `unresponsive.test.ts`; these cover the sample tally,
    // the tie-order-preserving sort and the report text.
    use super::*;
    use crate::main::window_state::WebContentsUrlState;

    struct Contents {
        destroyed: bool,
    }

    impl WebContentsUrlState for Contents {
        fn is_destroyed(&self) -> bool {
            self.destroyed
        }
        fn get_url(&self) -> String {
            "oc://renderer/index.html".to_string()
        }
    }

    struct Win {
        destroyed: bool,
        contents: Contents,
        dev_tools: bool,
    }

    impl WindowUrlState for Win {
        type Contents = Contents;
        fn is_destroyed(&self) -> bool {
            self.destroyed
        }
        fn web_contents(&self) -> &Contents {
            &self.contents
        }
    }

    impl UnresponsiveWindow for Win {
        fn is_dev_tools_opened(&self) -> bool {
            self.dev_tools
        }
        fn collect_javascript_call_stack(&self) -> Result<String, String> {
            Ok("at foo".to_string())
        }
    }

    fn win() -> Win {
        Win {
            destroyed: false,
            contents: Contents { destroyed: false },
            dev_tools: false,
        }
    }

    #[test]
    fn start_requires_a_live_window_without_dev_tools() {
        let mut sampler = UnresponsiveSampler::new(win(), "main".to_string());
        assert!(sampler.start());
        // Already sampling.
        assert!(!sampler.start());
        assert!(sampler.is_sampling());

        let mut dev = UnresponsiveSampler::new(
            Win {
                dev_tools: true,
                ..win()
            },
            "main".to_string(),
        );
        assert!(!dev.start());
        assert!(!dev.is_sampling());

        let mut gone = UnresponsiveSampler::new(
            Win {
                destroyed: true,
                ..win()
            },
            "main".to_string(),
        );
        assert!(!gone.start());
    }

    #[test]
    fn samples_only_accumulate_while_sampling_and_are_flushed_once() {
        let mut sampler = UnresponsiveSampler::new(win(), "main".to_string());
        // Not sampling yet, so `collect()` returns early.
        sampler.collect_sample(Some("at foo".to_string()));
        assert_eq!(sampler.stop_and_flush(), (false, None));

        assert!(sampler.start());
        sampler.collect_sample(Some("at foo".to_string()));
        sampler.collect_sample(Some("at bar".to_string()));
        sampler.collect_sample(Some("at foo".to_string()));
        let (was_sampling, message) = sampler.stop_and_flush();
        assert!(was_sampling);
        let message = message.expect("a report");
        // `at foo` has 2 samples and `at bar` has 1, so the count order puts
        // `at foo` first regardless of insertion order.
        assert_eq!(
            message,
            concat!(
                "renderer unresponsive samples\n",
                "Window: main\n",
                "URL: oc://renderer/index.html\n",
                "<2> at foo\n",
                "<1> at bar\n",
                "Total Samples: 3",
            )
        );
        // The map is cleared, so a second flush is silent.
        assert_eq!(sampler.stop_and_flush(), (false, None));
    }

    #[test]
    fn a_destroyed_contents_stops_the_sampler() {
        let mut sampler = UnresponsiveSampler::new(
            Win {
                contents: Contents { destroyed: true },
                ..win()
            },
            "main".to_string(),
        );
        assert!(!sampler.start());
        assert!(!sampler.active());
    }
}
