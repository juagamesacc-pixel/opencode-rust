//! Assets module — passthrough for `packages/app/src/assets/help/*`.
//!
//! Binary assets are included via `include_bytes!` with no logic change.
//! Files: `home.png`, `placeholder.png`, `tabs.png`, `introducing-tabs.mp4`.

pub mod help {
    pub static HOME_PNG: &[u8] = include_bytes!("help/home.png");
    pub static PLACEHOLDER_PNG: &[u8] = include_bytes!("help/placeholder.png");
    pub static TABS_PNG: &[u8] = include_bytes!("help/tabs.png");
    pub static INTRODUCING_TABS_MP4: &[u8] = include_bytes!("help/introducing-tabs.mp4");
}
