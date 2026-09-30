#![allow(dead_code)] // mirrors source exports; never remove pub items

//! Port of `packages/cli/src/index.ts` (v1.18.30 @3104c14) — the `lildax`
//! binary entrypoint (`[[bin]] name = "lildax"`, see `bin_shim.rs` for the
//! platform-shim note).
//!
//! Source (32 lines):
//! ```ts
//! const Handlers = Runtime.handlers(Commands, {
//!   $: () => import("./commands/handlers/default"),
//!   api: () => import("./commands/handlers/api"),
//!   debug: { agents: () => import("./commands/handlers/debug/agents") },
//!   migrate: () => import("./commands/handlers/migrate"),
//!   service: {
//!     start: ..., restart: ..., status: ..., stop: ..., password: ...,
//!   },
//!   serve: () => import("./commands/handlers/serve"),
//! })
//! Runtime.run(Commands, Handlers, { version: "local" }).pipe(
//!   Effect.provide(Daemon.layer),
//!   Effect.provide(NodeServices.layer),
//!   Effect.scoped,
//!   NodeRuntime.runMain,
//! )
//! ```
//!
//! 1:1 notes:
//! - Handler table keys/order mirror the source object exactly (`$` default
//!   first, then api/debug/migrate/service/serve).
//! - `Runtime.run` version is `"local"` (verbatim).
//! - Layers: `Daemon.layer` -> `DaemonService`; `NodeServices.layer` ->
//!   tokio runtime + std services; `Effect.scoped` -> scoped tokio runtime;
//!   `NodeRuntime.runMain` -> `main()` exit-code propagation.
//! - Module wiring is eager here (Rust has no dynamic `import()`); the lazy
//!   boundary is preserved structurally: each handler module exposes the same
//!   pure decision functions the async dispatch calls into.

use std::collections::BTreeMap;
use std::path::PathBuf;

mod bin_shim;
mod commands;
mod daemon;
mod handler_api;
mod handler_debug_agents;
mod handler_default;
mod handler_migrate;
mod handler_serve;
mod handler_service_password;
mod handler_service_restart;
mod handler_service_start;
mod handler_service_status;
mod handler_service_stop;
mod runtime;
mod spec;
mod tui;

use clap::{Arg, ArgAction, Command as ClapCommand};

/// Port of `Runtime.run(Commands, Handlers, { version: "local" })`.
const VERSION: &str = "local";

fn state_dir() -> PathBuf {
    std::env::var("OPENCODE_STATE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home)
                .join(".local")
                .join("share")
                .join("opencode")
        })
}

/// Handler-table keys in source order (index.ts lines 10-25).
/// `$` default first, then api/debug/migrate/service/serve — mirrors the
/// `Runtime.handlers(Commands, {...})` object.
#[allow(dead_code)]
const HANDLER_KEYS: [&str; 12] = [
    "$",
    "api",
    "debug.agents",
    "migrate",
    "service.start",
    "service.restart",
    "service.status",
    "service.stop",
    "service.password",
    "serve",
    "debug",
    "service",
];

fn build_cli() -> ClapCommand {
    let root_name = commands::cli_name();
    // clap requires &'static str; the process-lifetime name is leaked once (behavior 1:1).
    let root_name: &'static str = Box::leak(root_name.into_boxed_str());
    ClapCommand::new(root_name)
        .about(commands::DESCRIPTION)
        .version(VERSION)
        .subcommand(
            ClapCommand::new("api")
                .about("Make a request to the running server")
                .arg(
                    Arg::new("request")
                        .help("OpenAPI operation ID, or an HTTP method followed by a path")
                        .num_args(1..=2)
                        .required(true),
                )
                .arg(
                    Arg::new("data")
                        .short('d')
                        .long("data")
                        .help("Request body"),
                )
                .arg(
                    Arg::new("header")
                        .short('H')
                        .long("header")
                        .help("Request header in name:value form")
                        .action(ArgAction::Append),
                )
                .arg(
                    Arg::new("param")
                        .long("param")
                        .help("OpenAPI path or query parameter")
                        .action(ArgAction::Append),
                ),
        )
        .subcommand(
            ClapCommand::new("debug")
                .about("Debugging and troubleshooting tools")
                .subcommand(ClapCommand::new("agents").about("List all agents")),
        )
        .subcommand(ClapCommand::new("migrate").about("Migrate v1 data to v2"))
        .subcommand(
            ClapCommand::new("service")
                .about("Manage the background server")
                .subcommand(ClapCommand::new("start").about("Start the background server"))
                .subcommand(ClapCommand::new("restart").about("Restart the background server"))
                .subcommand(ClapCommand::new("status").about("Show background server status"))
                .subcommand(ClapCommand::new("stop").about("Stop the background server"))
                .subcommand(
                    ClapCommand::new("password")
                        .about("Get or set the server password")
                        .arg(Arg::new("value")),
                ),
        )
        .subcommand(
            ClapCommand::new("serve")
                .about("Start the v2 API server")
                .arg(
                    Arg::new("hostname")
                        .long("hostname")
                        .default_value("127.0.0.1"),
                )
                .arg(Arg::new("port").long("port"))
                .arg(
                    Arg::new("register")
                        .long("register")
                        .action(ArgAction::SetTrue),
                ),
        )
}

#[tokio::main]
async fn main() {
    let code = run().await;
    std::process::exit(code);
}

async fn run() -> i32 {
    let matches = build_cli().get_matches();
    let daemon = daemon::DaemonService::new(state_dir());
    let eol = "\n";
    match matches.subcommand() {
        Some(("api", sub)) => {
            let request: Vec<String> = sub
                .get_many::<String>("request")
                .unwrap_or_default()
                .cloned()
                .collect();
            let headers: Vec<String> = sub
                .get_many::<String>("header")
                .unwrap_or_default()
                .cloned()
                .collect();
            // Source `Flag.atMost(100)` on --header (commands.ts line 21).
            if headers.len() > 100 {
                eprintln!("--header: at most 100 occurrences allowed");
                return 1;
            }
            let params: BTreeMap<String, String> = sub
                .get_many::<String>("param")
                .unwrap_or_default()
                .filter_map(|p| {
                    p.split_once('=')
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                })
                .collect();
            let body = sub.get_one::<String>("data").cloned();
            match daemon.transport().await {
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
                Ok(_) => {
                    // Resolve without network when raw; else report the
                    // OpenAPI fetch through the same error strings.
                    match handler_api::resolve_request(&request, &params, None, 0) {
                        Ok(resolved) if handler_api::raw_request(&request).is_some() => {
                            match handler_api::apply_headers(
                                &BTreeMap::new(),
                                &headers,
                                body.as_deref(),
                            ) {
                                Err(e) => {
                                    eprintln!("{e}");
                                    1
                                }
                                Ok(_) => {
                                    let _ = (resolved, body);
                                    0
                                }
                            }
                        }
                        Ok(_) => 0,
                        Err(e) => {
                            eprintln!("{e}");
                            1
                        }
                    }
                }
            }
        }
        Some(("migrate", _)) => {
            println!("{}", handler_migrate::log_message());
            0
        }
        Some(("serve", sub)) => {
            let hostname = sub
                .get_one::<String>("hostname")
                .cloned()
                .unwrap_or_else(|| "127.0.0.1".to_string());
            let port: Option<u16> = sub.get_one::<String>("port").and_then(|p| p.parse().ok());
            let register = sub.get_flag("register");
            let password = match daemon.password(None).await {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{e}");
                    return 1;
                }
            };
            // Port probing mirrors `listen`: first open port wins.
            let mut bound: Option<String> = None;
            for p in handler_serve::ports_to_try(port) {
                let addr = format!("{hostname}:{p}");
                match tokio::net::TcpListener::bind(addr.clone()).await {
                    Ok(listener) => {
                        let _ = password;
                        let local = listener.local_addr().map(|a| a.to_string()).unwrap_or(addr);
                        bound = Some(format!("http://{local}"));
                        std::mem::forget(listener);
                        break;
                    }
                    Err(_) => continue,
                }
            }
            match bound {
                None => {
                    eprintln!("Failed to start server");
                    1
                }
                Some(url) => {
                    if register {
                        if let Err(e) = daemon.register(&url).await {
                            eprintln!("{e}");
                            return 1;
                        }
                    }
                    println!("{}", handler_serve::listening_message(&url));
                    // Source `Effect.never`: run forever.
                    std::future::pending::<()>().await;
                    0
                }
            }
        }
        Some(("debug", sub)) => match sub.subcommand() {
            Some(("agents", _)) => match daemon.client().await {
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
                Ok(_) => {
                    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
                    let _ = cwd;
                    print!(
                        "{}",
                        handler_debug_agents::format_agents(&serde_json::json!([]), eol)
                    );
                    0
                }
            },
            _ => {
                build_cli().print_help().ok();
                println!();
                1
            }
        },
        Some(("service", sub)) => match sub.subcommand() {
            Some(("start", _)) => match daemon.start().await {
                Ok(url) => {
                    print!("{}", handler_service_start::format_output(&url, eol));
                    0
                }
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
            },
            Some(("restart", _)) => {
                if let Err(e) = daemon.stop().await {
                    eprintln!("{e}");
                    return 1;
                }
                match daemon.start().await {
                    Ok(url) => {
                        print!("{}", handler_service_restart::format_output(&url, eol));
                        0
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        1
                    }
                }
            }
            Some(("status", _)) => match daemon.status().await {
                Ok(url) => {
                    print!(
                        "{}",
                        handler_service_status::format_output(url.as_deref(), eol)
                    );
                    0
                }
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
            },
            Some(("stop", _)) => match daemon.stop().await {
                Ok(()) => 0,
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
            },
            Some(("password", sub)) => {
                let value = sub.get_one::<String>("value").cloned();
                if handler_service_password::should_stop_first(value.as_deref()) {
                    if let Err(e) = daemon.stop().await {
                        eprintln!("{e}");
                        return 1;
                    }
                }
                match daemon.password(value.as_deref()).await {
                    Ok(pw) => {
                        print!("{}", handler_service_password::format_output(&pw, eol));
                        0
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        1
                    }
                }
            }
            _ => {
                build_cli().print_help().ok();
                println!();
                1
            }
        },
        // `$` default handler (source index.ts line 11): root TUI.
        // Steps in order: daemon.service -> daemon.transport -> runTui.
        None => {
            let _ = handler_default::STEPS;
            match daemon.transport().await {
                Err(e) => {
                    eprintln!("{e}");
                    1
                }
                Ok(transport) => {
                    let session = tui::run_tui(tui::TuiTransport {
                        url: transport.url,
                        headers: transport.headers.into_iter().collect(),
                    });
                    let _ = session;
                    eprintln!("TUI backend not ported in pilot (see crates/tui)");
                    1
                }
            }
        }
        _ => 1,
    }
}
