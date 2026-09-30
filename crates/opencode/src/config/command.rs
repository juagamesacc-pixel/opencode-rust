// source: src/config/command.ts — exports: load, ConfigCommand
// PROVISIONAL pending crates/core (util/glob, v1/config/command,
// v1/config/error) + ./entry-name + ./markdown: glob + template trim verbatim.

/// source: command glob "{command,commands}/**/*.md" — verbatim.
pub const COMMAND_GLOB: &str = "{command,commands}/**/*.md";
/// source: command prefixes ["command/", "commands/"] — verbatim.
pub const COMMAND_PREFIXES: &[&str] = &["command/", "commands/"];
/// source: template = md.content.trim() — verbatim rule.
pub fn template_of(content: &str) -> String {
    content.trim().to_string()
}
