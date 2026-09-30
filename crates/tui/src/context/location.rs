// source: packages/tui/src/context/location.tsx (14 lines, v1.18.30)
// 1:1 port — the optional location accessor becomes an explicit holder.

#![allow(dead_code)]

use super::data::LocationRef;

/// Mirrors the Location context value (accessor of an optional ref).
#[derive(Debug, Clone, Default)]
pub struct Location {
    location: Option<LocationRef>,
}

impl Location {
    pub fn new(location: Option<LocationRef>) -> Self {
        Self { location }
    }

    /// Mirrors `useLocation` (throws outside a provider — here `None`
    /// means "no provider value", the holder itself is always present).
    pub fn get(&self) -> Option<&LocationRef> {
        self.location.as_ref()
    }
}
