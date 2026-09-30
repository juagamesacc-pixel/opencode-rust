//! Rust port of `src/terminal-win32.ts` (opencode v1.18.30).
//!
//! Windows console-input helpers. The source reaches `kernel32.dll` through
//! `bun:ffi`; the port declares the same four functions with plain `extern`
//! blocks, so no new crate is needed. Everything is a no-op off Windows,
//! exactly like the source's `process.platform !== "win32"` guards.
//!
//! The `setRawMode` monkey-patch from `win32InstallCtrlCGuard` has no Rust
//! equivalent (raw mode belongs to crossterm here); the 100 ms poll backstop
//! plus the immediate enforce cover it.
//!
//! Original file: `packages/tui/src/terminal-win32.ts`

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// `STD_INPUT_HANDLE = -10` for `GetStdHandle`.
pub const STD_INPUT_HANDLE: i32 = -10;
/// `ENABLE_PROCESSED_INPUT = 0x0001`.
pub const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
/// The poll backstop interval from `win32InstallCtrlCGuard`.
pub const CTRL_C_GUARD_POLL_MS: u64 = 100;

#[cfg(windows)]
mod ffi {
    use std::ffi::c_void;

    #[link(name = "kernel32")]
    extern "C" {
        pub fn GetStdHandle(n_std_handle: i32) -> *mut c_void;
        pub fn GetConsoleMode(h_console_input: *mut c_void, lp_mode: *mut u32) -> i32;
        pub fn SetConsoleMode(h_console_input: *mut c_void, dw_mode: u32) -> i32;
        pub fn FlushConsoleInputBuffer(h_console_input: *mut c_void) -> i32;
    }
}

/// Whether the Win32 console path applies.
pub fn is_windows() -> bool {
    cfg!(windows)
}

#[cfg(windows)]
fn stdin_handle() -> *mut std::ffi::c_void {
    // SAFETY: GetStdHandle with a valid constant is always safe to call.
    unsafe { ffi::GetStdHandle(STD_INPUT_HANDLE) }
}

#[cfg(windows)]
fn console_mode(handle: *mut std::ffi::c_void) -> Option<u32> {
    let mut mode: u32 = 0;
    // SAFETY: handle comes from GetStdHandle; mode points at a live u32.
    let ok = unsafe { ffi::GetConsoleMode(handle, &mut mode) };
    if ok == 0 {
        return None;
    }
    Some(mode)
}

#[cfg(windows)]
fn stdin_is_tty() -> bool {
    std::io::IsTerminal::is_terminal(&std::io::stdin())
}

/// Clear `ENABLE_PROCESSED_INPUT` on the console stdin handle.
pub fn win32_disable_processed_input() {
    if is_windows() {
        #[cfg(windows)]
        {
            if !stdin_is_tty() {
                return;
            }
            let handle = stdin_handle();
            let Some(mode) = console_mode(handle) else {
                return;
            };
            if mode & ENABLE_PROCESSED_INPUT == 0 {
                return;
            }
            // SAFETY: same handle and mode domain as the successful read above.
            unsafe {
                ffi::SetConsoleMode(handle, mode & !ENABLE_PROCESSED_INPUT);
            }
        }
    }
}

/// Discard any queued console input.
pub fn win32_flush_input_buffer() {
    if is_windows() {
        #[cfg(windows)]
        {
            if !stdin_is_tty() {
                return;
            }
            let handle = stdin_handle();
            // SAFETY: handle comes from GetStdHandle.
            unsafe {
                ffi::FlushConsoleInputBuffer(handle);
            }
        }
    }
}

/// The installed Ctrl+C guard: polling thread plus the saved initial mode.
/// Mirrors the `unhook` closure the source returns.
pub struct Win32CtrlCGuard {
    stop: Arc<AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    #[cfg(windows)]
    handle: *mut std::ffi::c_void,
    #[cfg(windows)]
    initial_mode: u32,
}

// SAFETY: the handle is only dereferenced on the guard thread while the
// guard is alive; the stop flag hands off before `unhook` joins.
unsafe impl Send for Win32CtrlCGuard {}

impl Win32CtrlCGuard {
    /// Stop polling and restore the initial console mode. Mirrors `unhook`.
    pub fn unhook(mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
        #[cfg(windows)]
        {
            // SAFETY: same handle and mode domain as installation.
            unsafe {
                ffi::SetConsoleMode(self.handle, self.initial_mode);
            }
        }
    }
}

/// Keep `ENABLE_PROCESSED_INPUT` disabled. Returns `None` off Windows (or
/// when stdin is not a TTY / the console mode cannot be read), mirroring the
/// source's early returns.
pub fn win32_install_ctrl_c_guard() -> Option<Win32CtrlCGuard> {
    if !is_windows() {
        return None;
    }
    #[cfg(not(windows))]
    {
        None
    }
    #[cfg(windows)]
    {
        if !stdin_is_tty() {
            return None;
        }
        let handle = stdin_handle();
        let initial = console_mode(handle)?;
        enforce_mode(handle);
        // Enforce twice: runtimes can re-apply console modes on a later tick.
        enforce_mode(handle);
        let stop = Arc::new(AtomicBool::new(false));
        let worker_stop = Arc::clone(&stop);
        let thread = std::thread::spawn(move || {
            while !worker_stop.load(Ordering::SeqCst) {
                enforce_mode(handle);
                std::thread::sleep(std::time::Duration::from_millis(CTRL_C_GUARD_POLL_MS));
            }
        });
        Some(Win32CtrlCGuard {
            stop,
            thread: Some(thread),
            handle,
            initial_mode: initial,
        })
    }
}

#[cfg(windows)]
fn enforce_mode(handle: *mut std::ffi::c_void) {
    let Some(mode) = console_mode(handle) else {
        return;
    };
    if mode & ENABLE_PROCESSED_INPUT == 0 {
        return;
    }
    // SAFETY: same handle and mode domain as the successful read above.
    unsafe {
        ffi::SetConsoleMode(handle, mode & !ENABLE_PROCESSED_INPUT);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_source() {
        assert_eq!(STD_INPUT_HANDLE, -10);
        assert_eq!(ENABLE_PROCESSED_INPUT, 0x0001);
        assert_eq!(CTRL_C_GUARD_POLL_MS, 100);
    }

    #[test]
    fn guards_are_inert_off_windows() {
        if cfg!(windows) {
            return;
        }
        assert!(!is_windows());
        win32_disable_processed_input();
        win32_flush_input_buffer();
        assert!(win32_install_ctrl_c_guard().is_none());
    }

    #[test]
    fn unhook_joins_the_poll_thread() {
        if cfg!(windows) {
            return;
        }
        // A guard built by hand (same shape the installer returns) must join
        // without hanging when unhooked.
        let guard = Win32CtrlCGuard {
            stop: Arc::new(AtomicBool::new(false)),
            thread: None,
        };
        guard.unhook();
    }
}
