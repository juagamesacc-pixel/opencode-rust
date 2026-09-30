//! Rust port of `packages/server/src/handlers/location.ts` (opencode v1.18.30).
//!
//! Source 15 lines: `LocationHandler.handle("location.get", fn* => new Location.Info{ directory, workspaceID, project })`
//!
//! WIRED: `Location.Service` now from `core::location` (verified: LocationRef/LocationInfo/ProjectRef + SERVICE_ID "@opencode/Location").

pub const GROUP: &str = "server.location";
pub const OPERATION: &str = "location.get";
pub const SERVICE_ID: &str = core::location::SERVICE_ID;

// Re-export wired core types
pub use core::location::{LocationInfo, LocationRef, ProjectRef};

/// Pure handler: mirrors `Location.Info{ directory, workspaceID, project }` response construction.
pub fn make_info(
    directory: String,
    workspace_id: Option<String>,
    project: ProjectRef,
) -> LocationInfo {
    LocationInfo {
        directory,
        workspace_id,
        project,
    }
}
