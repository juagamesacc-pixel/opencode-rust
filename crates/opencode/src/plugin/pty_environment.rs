// source: src/plugin/pty-environment.ts — exports: layer, PluginPtyEnvironment
// (shell.env trigger { cwd } → env map, instance-provided; verbatim).
// PROVISIONAL pending server PtyEnvironment + effect + @/project/instance-store + plugin.

/// source: trigger event "shell.env" — verbatim.
pub const SHELL_ENV_EVENT: &str = "shell.env";
