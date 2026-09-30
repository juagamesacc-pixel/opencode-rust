//! Rust port of `src/main/desktop-menu-actions.ts` (opencode v1.18.30).
//!
//! The action dispatch is ported in full: all nineteen `DesktopMenuAction`
//! cases, the `?.` no-window short-circuits, the `isMaximized()` toggle
//! branch, the `win?.webContents.getZoomFactor() ?? 1` reads for the two zoom
//! actions, and [`set_zoom`]'s `Math.min(Math.max(value, 0.2), 10)` clamp
//! followed by `updateTitlebar`.
//!
//! PROVISIONAL: `BrowserWindow` (Electron) has no in-workspace Rust binding,
//! so the window operations go through the [`MenuWindow`] trait and
//! [`crate::main::windows::create_main_window`] /
//! [`crate::main::windows::update_titlebar`] are PROVISIONAL.
//!
//! Original file: `packages/desktop/src/main/desktop-menu-actions.ts`

use crate::preload::types::DesktopMenuAction;

/// Mirrors the window operations the action switch performs.
pub trait MenuWindow {
    /// Stands in for the `WeakMap<BrowserWindow, ...>` keys in `windows.ts`
    /// (`titlebarThemes` / `windowIDs`): Rust cannot key a `WeakMap` through
    /// `&mut dyn`.
    fn handle_id(&self) -> String;
    fn close(&mut self);
    fn minimize(&mut self);
    fn is_maximized(&self) -> bool;
    fn maximize(&mut self);
    fn unmaximize(&mut self);
    fn reload(&mut self);
    fn web_contents_toggle_dev_tools(&mut self);
    fn get_zoom_factor(&self) -> f64;
    fn set_zoom_factor(&mut self, factor: f64);
    fn is_full_screen(&self) -> bool;
    fn set_full_screen(&mut self, fullscreen: bool);
    fn web_contents_undo(&mut self);
    fn web_contents_redo(&mut self);
    fn web_contents_cut(&mut self);
    fn web_contents_copy(&mut self);
    fn web_contents_paste(&mut self);
    fn web_contents_delete(&mut self);
    fn web_contents_select_all(&mut self);
}

/// Mirrors `type DesktopMenuActionHandlers`.
#[derive(Default)]
pub struct DesktopMenuActionHandlers {
    pub check_for_updates: Option<Box<dyn FnMut()>>,
    pub relaunch: Option<Box<dyn FnMut()>>,
}

/// Mirrors `function setZoom(win, value)`.
pub fn set_zoom(win: Option<&mut dyn MenuWindow>, value: f64) {
    let Some(win) = win else {
        return;
    };
    // `Math.min(Math.max(value, 0.2), 10)`
    win.set_zoom_factor(value.max(0.2).min(10.0));
    crate::main::windows::update_titlebar(win);
}

/// Mirrors `runDesktopMenuAction(win, action, handlers)`.
pub fn run_desktop_menu_action(
    win: Option<&mut dyn MenuWindow>,
    action: DesktopMenuAction,
    handlers: &mut DesktopMenuActionHandlers,
) {
    match action {
        DesktopMenuAction::AppCheckForUpdates => {
            if let Some(handler) = handlers.check_for_updates.as_mut() {
                handler();
            }
            return;
        }
        DesktopMenuAction::AppRelaunch => {
            if let Some(handler) = handlers.relaunch.as_mut() {
                handler();
            }
            return;
        }
        DesktopMenuAction::WindowNew => {
            // PROVISIONAL(packages/desktop/src/main/windows.ts):
            // `createMainWindow()` needs the Electron `BrowserWindow`.
            let _ = crate::main::windows::create_main_window();
            return;
        }
        DesktopMenuAction::WindowClose => {
            if let Some(win) = win {
                win.close();
            }
            return;
        }
        DesktopMenuAction::WindowMinimize => {
            if let Some(win) = win {
                win.minimize();
            }
            return;
        }
        DesktopMenuAction::WindowToggleMaximize => {
            if let Some(win) = win {
                if win.is_maximized() {
                    win.unmaximize();
                    return;
                }
                win.maximize();
            }
            return;
        }
        DesktopMenuAction::ViewReload => {
            if let Some(win) = win {
                win.reload();
            }
            return;
        }
        DesktopMenuAction::ViewToggleDevTools => {
            if let Some(win) = win {
                win.web_contents_toggle_dev_tools();
            }
            return;
        }
        DesktopMenuAction::ViewResetZoom => {
            set_zoom(win, 1.0);
            return;
        }
        DesktopMenuAction::ViewZoomIn => {
            let factor = win
                .as_deref()
                .map(|win| win.get_zoom_factor())
                .unwrap_or(1.0);
            set_zoom(win, factor + 0.2);
            return;
        }
        DesktopMenuAction::ViewZoomOut => {
            let factor = win
                .as_deref()
                .map(|win| win.get_zoom_factor())
                .unwrap_or(1.0);
            set_zoom(win, factor - 0.2);
            return;
        }
        DesktopMenuAction::ViewToggleFullscreen => {
            if let Some(win) = win {
                let next = !win.is_full_screen();
                win.set_full_screen(next);
            }
            return;
        }
        DesktopMenuAction::EditUndo => {
            if let Some(win) = win {
                win.web_contents_undo();
            }
            return;
        }
        DesktopMenuAction::EditRedo => {
            if let Some(win) = win {
                win.web_contents_redo();
            }
            return;
        }
        DesktopMenuAction::EditCut => {
            if let Some(win) = win {
                win.web_contents_cut();
            }
            return;
        }
        DesktopMenuAction::EditCopy => {
            if let Some(win) = win {
                win.web_contents_copy();
            }
            return;
        }
        DesktopMenuAction::EditPaste => {
            if let Some(win) = win {
                win.web_contents_paste();
            }
            return;
        }
        DesktopMenuAction::EditDelete => {
            if let Some(win) = win {
                win.web_contents_delete();
            }
            return;
        }
        DesktopMenuAction::EditSelectAll => {
            if let Some(win) = win {
                win.web_contents_select_all();
            }
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    // The source ships no `desktop-menu-actions.test.ts`; these cover the
    // 1:1 dispatch, the `?.` short-circuits and the zoom clamp.
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default)]
    struct Recorder {
        maximized: bool,
        full_screen: bool,
        zoom: f64,
        log: Vec<String>,
    }

    impl MenuWindow for Recorder {
        fn handle_id(&self) -> String {
            "test-window".to_string()
        }
        fn close(&mut self) {
            self.log.push("close".into());
        }
        fn minimize(&mut self) {
            self.log.push("minimize".into());
        }
        fn is_maximized(&self) -> bool {
            self.maximized
        }
        fn maximize(&mut self) {
            self.log.push("maximize".into());
            self.maximized = true;
        }
        fn unmaximize(&mut self) {
            self.log.push("unmaximize".into());
            self.maximized = false;
        }
        fn reload(&mut self) {
            self.log.push("reload".into());
        }
        fn web_contents_toggle_dev_tools(&mut self) {
            self.log.push("toggleDevTools".into());
        }
        fn get_zoom_factor(&self) -> f64 {
            self.zoom
        }
        fn set_zoom_factor(&mut self, factor: f64) {
            self.log.push(format!("zoom:{}", factor));
            self.zoom = factor;
        }
        fn is_full_screen(&self) -> bool {
            self.full_screen
        }
        fn set_full_screen(&mut self, fullscreen: bool) {
            self.log.push(format!("fullscreen:{}", fullscreen));
            self.full_screen = fullscreen;
        }
        fn web_contents_undo(&mut self) {
            self.log.push("undo".into());
        }
        fn web_contents_redo(&mut self) {
            self.log.push("redo".into());
        }
        fn web_contents_cut(&mut self) {
            self.log.push("cut".into());
        }
        fn web_contents_copy(&mut self) {
            self.log.push("copy".into());
        }
        fn web_contents_paste(&mut self) {
            self.log.push("paste".into());
        }
        fn web_contents_delete(&mut self) {
            self.log.push("delete".into());
        }
        fn web_contents_select_all(&mut self) {
            self.log.push("selectAll".into());
        }
    }

    /// Runs `action` against a fresh window and reports the handler counts.
    fn run(action: DesktopMenuAction) -> (Recorder, usize, usize) {
        let mut win = Recorder::default();
        let (checks, relaunches) = run_counting(action, &mut win);
        (win, checks, relaunches)
    }

    fn run_counting(action: DesktopMenuAction, win: &mut Recorder) -> (usize, usize) {
        let checks = Rc::new(RefCell::new(0usize));
        let relaunches = Rc::new(RefCell::new(0usize));
        let checks_sink = Rc::clone(&checks);
        let relaunch_sink = Rc::clone(&relaunches);
        let mut handlers = DesktopMenuActionHandlers {
            check_for_updates: Some(Box::new(move || *checks_sink.borrow_mut() += 1)),
            relaunch: Some(Box::new(move || *relaunch_sink.borrow_mut() += 1)),
        };
        run_desktop_menu_action(Some(win), action, &mut handlers);
        let counts = (*checks.borrow(), *relaunches.borrow());
        (counts.0, counts.1)
    }

    #[test]
    fn window_actions_reach_the_window() {
        for (action, expected) in [
            (DesktopMenuAction::WindowClose, "close"),
            (DesktopMenuAction::WindowMinimize, "minimize"),
            (DesktopMenuAction::ViewReload, "reload"),
            (DesktopMenuAction::ViewToggleDevTools, "toggleDevTools"),
            (DesktopMenuAction::EditUndo, "undo"),
            (DesktopMenuAction::EditRedo, "redo"),
            (DesktopMenuAction::EditCut, "cut"),
            (DesktopMenuAction::EditCopy, "copy"),
            (DesktopMenuAction::EditPaste, "paste"),
            (DesktopMenuAction::EditDelete, "delete"),
            (DesktopMenuAction::EditSelectAll, "selectAll"),
        ] {
            let (win, _, _) = run(action);
            assert_eq!(win.log, vec![expected.to_string()], "action {:?}", action);
        }
    }

    #[test]
    fn maximize_toggles_on_the_current_state() {
        let (win, _, _) = run(DesktopMenuAction::WindowToggleMaximize);
        assert_eq!(win.log, vec!["maximize".to_string()]);

        let mut win = Recorder {
            maximized: true,
            ..Recorder::default()
        };
        let mut handlers = DesktopMenuActionHandlers::default();
        run_desktop_menu_action(
            Some(&mut win),
            DesktopMenuAction::WindowToggleMaximize,
            &mut handlers,
        );
        assert_eq!(win.log, vec!["unmaximize".to_string()]);
    }

    #[test]
    fn zoom_actions_clamp_and_read_the_current_factor() {
        let (win, _, _) = run(DesktopMenuAction::ViewResetZoom);
        assert_eq!(win.log, vec!["zoom:1".to_string()]);

        let mut win = Recorder {
            zoom: 1.0,
            ..Recorder::default()
        };
        let mut handlers = DesktopMenuActionHandlers::default();
        run_desktop_menu_action(Some(&mut win), DesktopMenuAction::ViewZoomIn, &mut handlers);
        assert_eq!(win.zoom, 1.2);

        run_desktop_menu_action(
            Some(&mut win),
            DesktopMenuAction::ViewZoomOut,
            &mut handlers,
        );
        assert_eq!(win.zoom, 1.0);

        // The 0.2 floor holds.
        run_desktop_menu_action(
            Some(&mut win),
            DesktopMenuAction::ViewZoomOut,
            &mut handlers,
        );
        run_desktop_menu_action(
            Some(&mut win),
            DesktopMenuAction::ViewZoomOut,
            &mut handlers,
        );
        assert_eq!(win.zoom, 0.2);
    }

    #[test]
    fn fullscreen_toggles_the_current_state() {
        let (win, _, _) = run(DesktopMenuAction::ViewToggleFullscreen);
        assert_eq!(win.log, vec!["fullscreen:true".to_string()]);

        let mut win = Recorder {
            full_screen: true,
            ..Recorder::default()
        };
        let mut handlers = DesktopMenuActionHandlers::default();
        run_desktop_menu_action(
            Some(&mut win),
            DesktopMenuAction::ViewToggleFullscreen,
            &mut handlers,
        );
        assert_eq!(win.log, vec!["fullscreen:false".to_string()]);
    }

    #[test]
    fn a_null_window_is_a_no_op() {
        let mut handlers = DesktopMenuActionHandlers::default();
        for action in [
            DesktopMenuAction::WindowClose,
            DesktopMenuAction::WindowMinimize,
            DesktopMenuAction::WindowToggleMaximize,
            DesktopMenuAction::ViewReload,
            DesktopMenuAction::ViewToggleDevTools,
            DesktopMenuAction::ViewToggleFullscreen,
            DesktopMenuAction::EditSelectAll,
        ] {
            run_desktop_menu_action(None, action, &mut handlers);
        }
    }

    #[test]
    fn app_actions_call_their_own_handler() {
        let (_, checks, relaunches) = run(DesktopMenuAction::AppCheckForUpdates);
        assert_eq!((checks, relaunches), (1, 0));
        let (_, checks, relaunches) = run(DesktopMenuAction::AppRelaunch);
        assert_eq!((checks, relaunches), (0, 1));
    }
}
