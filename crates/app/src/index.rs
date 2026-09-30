//! Rust port of `packages/app/src/index.ts` (opencode v1.18.30).
//!
//! Source 30 lines: barrel re-exports (verbatim export list preserved below
//! as documentation; Rust re-exports resolve via `crate::` paths).
//! Source order:
//! - `./app`: `AppBaseProviders`, `AppInterface`
//! - `./context/layout`: `useLayout`
//! - `./context/server-sdk`: `useServerSDK`
//! - `./context/server-sync`: `useServerSync`
//! - `./context/server`: `useServer`, `ServerConnection`
//! - `./context/settings`: `useSettings`
//! - `./context/tabs`: `useTabs`
//! - `./hooks/use-providers`: `useProviders`
//! - `./constants/file-picker`: `ACCEPTED_FILE_EXTENSIONS`, `ACCEPTED_FILE_TYPES`, `filePickerFilters`
//! - `./context/command`: `useCommand`
//! - `./context/language`: `loadLocaleDict`, `normalizeLocale`, `Locale`, `useLanguage`
//! - `./wsl/context`: `useWslServers`
//! - `./context/platform`: `DisplayBackend`, `FatalRendererErrorLog`, `Platform`, `PlatformProvider`
//! - `./updater`: `UpdaterPlatform`, `UpdaterState`
//! - `./wsl/types`: `WslDistroProbe`, `WslInstalledDistro`, `WslJob`, `WslOnlineDistro`,
//!   `WslOpencodeCheck`, `WslRuntimeCheck`, `WslServerConfig`, `WslServerItem`,
//!   `WslServerRuntime`, `WslServersEvent`, `WslServersPlatform`, `WslServersState`
//! - `./utils/draft-store`: `createDraftStore`, `DraftStore`
//! Original file: `packages/app/src/index.ts`

#![allow(dead_code)]

// PROVISIONAL: barrel wiring for solid-js hooks is pending — mirrors `packages/app/src/index.ts`.
// Verified re-export (pure type, no reactive runtime).
pub use crate::updater::UpdaterState;
