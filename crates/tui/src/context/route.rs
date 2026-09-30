// source: packages/tui/src/context/route.tsx (60 lines, v1.18.30)
// 1:1 port — the route store is an explicit struct; startup seeding and
// the unknown-shape guard in `initialRoute` are preserved verbatim.

#![allow(dead_code)]

use serde_json::Value;

use crate::prompt::history::PromptInfo;

/// Mirrors `HomeRoute`.
#[derive(Debug, Clone, Default)]
pub struct HomeRoute {
    pub prompt: Option<PromptInfo>,
}

/// Mirrors `SessionRoute`.
#[derive(Debug, Clone)]
pub struct SessionRoute {
    pub session_id: String,
    pub prompt: Option<PromptInfo>,
}

/// Mirrors `PluginRoute`.
#[derive(Debug, Clone, Default)]
pub struct PluginRoute {
    pub id: String,
    pub data: Option<Value>,
}

/// Mirrors `Route`.
#[derive(Debug, Clone)]
pub enum Route {
    Home(HomeRoute),
    Session(SessionRoute),
    Plugin(PluginRoute),
}

impl Default for Route {
    fn default() -> Self {
        Route::Home(HomeRoute::default())
    }
}

impl Route {
    pub fn route_type(&self) -> &'static str {
        match self {
            Route::Home(_) => "home",
            Route::Session(_) => "session",
            Route::Plugin(_) => "plugin",
        }
    }
}

/// Mirrors `initialRoute` — unknown shapes resolve to `None` (caller
/// falls back to `{ type: "home" }`).
pub fn initial_route(value: &Value) -> Option<Route> {
    let kind = value.get("type")?.as_str()?;
    match kind {
        "home" => Some(Route::Home(HomeRoute::default())),
        "session" => Some(Route::Session(SessionRoute {
            session_id: value.get("sessionID")?.as_str()?.to_string(),
            prompt: None,
        })),
        "plugin" => Some(Route::Plugin(PluginRoute {
            id: value.get("id")?.as_str()?.to_string(),
            data: None,
        })),
        _ => None,
    }
}

/// Mirrors the Route context value.
#[derive(Debug, Clone, Default)]
pub struct RouteStore {
    data: Route,
}

impl RouteStore {
    /// Mirrors `init` — explicit route wins, then the startup seed
    /// (guarded by `initialRoute`), then home.
    pub fn new(initial: Option<Route>, startup_initial: Option<&Value>) -> Self {
        let data = initial
            .or_else(|| startup_initial.and_then(initial_route))
            .unwrap_or_default();
        Self { data }
    }

    pub fn data(&self) -> &Route {
        &self.data
    }

    /// Mirrors `navigate` (reconcile = wholesale replace).
    pub fn navigate(&mut self, route: Route) {
        self.data = route;
    }
}

/// Mirrors `useRouteData(type)` — typed access to the current route.
pub fn route_data<'a>(store: &'a RouteStore, route_type: &str) -> Option<&'a Route> {
    if store.data.route_type() == route_type {
        Some(&store.data)
    } else {
        None
    }
}
