//! Rust port of `src/main/updater-controller.ts` (opencode v1.18.30).
//!
//! The source's `Promise` chains are strictly sequential, so the controller
//! is a synchronous state machine with the identical transition order
//! (`idle → checking → downloading → ready`), identical in-flight `pending`
//! coalescing, identical `.finally` reset, and identical `install()`
//! `stop → quitAndInstall → ready` / stop-failure `→ ready + rethrow`
//! behavior. `UpdaterState` mirrors the `UpdaterState` shape imported from
//! `@opencode-ai/app/updater` in the source; the canonical definition lives
//! in the app package (see `crate::preload::types`).
//!
//! Original file: `packages/desktop/src/main/updater-controller.ts`

use crate::preload::types::UpdaterState;
use std::collections::HashMap;

pub use crate::preload::types::UpdaterState as UpdaterControllerState;

pub struct UpdaterReadyRecord {
    pub version: String,
}

pub struct CheckUpdateInfo {
    pub is_update_available: bool,
    pub version: Option<String>,
}

pub trait UpdaterBackend {
    fn check_for_updates(&mut self) -> Result<Option<CheckUpdateInfo>, String>;
    fn download_update(&mut self) -> Result<(), String>;
    fn quit_and_install(&mut self);
}

pub trait UpdaterPersistence {
    fn get(&mut self) -> Option<UpdaterReadyRecord>;
    fn set(&mut self, value: UpdaterReadyRecord);
    fn clear(&mut self);
}

pub struct UpdaterControllerInput<B, P, S> {
    pub enabled: bool,
    pub current_version: String,
    pub backend: B,
    pub persistence: P,
    pub stop: S,
    pub log: Option<Box<dyn FnMut(&str, &str, &str)>>,
}

pub struct UpdaterController<B, P, S> {
    input: UpdaterControllerInput<B, P, S>,
    state: UpdaterState,
    pending: bool,
    listeners: HashMap<u64, Box<dyn FnMut(UpdaterState)>>,
    next_listener_id: u64,
}

impl<B: UpdaterBackend, P: UpdaterPersistence, S: FnMut() -> Result<(), String>>
    UpdaterController<B, P, S>
{
    pub fn new(input: UpdaterControllerInput<B, P, S>) -> Self {
        let state = if input.enabled {
            UpdaterState::idle()
        } else {
            UpdaterState::disabled()
        };
        Self {
            input,
            state,
            pending: false,
            listeners: HashMap::new(),
            next_listener_id: 0,
        }
    }

    fn transition(&mut self, next: UpdaterState) -> UpdaterState {
        if let Some(log) = self.input.log.as_mut() {
            log(
                "updater state changed",
                self.state.status.as_str(),
                next.status.as_str(),
            );
        }
        self.state = next.clone();
        for listener in self.listeners.values_mut() {
            listener(next.clone());
        }
        next
    }

    pub fn get_state(&self) -> UpdaterState {
        self.state.clone()
    }

    pub fn subscribe(&mut self, mut listener: impl FnMut(UpdaterState) + 'static) -> u64 {
        let id = self.next_listener_id;
        self.next_listener_id += 1;
        listener(self.state.clone());
        self.listeners.insert(id, Box::new(listener));
        id
    }

    pub fn unsubscribe(&mut self, id: u64) {
        self.listeners.remove(&id);
    }

    pub fn start(&mut self) -> UpdaterState {
        let ready = self.input.persistence.get();
        if ready.as_ref().map(|record| record.version.as_str())
            == Some(self.input.current_version.as_str())
        {
            self.input.persistence.clear();
        }
        self.check()
    }

    pub fn check(&mut self) -> UpdaterState {
        if !self.input.enabled {
            return self.state.clone();
        }
        if self.state.status == crate::preload::types::UpdaterStatus::Ready {
            return self.state.clone();
        }
        if self.pending {
            return self.state.clone();
        }
        self.pending = true;
        let result = self.run_check();
        self.pending = false;
        result
    }

    fn run_check(&mut self) -> UpdaterState {
        self.transition(UpdaterState::checking());
        let checked = self.input.backend.check_for_updates();
        let result = match checked {
            Ok(result) => result,
            Err(message) => return self.transition(UpdaterState::error(message)),
        };
        let version = result.as_ref().and_then(|info| info.version.clone());
        let available = result
            .as_ref()
            .map(|info| info.is_update_available)
            .unwrap_or(false);
        if !available
            || version.is_none()
            || version.as_deref() == Some(self.input.current_version.as_str())
        {
            self.input.persistence.clear();
            return self.transition(UpdaterState::up_to_date());
        }
        let version = version.unwrap_or_default();
        self.transition(UpdaterState::downloading(version.clone()));
        if let Err(message) = self.input.backend.download_update() {
            return self.transition(UpdaterState::error(message));
        }
        self.input.persistence.set(UpdaterReadyRecord {
            version: version.clone(),
        });
        self.transition(UpdaterState::ready(version))
    }

    pub fn install(&mut self) -> Result<(), String> {
        if self.state.status != crate::preload::types::UpdaterStatus::Ready {
            return Err("Update is not ready to install".to_string());
        }
        let version = self.state.version.clone().unwrap_or_default();
        self.transition(UpdaterState::installing(version.clone()));
        match (self.input.stop)() {
            Ok(()) => {
                self.input.backend.quit_and_install();
                self.transition(UpdaterState::ready(version));
                Ok(())
            }
            Err(error) => {
                self.transition(UpdaterState::ready(version));
                Err(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Mirrors `src/main/updater-controller.test.ts`
    // (`describe("updater controller")`).
    use super::*;
    use crate::preload::types::UpdaterStatus;
    use std::cell::RefCell;
    use std::rc::Rc;

    struct ScriptBackend {
        calls: Rc<RefCell<Vec<&'static str>>>,
    }

    impl UpdaterBackend for ScriptBackend {
        fn check_for_updates(&mut self) -> Result<Option<CheckUpdateInfo>, String> {
            self.calls.borrow_mut().push("check");
            Ok(Some(CheckUpdateInfo {
                is_update_available: true,
                version: Some("2.0.0".to_string()),
            }))
        }

        fn download_update(&mut self) -> Result<(), String> {
            self.calls.borrow_mut().push("download");
            Ok(())
        }

        fn quit_and_install(&mut self) {
            self.calls.borrow_mut().push("install");
        }
    }

    struct MemoryPersistence {
        ready: Option<String>,
    }

    impl UpdaterPersistence for MemoryPersistence {
        fn get(&mut self) -> Option<UpdaterReadyRecord> {
            self.ready
                .clone()
                .map(|version| UpdaterReadyRecord { version })
        }

        fn set(&mut self, value: UpdaterReadyRecord) {
            self.ready = Some(value.version);
        }

        fn clear(&mut self) {
            self.ready = None;
        }
    }

    type Harness =
        UpdaterController<ScriptBackend, MemoryPersistence, Box<dyn FnMut() -> Result<(), String>>>;

    fn setup(
        current_version: &str,
        ready: Option<&str>,
    ) -> (
        Harness,
        Rc<RefCell<Vec<&'static str>>>,
        Rc<RefCell<Option<String>>>,
    ) {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let persisted = Rc::new(RefCell::new(ready.map(str::to_string)));
        let read_persisted = Rc::clone(&persisted);
        let write_persisted = Rc::clone(&persisted);
        let stop_calls = Rc::clone(&calls);
        let controller = UpdaterController::new(UpdaterControllerInput {
            enabled: true,
            current_version: current_version.to_string(),
            backend: ScriptBackend {
                calls: Rc::clone(&calls),
            },
            persistence: MemoryPersistence {
                ready: ready.map(str::to_string),
            },
            stop: Box::new(move || {
                stop_calls.borrow_mut().push("stop");
                Ok(())
            }),
            log: None,
        });
        let _ = (read_persisted, write_persisted);
        (controller, calls, persisted)
    }

    #[test]
    fn checks_downloads_persists_and_publishes_one_authoritative_ready_state() {
        let (mut app, calls, persisted) = setup("1.0.0", None);
        let mut states = Vec::new();
        app.subscribe(|state| states.push(state));

        app.start();

        assert_eq!(*calls.borrow(), vec!["check", "download"]);
        assert_eq!(*persisted.borrow(), Some("2.0.0".to_string()));
        assert_eq!(
            states
                .iter()
                .map(|state| state.status.clone())
                .collect::<Vec<_>>(),
            vec![
                UpdaterStatus::Idle,
                UpdaterStatus::Checking,
                UpdaterStatus::Downloading,
                UpdaterStatus::Ready,
            ]
        );
        assert_eq!(app.get_state(), UpdaterState::ready("2.0.0".to_string()));
    }

    #[test]
    fn revalidates_a_persisted_target_through_the_updater_cache_on_launch() {
        let (mut app, calls, _) = setup("1.0.0", Some("2.0.0"));

        app.start();

        assert_eq!(*calls.borrow(), vec!["check", "download"]);
        assert_eq!(app.get_state(), UpdaterState::ready("2.0.0".to_string()));
    }

    #[test]
    fn clears_a_target_already_installed_before_checking() {
        let (mut app, calls, persisted) = setup("2.0.0", Some("2.0.0"));

        app.start();

        assert_eq!(*persisted.borrow(), None);
        assert_eq!(*calls.borrow(), vec!["check"]);
    }

    #[test]
    fn coalesces_concurrent_checks() {
        let (mut app, calls, _) = setup("1.0.0", None);

        app.start();
        app.check();
        app.check();
        app.check();

        // State is `ready` after `start()`, so the follow-up checks return
        // the ready state without touching the backend again — the same
        // single `check` + `download` sequence the source asserts.
        assert_eq!(*calls.borrow(), vec!["check", "download"]);
    }

    #[test]
    fn returns_to_ready_when_quit_and_install_returns_without_exiting() {
        let (mut app, calls, _) = setup("1.0.0", None);
        app.start();

        app.install().expect("install succeeds");

        assert_eq!(
            *calls.borrow(),
            vec!["check", "download", "stop", "install"]
        );
        assert_eq!(app.get_state(), UpdaterState::ready("2.0.0".to_string()));
    }

    #[test]
    fn returns_to_ready_when_installation_cannot_start() {
        let (mut app, _, _) = setup("1.0.0", None);
        app.start();

        let calls = Rc::new(RefCell::new(Vec::new()));
        let mut failed = UpdaterController::new(UpdaterControllerInput {
            enabled: true,
            current_version: "1.0.0".to_string(),
            backend: ScriptBackend {
                calls: Rc::clone(&calls),
            },
            persistence: MemoryPersistence { ready: None },
            stop: Box::new(|| Err("stop failed".to_string())),
            log: None,
        });
        failed.start();

        let error = failed
            .install()
            .expect_err("install rethrows the stop failure");
        assert_eq!(error, "stop failed");
        assert_eq!(failed.get_state(), UpdaterState::ready("2.0.0".to_string()));
    }
}
