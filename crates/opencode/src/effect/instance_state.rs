// source: src/effect/instance-state.ts — exports: InstanceState,
// context, workspaceID, directory, make, get, use, useEffect, has, invalidate
// PROVISIONAL: ScopedCache keyed by directory (INFINITE capacity) modelled as
// data; "InstanceRef not provided" die-message verbatim; workspace fallback
// (WorkspaceRef ?? WorkspaceContext.workspaceID) verbatim.

/// source: TypeId "~opencode/InstanceState" — verbatim.
pub const TYPE_ID: &str = "~opencode/InstanceState";

/// source: "InstanceRef not provided" — verbatim die message.
pub const NO_REF_MESSAGE: &str = "InstanceRef not provided";

/// source: cache capacity Number.POSITIVE_INFINITY — verbatim.
pub const CACHE_UNBOUNDED: bool = true;
