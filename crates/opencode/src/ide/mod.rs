// source: src/ide/index.ts — exports: Event, AlreadyInstalledError,
// InstallFailedError, ide, alreadyInstalled, install, Ide
// PROVISIONAL pending crates/core (util/error NamedError), @/util/process,
// @opencode-ai/schema/ide-event: process/env access verbatim.

use serde::{Deserialize, Serialize};

/// source: SUPPORTED_IDES — verbatim name/cmd pairs in order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportedIde {
    pub name: &'static str,
    pub cmd: &'static str,
}

pub const SUPPORTED_IDES: &[SupportedIde] = &[
    SupportedIde {
        name: "Windsurf",
        cmd: "windsurf",
    },
    SupportedIde {
        name: "Visual Studio Code - Insiders",
        cmd: "code-insiders",
    },
    SupportedIde {
        name: "Visual Studio Code",
        cmd: "code",
    },
    SupportedIde {
        name: "Cursor",
        cmd: "cursor",
    },
    SupportedIde {
        name: "VSCodium",
        cmd: "codium",
    },
];

/// source: Event = IdeEvent (re-export) — PROVISIONAL pending @opencode-ai/schema/ide-event.
pub type Event = crate::core_provisional::event_v2::Info;

/// source: AlreadyInstalledError — verbatim tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlreadyInstalledError {}

/// source: InstallFailedError { stderr } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallFailedError {
    pub stderr: String,
}

/// source: extension id — verbatim.
pub const EXTENSION_ID: &str = "sst-dev.opencode";

/// source: ide() — TERM_PROGRAM/GIT_ASKPASS detection, verbatim.
pub fn ide(term_program: Option<&str>, git_askpass: Option<&str>) -> &'static str {
    if term_program == Some("vscode") {
        if let Some(v) = git_askpass {
            for ide in SUPPORTED_IDES {
                if v.contains(ide.name) {
                    return ide.name;
                }
            }
        }
    }
    "unknown"
}

/// source: alreadyInstalled() — OPENCODE_CALLER check, verbatim.
pub fn already_installed(opencode_caller: Option<&str>) -> bool {
    matches!(opencode_caller, Some("vscode") | Some("vscode-insiders"))
}

/// source: install() — `Unknown IDE: ${ide}` error, verbatim.
pub fn install_cmd(ide_name: &str) -> Result<Vec<String>, String> {
    let cmd = SUPPORTED_IDES
        .iter()
        .find(|i| i.name == ide_name)
        .map(|i| i.cmd)
        .ok_or_else(|| format!("Unknown IDE: {}", ide_name))?;
    Ok(vec![
        cmd.to_string(),
        "--install-extension".to_string(),
        EXTENSION_ID.to_string(),
    ])
}

/// source: "already installed" stdout marker — verbatim.
pub const ALREADY_INSTALLED_MARKER: &str = "already installed";
