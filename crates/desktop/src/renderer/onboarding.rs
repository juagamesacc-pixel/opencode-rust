//! Rust port of `src/renderer/onboarding.tsx` (opencode v1.18.30).
//!
//! `DesktopFirstLaunchOnboarding` needs the Solid runtime plus the app
//! package's server/settings/tabs contexts — PROVISIONAL here. Fully
//! ported: the `shouldTrigger` decision and the three
//! `[desktop-onboarding] …` log strings (byte-identical).
//!
//! Original file: `packages/desktop/src/renderer/onboarding.tsx`

pub const LOG_EVALUATED: &str = "[desktop-onboarding] first launch onboarding evaluated";
pub const LOG_STARTING_DRAFT: &str = "[desktop-onboarding] starting first launch draft";
pub const LOG_FAILED: &str = "[desktop-onboarding] first launch onboarding failed";

/// Mirrors the `shouldTrigger` computation in `runFirstLaunchOnboarding`:
/// `!existingInstall && initialUrl === "/" && tabs empty && every server
/// builtin`.
pub fn should_trigger_first_launch(
    existing_install: bool,
    initial_url: &str,
    tab_count: usize,
    all_servers_builtin: bool,
) -> bool {
    !existing_install && initial_url == "/" && tab_count == 0 && all_servers_builtin
}

// PROVISIONAL(packages/desktop/src/renderer/onboarding.tsx):
// `DesktopFirstLaunchOnboarding({ initialUrl, onLoaded })` — readiness
// gating (`server.ready`/`tabs.ready`/`tabs.recentReady`),
// `window.api.isOldLayoutEligible()`,
// `settings.general.setOldLayoutEligible/initializeAgentVisibility`,
// `server.isLocal()`, `window.api.isFirstLaunchOnboardingPending()`,
// `window.api.finishFirstLaunchOnboarding(shouldTrigger)`, and the draft
// creation (`server.projects.open/touch`, `tabs.newDraft`, `tabs.select`).
pub fn run_first_launch_onboarding(_initial_url: &str) {
    unimplemented!("Solid runtime + app server/settings/tabs context binding")
}

#[cfg(test)]
mod tests {
    // No `src/renderer/onboarding.test.ts` exists in the source; the case
    // below pins the ported trigger decision.
    use super::*;

    #[test]
    fn trigger_requires_fresh_first_launch_state() {
        assert!(should_trigger_first_launch(false, "/", 0, true));
        assert!(!should_trigger_first_launch(true, "/", 0, true));
        assert!(!should_trigger_first_launch(false, "/session/1", 0, true));
        assert!(!should_trigger_first_launch(false, "/", 2, true));
        assert!(!should_trigger_first_launch(false, "/", 0, false));
    }
}
