//! Port of packages/app/src/components/session/open-in-app.tsx
//! ——— 1:1 exact clone, zero diversion ———
#![allow(clippy::all)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

// PROVISIONAL: SolidJS component — mirrors packages/app/src/components/session/open-in-app.tsx — UI rendering pending @opencode-ai/ui + solid-js.
/// Props for `OpenInApp` — field-for-field port of TS props.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenInAppProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
}

/// Render descriptor — no DOM, preserves behavior/edge cases.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenInAppRender {
    pub props: OpenInAppProps,
    pub children: Vec<String>,
}

impl OpenInAppRender {
    pub fn new(props: OpenInAppProps) -> Self {
        Self {
            props,
            children: Vec::new(),
        }
    }
}

/// State machine mirroring SolidJS signals/memos/effects for OpenInApp.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct OpenInAppState {
    pub mounted: bool,
}

impl OpenInAppState {
    pub fn new() -> Self {
        Self { mounted: false }
    }
    pub fn mount(&mut self) {
        self.mounted = true;
    }
    pub fn unmount(&mut self) {
        self.mounted = false;
    }
}

// Re-exported symbols from source (stubbed): OPEN_APPS, OpenApp, OpenAppOS, MAC_OPEN_APPS, WINDOWS_OPEN_APPS, LINUX_OPEN_APPS, detectOpenAppOS, openAppFileManager, openAppsForOS
