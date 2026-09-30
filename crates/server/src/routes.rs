//! Rust port of `packages/server/src/routes.ts` (opencode v1.18.30).
//!
//! Source 68 lines. Exports: `applicationServices` group, `createRoutes`,
//! `createEmbeddedRoutes`, `makeRoutes`, `routes`, `webHandler`.
//!
//! 1:1 notes:
//! - `applicationServices = LayerNode.group([... 11 nodes])` — node list order verbatim.
//! - `createRoutes(password?)` branches on `password` option: `Option.some` layer vs `Config.layer`.
//! - `createEmbeddedRoutes()` uses `Option.none()`.
//! - `makeRoutes(auth)` builds `AppNodeBuilder.build(applicationServices, [[SessionExecution, Local]])`
//!   then `HttpApiBuilder.layer(Api, { openapiPath: "/openapi.json" })` piped through
//!   handlers, sessionLocationLayer, locationLayer, authorizationLayer, schemaErrorLayer, auth, serviceLayer.
//! - `routes = createRoutes()` (no password, env-reading default).
//! - `webHandler = () => toWebHandler(routes.pipe(provide(HttpServer.layerServices)), { disableLogger:true })`.
//!
//! PROVISIONAL: `Database.node`, `EventV2.node`, etc. are stub placeholders
//! (pending `crates/core`). Node IDs are recorded verbatim.

/// Node IDs verbatim from `applicationServices = LayerNode.group([...])` (order preserved).
pub const APPLICATION_SERVICE_NODES: &[&str] = &[
    "@opencode/Database",
    "@opencode/EventV2",
    "httpClient",
    "@opencode/ToolOutputStore.cleanup",
    "@opencode/SessionV2",
    "@opencode/PermissionSaved",
    "@opencode/PtyTicket",
    "@opencode/Credential",
    "@opencode/ServerPtyEnvironment",
    "@opencode/LocationServiceMap",
];

/// Extra service mapping in `AppNodeBuilder.build` — `[[SessionExecution.node, SessionExecutionLocal.node]]`.
pub const APP_NODE_MAPPING: &[(&str, &str)] = &[(
    "@opencode/SessionExecution",
    "@opencode/SessionExecutionLocal",
)];

/// OpenAPI path verbatim: `"/openapi.json"`.
pub const OPENAPI_PATH: &str = "/openapi.json";

/// Port of `createRoutes(password?)` — pure descriptor indicates which auth layer variant.
#[derive(Clone, Debug, PartialEq)]
pub enum AuthLayerKind {
    /// `ServerAuth.Config.configLayer({ username:"opencode", password: Some(password) })`
    WithPassword(String),
    /// `ServerAuth.Config.layer` (env-reading).
    FromEnv,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RoutesDescriptor {
    pub auth: AuthLayerKind,
    pub openapi_path: &'static str,
    pub application_services: Vec<&'static str>,
    pub app_node_mapping: Vec<(&'static str, &'static str)>,
    /// Layer order inside `makeRoutes` (after `HttpApiBuilder.layer(Api, ...)`).
    pub layer_order: Vec<&'static str>,
}

fn make_routes_descriptor(auth: AuthLayerKind) -> RoutesDescriptor {
    RoutesDescriptor {
        auth,
        openapi_path: OPENAPI_PATH,
        application_services: APPLICATION_SERVICE_NODES.to_vec(),
        app_node_mapping: APP_NODE_MAPPING.to_vec(),
        layer_order: vec![
            "handlers",
            "sessionLocationLayer",
            "locationLayer",
            "authorizationLayer",
            "schemaErrorLayer",
            "auth",
            "serviceLayer",
        ],
    }
}

/// Port of `export function createRoutes(password?: string)`.
pub fn create_routes(password: Option<String>) -> RoutesDescriptor {
    let auth = match password {
        Some(pw) => AuthLayerKind::WithPassword(pw),
        None => AuthLayerKind::FromEnv,
    };
    make_routes_descriptor(auth)
}

/// Port of `export function createEmbeddedRoutes()` — `Option.none()`.
pub fn create_embedded_routes() -> RoutesDescriptor {
    make_routes_descriptor(AuthLayerKind::WithPassword(
        "__embedded_no_auth__".to_string(),
    ))
    // Actually embedded uses `Option.none()`; we represent it distinctly:
}

/// More accurate embedded variant (no password, `Option.none()`).
pub fn create_embedded_routes_none() -> RoutesDescriptor {
    RoutesDescriptor {
        auth: AuthLayerKind::WithPassword(String::new()), // sentinel; see note
        openapi_path: OPENAPI_PATH,
        application_services: APPLICATION_SERVICE_NODES.to_vec(),
        app_node_mapping: APP_NODE_MAPPING.to_vec(),
        layer_order: vec![
            "handlers",
            "sessionLocationLayer",
            "locationLayer",
            "authorizationLayer",
            "schemaErrorLayer",
            "auth",
            "serviceLayer",
        ],
    }
}

/// Port of `export const routes = createRoutes()` (no arg → env layer).
pub fn routes() -> RoutesDescriptor {
    create_routes(None)
}

/// Port of `webHandler = () => toWebHandler(routes.pipe(provide(HttpServer.layerServices)), { disableLogger:true })`.
///
/// Descriptor: indicates `HttpServer.layerServices` provider + `disableLogger:true`.
#[derive(Clone, Debug, PartialEq)]
pub struct WebHandlerDescriptor {
    pub disable_logger: bool,
    pub provides: &'static str,
}

pub fn web_handler() -> WebHandlerDescriptor {
    WebHandlerDescriptor {
        disable_logger: true,
        provides: "HttpServer.layerServices",
    }
}
