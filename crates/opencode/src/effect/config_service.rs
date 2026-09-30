// source: src/effect/config-service.ts — exports: Shape, ServiceClass,
// Service, ConfigService (configLayer/test + layer/prod duality, example
// ServerAuthConfig ids verbatim).

/// source: example ServerAuthConfig fields — verbatim env names/default.
pub const EXAMPLE_ID: &str = "@opencode/ServerAuthConfig";
pub const EXAMPLE_PASSWORD_ENV: &str = "OPENCODE_SERVER_PASSWORD";
pub const EXAMPLE_USERNAME_ENV: &str = "OPENCODE_SERVER_USERNAME";
pub const EXAMPLE_USERNAME_DEFAULT: &str = "opencode";

/// source: Service helpers — configLayer(input) + layer, verbatim names.
pub const HELPERS: &[&str] = &["configLayer", "layer"];
