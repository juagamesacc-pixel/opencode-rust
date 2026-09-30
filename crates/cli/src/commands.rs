//! Port of `packages/cli/src/commands/commands.ts` (v1.18.30 @3104c14).
//!
//! Source exports: `Commands` (const, lines 6-52) built via `Spec.make` with
//! `OPENCODE_CLI_NAME` (else `"opencode"`) and description
//! `"OpenCode 2.0 preview command line interface"`.
//!
//! 1:1 notes: every subcommand name, description, flag, alias, default, and
//! constraint is preserved verbatim:
//! - `api`: `request` variadic string min 1 max 2 ("operation | method path",
//!   "OpenAPI operation ID, or an HTTP method followed by a path");
//!   `--data/-d` optional string ("Request body"); `--header/-H` string
//!   repeatable at most 100 ("Request header in name:value form");
//!   `--param` optional key=value pairs ("OpenAPI path or query parameter").
//! - `debug agents`: "Debugging and troubleshooting tools" / "List all agents".
//! - `migrate`: "Migrate v1 data to v2".
//! - `service start|restart|status|stop|password`: "Manage the background
//!   server" + per-command descriptions; `password` takes one optional
//!   positional `value`.
//! - `serve`: "Start the v2 API server"; `--hostname` default `"127.0.0.1"`,
//!   `--port` optional integer, `--register` boolean default `false`.

use crate::spec::{make, Options};

/// `OPENCODE_CLI_NAME` if set, else `"opencode"` (source line 6).
pub fn cli_name() -> String {
    std::env::var("OPENCODE_CLI_NAME")
        .ok()
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "opencode".to_string())
}

pub const DESCRIPTION: &str = "OpenCode 2.0 preview command line interface";

/// Flag/argument metadata for one command, mirroring the Effect CLI builder
/// chain in source. Used to construct the clap parser in `main.rs` and kept
/// here so names/descriptions/defaults live next to the tree.
#[derive(Debug, Clone)]
pub struct ParamMeta {
    pub flag: &'static str,
    pub alias: Option<char>,
    pub description: &'static str,
    pub default_string: Option<&'static str>,
    pub default_bool: Option<bool>,
    pub optional: bool,
    pub repeatable: bool,
    pub max: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct ArgMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub optional: bool,
    pub variadic_min: usize,
    pub variadic_max: usize,
}

/// Source `params` for `api` (lines 12-23).
pub const API_REQUEST: ArgMeta = ArgMeta {
    name: "operation | method path",
    description: "OpenAPI operation ID, or an HTTP method followed by a path",
    optional: false,
    variadic_min: 1,
    variadic_max: 2,
};
pub const API_DATA: ParamMeta = ParamMeta {
    flag: "data",
    alias: Some('d'),
    description: "Request body",
    default_string: None,
    default_bool: None,
    optional: true,
    repeatable: false,
    max: None,
};
pub const API_HEADER: ParamMeta = ParamMeta {
    flag: "header",
    alias: Some('H'),
    description: "Request header in name:value form",
    default_string: None,
    default_bool: None,
    optional: true,
    repeatable: true,
    max: Some(100),
};
/// `--param` is `Flag.keyValuePair("param")` + optional (line 22).
pub const API_PARAM: ParamMeta = ParamMeta {
    flag: "param",
    alias: None,
    description: "OpenAPI path or query parameter",
    default_string: None,
    default_bool: None,
    optional: true,
    repeatable: true,
    max: None,
};

/// Source `serve` params (lines 46-49).
pub const SERVE_HOSTNAME: ParamMeta = ParamMeta {
    flag: "hostname",
    alias: None,
    description: "",
    default_string: Some("127.0.0.1"),
    default_bool: None,
    optional: false,
    repeatable: false,
    max: None,
};
pub const SERVE_PORT: ParamMeta = ParamMeta {
    flag: "port",
    alias: None,
    description: "",
    default_string: None,
    default_bool: None,
    optional: true,
    repeatable: false,
    max: None,
};
pub const SERVE_REGISTER: ParamMeta = ParamMeta {
    flag: "register",
    alias: None,
    description: "",
    default_string: None,
    default_bool: Some(false),
    optional: false,
    repeatable: false,
    max: None,
};

/// Port of `Commands` (lines 6-52): builds the full `Spec` tree.
pub fn build_commands() -> crate::spec::Node {
    make(
        &cli_name(),
        Options {
            description: Some(DESCRIPTION.to_string()),
            commands: vec![
                make(
                    "api",
                    Options {
                        description: Some("Make a request to the running server".to_string()),
                        commands: vec![],
                    },
                ),
                make(
                    "debug",
                    Options {
                        description: Some("Debugging and troubleshooting tools".to_string()),
                        commands: vec![make(
                            "agents",
                            Options {
                                description: Some("List all agents".to_string()),
                                commands: vec![],
                            },
                        )],
                    },
                ),
                make(
                    "migrate",
                    Options {
                        description: Some("Migrate v1 data to v2".to_string()),
                        commands: vec![],
                    },
                ),
                make(
                    "service",
                    Options {
                        description: Some("Manage the background server".to_string()),
                        commands: vec![
                            make(
                                "start",
                                Options {
                                    description: Some("Start the background server".to_string()),
                                    commands: vec![],
                                },
                            ),
                            make(
                                "restart",
                                Options {
                                    description: Some("Restart the background server".to_string()),
                                    commands: vec![],
                                },
                            ),
                            make(
                                "status",
                                Options {
                                    description: Some("Show background server status".to_string()),
                                    commands: vec![],
                                },
                            ),
                            make(
                                "stop",
                                Options {
                                    description: Some("Stop the background server".to_string()),
                                    commands: vec![],
                                },
                            ),
                            make(
                                "password",
                                Options {
                                    description: Some("Get or set the server password".to_string()),
                                    commands: vec![],
                                },
                            ),
                        ],
                    },
                ),
                make(
                    "serve",
                    Options {
                        description: Some("Start the v2 API server".to_string()),
                        commands: vec![],
                    },
                ),
            ],
        },
    )
}

/// Node paths in walk order (mirrors `Object.entries` insertion order).
pub const NODE_PATHS: [&str; 12] = [
    "api",
    "debug",
    "debug.agents",
    "migrate",
    "service",
    "service.start",
    "service.restart",
    "service.status",
    "service.stop",
    "service.password",
    "serve",
    "$",
];
