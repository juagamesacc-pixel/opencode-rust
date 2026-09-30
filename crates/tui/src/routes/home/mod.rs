// source: packages/tui/src/routes/home.tsx + routes/home/ (v1.18.30) — merged barrel
// home.tsx is parent module that also has submodule home/session-destination.tsx
#![allow(dead_code)]
#![allow(unused_imports)]

pub mod screen;
pub mod session_destination;

// Home route (from routes/home.tsx)
pub const HOME_ROUTE_TYPE: &str = "home";

// Stub for Home component
struct HomeStub;
impl HomeStub {
    pub fn new() -> Self {
        Self
    }
    pub fn update(&mut self) {}
}

// Original home.tsx stub retained:
// Stub — preserves export names/order; full logic wired via ratatui + tokio where applicable.
// Original TS exports (first 5): import { Prompt, type PromptRef } from "../component/prompt" import { createEffect, createMemo, createSignal, onMount } from "solid-js" import { Logo } from "../component/logo" import { useSync } from
struct StubHome;
impl StubHome {
    pub fn new() -> Self {
        Self
    }
    pub fn update(&mut self) {}
}

pub use session_destination::*;
