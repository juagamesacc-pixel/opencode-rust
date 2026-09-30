// source: packages/tui/src/context/permission.tsx (26 lines, v1.18.30)
// 1:1 port — seeded from `args.auto`, explicit toggle.

#![allow(dead_code)]

/// Mirrors `PermissionMode`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionMode {
    Auto,
    Normal,
}

/// Mirrors the Permission context value.
#[derive(Debug, Clone, Copy)]
pub struct PermissionStore {
    pub mode: PermissionMode,
}

impl PermissionStore {
    /// Mirrors `init` (`args.auto ? "auto" : "normal"`).
    pub fn new(auto: bool) -> Self {
        Self {
            mode: if auto {
                PermissionMode::Auto
            } else {
                PermissionMode::Normal
            },
        }
    }

    pub fn set(&mut self, mode: PermissionMode) {
        self.mode = mode;
    }

    pub fn toggle(&mut self) {
        self.mode = match self.mode {
            PermissionMode::Auto => PermissionMode::Normal,
            PermissionMode::Normal => PermissionMode::Auto,
        };
    }
}
