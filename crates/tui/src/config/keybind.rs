//! Rust port of `src/config/keybind.ts` (opencode v1.18.30).
//!
//! Keybind definitions, command map, parse/validate, and a binding lookup.
//! The `@opentui/keymap` engine (`createBindingLookup`, `Binding`, timed
//! leader, managed textarea layer) has no Rust equivalent; the lookup is
//! implemented here as a flat command→bindings map with the same
//! `get`/`has`/`gather`/`pick`/`omit` surface.
//!
//! Original file: `packages/tui/src/config/keybind.ts`

use std::collections::HashMap;

/// A key stroke with optional modifiers (mirrors `KeyStroke` schema).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyStroke {
    pub name: String,
    pub ctrl: Option<bool>,
    pub shift: Option<bool>,
    pub meta: Option<bool>,
    pub super_: Option<bool>,
    pub hyper: Option<bool>,
}

/// A binding object with a key and optional event/preventDefault/fallthrough.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingObject {
    pub key: BindingKey,
    pub event: Option<String>,
    pub prevent_default: Option<bool>,
    pub fallthrough: Option<bool>,
}

/// The key field of a binding object: either a string or a KeyStroke.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingKey {
    Str(String),
    Stroke(KeyStroke),
}

/// A single binding item (mirrors `BindingItem` union).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingItem {
    Key(String),
    Stroke(KeyStroke),
    Object(BindingObject),
}

/// A binding value (mirrors `BindingValueSchema` union).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingValue {
    Disabled,
    None,
    Item(BindingItem),
    Items(Vec<BindingItem>),
}

/// The default value for a keybind definition (const-compatible; the only
/// object default in source is `input_paste`'s `{key, preventDefault}`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultBinding {
    Str(&'static str),
    Paste,
}

/// Expand a [`DefaultBinding`] into its [`BindingValue`].
pub fn default_binding_value(default: &DefaultBinding) -> BindingValue {
    match default {
        DefaultBinding::Str(s) if *s == "none" => BindingValue::None,
        DefaultBinding::Str(s) => BindingValue::Item(BindingItem::Key(s.to_string())),
        DefaultBinding::Paste => BindingValue::Item(BindingItem::Object(BindingObject {
            key: BindingKey::Str("ctrl+v".to_string()),
            event: None,
            prevent_default: Some(false),
            fallthrough: None,
        })),
    }
}

/// A keybind definition: default value + description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Definition {
    pub name: &'static str,
    pub default: DefaultBinding,
    pub description: &'static str,
}

/// The leader key default.
pub const LEADER_DEFAULT: &str = "ctrl+x";

/// All keybind definitions in source order.
pub const DEFINITIONS: &[Definition] = &[
    Definition {
        name: "app_exit",
        default: DefaultBinding::Str("ctrl+c,ctrl+d,<leader>q"),
        description: "Exit the application",
    },
    Definition {
        name: "app_debug",
        default: DefaultBinding::Str("none"),
        description: "Toggle debug panel",
    },
    Definition {
        name: "app_console",
        default: DefaultBinding::Str("none"),
        description: "Toggle console",
    },
    Definition {
        name: "app_heap_snapshot",
        default: DefaultBinding::Str("none"),
        description: "Write heap snapshot",
    },
    Definition {
        name: "app_toggle_animations",
        default: DefaultBinding::Str("none"),
        description: "Toggle animations",
    },
    Definition {
        name: "app_toggle_file_context",
        default: DefaultBinding::Str("none"),
        description: "Toggle file context",
    },
    Definition {
        name: "app_toggle_diffwrap",
        default: DefaultBinding::Str("none"),
        description: "Toggle diff wrapping",
    },
    Definition {
        name: "app_toggle_paste_summary",
        default: DefaultBinding::Str("none"),
        description: "Toggle paste summary",
    },
    Definition {
        name: "app_toggle_session_directory_filter",
        default: DefaultBinding::Str("none"),
        description: "Toggle session directory filtering",
    },
    Definition {
        name: "command_list",
        default: DefaultBinding::Str("ctrl+p"),
        description: "List available commands",
    },
    Definition {
        name: "help_show",
        default: DefaultBinding::Str("none"),
        description: "Open help dialog",
    },
    Definition {
        name: "docs_open",
        default: DefaultBinding::Str("none"),
        description: "Open documentation",
    },
    Definition {
        name: "diff_open",
        default: DefaultBinding::Str("none"),
        description: "Open diff viewer",
    },
    Definition {
        name: "diff_close",
        default: DefaultBinding::Str("escape,q"),
        description: "Close diff viewer",
    },
    Definition {
        name: "diff_toggle",
        default: DefaultBinding::Str("enter,space"),
        description: "Toggle diff viewer item",
    },
    Definition {
        name: "diff_expand",
        default: DefaultBinding::Str("right"),
        description: "Expand diff viewer item",
    },
    Definition {
        name: "diff_expand_all",
        default: DefaultBinding::Str("E"),
        description: "Expand all diff viewer folders",
    },
    Definition {
        name: "diff_collapse",
        default: DefaultBinding::Str("left"),
        description: "Collapse diff viewer item",
    },
    Definition {
        name: "diff_switch_focus",
        default: DefaultBinding::Str("tab"),
        description: "Switch diff viewer focus",
    },
    Definition {
        name: "diff_next_hunk",
        default: DefaultBinding::Str("]"),
        description: "Jump to next diff hunk",
    },
    Definition {
        name: "diff_previous_hunk",
        default: DefaultBinding::Str("["),
        description: "Jump to previous diff hunk",
    },
    Definition {
        name: "diff_next_file",
        default: DefaultBinding::Str("n"),
        description: "Jump to next diff file",
    },
    Definition {
        name: "diff_previous_file",
        default: DefaultBinding::Str("p"),
        description: "Jump to previous diff file",
    },
    Definition {
        name: "diff_toggle_file_tree",
        default: DefaultBinding::Str("b"),
        description: "Toggle diff viewer file tree",
    },
    Definition {
        name: "diff_single_patch",
        default: DefaultBinding::Str("s"),
        description: "Toggle single patch view",
    },
    Definition {
        name: "diff_switch_source",
        default: DefaultBinding::Str("d"),
        description: "Switch diff viewer source",
    },
    Definition {
        name: "diff_toggle_view",
        default: DefaultBinding::Str("v"),
        description: "Toggle diff viewer split or unified view",
    },
    Definition {
        name: "diff_help",
        default: DefaultBinding::Str("?"),
        description: "Show more diff viewer shortcuts",
    },
    Definition {
        name: "editor_open",
        default: DefaultBinding::Str("<leader>e"),
        description: "Open external editor",
    },
    Definition {
        name: "theme_list",
        default: DefaultBinding::Str("<leader>t"),
        description: "List available themes",
    },
    Definition {
        name: "theme_switch_mode",
        default: DefaultBinding::Str("none"),
        description: "Switch between light and dark theme mode",
    },
    Definition {
        name: "theme_mode_lock",
        default: DefaultBinding::Str("none"),
        description: "Lock or unlock theme mode",
    },
    Definition {
        name: "sidebar_toggle",
        default: DefaultBinding::Str("<leader>b"),
        description: "Toggle sidebar",
    },
    Definition {
        name: "scrollbar_toggle",
        default: DefaultBinding::Str("none"),
        description: "Toggle session scrollbar",
    },
    Definition {
        name: "status_view",
        default: DefaultBinding::Str("<leader>s"),
        description: "View status",
    },
    Definition {
        name: "debug_view",
        default: DefaultBinding::Str("none"),
        description: "View debug info",
    },
    Definition {
        name: "session_export",
        default: DefaultBinding::Str("<leader>x"),
        description: "Export session to editor",
    },
    Definition {
        name: "session_copy",
        default: DefaultBinding::Str("none"),
        description: "Copy session transcript",
    },
    Definition {
        name: "session_move",
        default: DefaultBinding::Str("none"),
        description: "Move session",
    },
    Definition {
        name: "session_new",
        default: DefaultBinding::Str("<leader>n"),
        description: "Create a new session",
    },
    Definition {
        name: "session_list",
        default: DefaultBinding::Str("<leader>l"),
        description: "List all sessions",
    },
    Definition {
        name: "session_timeline",
        default: DefaultBinding::Str("<leader>g"),
        description: "Show session timeline",
    },
    Definition {
        name: "session_fork",
        default: DefaultBinding::Str("none"),
        description: "Fork session from message",
    },
    Definition {
        name: "session_rename",
        default: DefaultBinding::Str("ctrl+r"),
        description: "Rename session",
    },
    Definition {
        name: "session_delete",
        default: DefaultBinding::Str("ctrl+d"),
        description: "Delete session",
    },
    Definition {
        name: "session_share",
        default: DefaultBinding::Str("none"),
        description: "Share current session",
    },
    Definition {
        name: "session_unshare",
        default: DefaultBinding::Str("none"),
        description: "Unshare current session",
    },
    Definition {
        name: "session_interrupt",
        default: DefaultBinding::Str("escape"),
        description: "Interrupt current session",
    },
    Definition {
        name: "session_background",
        default: DefaultBinding::Str("ctrl+b"),
        description: "Background synchronous subagents",
    },
    Definition {
        name: "session_compact",
        default: DefaultBinding::Str("<leader>c"),
        description: "Compact the session",
    },
    Definition {
        name: "session_toggle_timestamps",
        default: DefaultBinding::Str("none"),
        description: "Toggle message timestamps",
    },
    Definition {
        name: "session_toggle_generic_tool_output",
        default: DefaultBinding::Str("none"),
        description: "Toggle generic tool output",
    },
    Definition {
        name: "session_queued_prompts",
        default: DefaultBinding::Str("<leader>q"),
        description: "Manage queued prompts",
    },
    Definition {
        name: "session_child_first",
        default: DefaultBinding::Str("<leader>down"),
        description: "Go to first child session",
    },
    Definition {
        name: "session_child_cycle",
        default: DefaultBinding::Str("right"),
        description: "Go to next child session",
    },
    Definition {
        name: "session_child_cycle_reverse",
        default: DefaultBinding::Str("left"),
        description: "Go to previous child session",
    },
    Definition {
        name: "session_parent",
        default: DefaultBinding::Str("up"),
        description: "Go to parent session",
    },
    Definition {
        name: "session_pin_toggle",
        default: DefaultBinding::Str("ctrl+f"),
        description: "Pin or unpin session in the session list",
    },
    Definition {
        name: "session_quick_switch_1",
        default: DefaultBinding::Str("<leader>1"),
        description: "Switch to session in quick slot 1",
    },
    Definition {
        name: "session_quick_switch_2",
        default: DefaultBinding::Str("<leader>2"),
        description: "Switch to session in quick slot 2",
    },
    Definition {
        name: "session_quick_switch_3",
        default: DefaultBinding::Str("<leader>3"),
        description: "Switch to session in quick slot 3",
    },
    Definition {
        name: "session_quick_switch_4",
        default: DefaultBinding::Str("<leader>4"),
        description: "Switch to session in quick slot 4",
    },
    Definition {
        name: "session_quick_switch_5",
        default: DefaultBinding::Str("<leader>5"),
        description: "Switch to session in quick slot 5",
    },
    Definition {
        name: "session_quick_switch_6",
        default: DefaultBinding::Str("<leader>6"),
        description: "Switch to session in quick slot 6",
    },
    Definition {
        name: "session_quick_switch_7",
        default: DefaultBinding::Str("<leader>7"),
        description: "Switch to session in quick slot 7",
    },
    Definition {
        name: "session_quick_switch_8",
        default: DefaultBinding::Str("<leader>8"),
        description: "Switch to session in quick slot 8",
    },
    Definition {
        name: "session_quick_switch_9",
        default: DefaultBinding::Str("<leader>9"),
        description: "Switch to session in quick slot 9",
    },
    Definition {
        name: "stash_delete",
        default: DefaultBinding::Str("ctrl+d"),
        description: "Delete stash entry",
    },
    Definition {
        name: "model_provider_list",
        default: DefaultBinding::Str("ctrl+a"),
        description: "Open provider list from model dialog",
    },
    Definition {
        name: "model_favorite_toggle",
        default: DefaultBinding::Str("ctrl+f"),
        description: "Toggle model favorite status",
    },
    Definition {
        name: "model_list",
        default: DefaultBinding::Str("<leader>m"),
        description: "List available models",
    },
    Definition {
        name: "model_cycle_recent",
        default: DefaultBinding::Str("f2"),
        description: "Next recently used model",
    },
    Definition {
        name: "model_cycle_recent_reverse",
        default: DefaultBinding::Str("shift+f2"),
        description: "Previous recently used model",
    },
    Definition {
        name: "model_cycle_favorite",
        default: DefaultBinding::Str("none"),
        description: "Next favorite model",
    },
    Definition {
        name: "model_cycle_favorite_reverse",
        default: DefaultBinding::Str("none"),
        description: "Previous favorite model",
    },
    Definition {
        name: "mcp_list",
        default: DefaultBinding::Str("none"),
        description: "List MCP servers",
    },
    Definition {
        name: "provider_connect",
        default: DefaultBinding::Str("none"),
        description: "Connect provider",
    },
    Definition {
        name: "console_org_switch",
        default: DefaultBinding::Str("none"),
        description: "Switch console organization",
    },
    Definition {
        name: "agent_list",
        default: DefaultBinding::Str("<leader>a"),
        description: "List agents",
    },
    Definition {
        name: "agent_cycle",
        default: DefaultBinding::Str("tab"),
        description: "Next agent",
    },
    Definition {
        name: "agent_cycle_reverse",
        default: DefaultBinding::Str("shift+tab"),
        description: "Previous agent",
    },
    Definition {
        name: "variant_cycle",
        default: DefaultBinding::Str("ctrl+t"),
        description: "Cycle model variants",
    },
    Definition {
        name: "variant_list",
        default: DefaultBinding::Str("none"),
        description: "List model variants",
    },
    Definition {
        name: "messages_page_up",
        default: DefaultBinding::Str("pageup,ctrl+alt+b"),
        description: "Scroll messages up by one page",
    },
    Definition {
        name: "messages_page_down",
        default: DefaultBinding::Str("pagedown,ctrl+alt+f"),
        description: "Scroll messages down by one page",
    },
    Definition {
        name: "messages_line_up",
        default: DefaultBinding::Str("ctrl+alt+y"),
        description: "Scroll messages up by one line",
    },
    Definition {
        name: "messages_line_down",
        default: DefaultBinding::Str("ctrl+alt+e"),
        description: "Scroll messages down by one line",
    },
    Definition {
        name: "messages_half_page_up",
        default: DefaultBinding::Str("ctrl+alt+u"),
        description: "Scroll messages up by half page",
    },
    Definition {
        name: "messages_half_page_down",
        default: DefaultBinding::Str("ctrl+alt+d"),
        description: "Scroll messages down by half page",
    },
    Definition {
        name: "messages_first",
        default: DefaultBinding::Str("ctrl+g,home"),
        description: "Navigate to first message",
    },
    Definition {
        name: "messages_last",
        default: DefaultBinding::Str("ctrl+alt+g,end"),
        description: "Navigate to last message",
    },
    Definition {
        name: "messages_next",
        default: DefaultBinding::Str("none"),
        description: "Navigate to next message",
    },
    Definition {
        name: "messages_previous",
        default: DefaultBinding::Str("none"),
        description: "Navigate to previous message",
    },
    Definition {
        name: "messages_last_user",
        default: DefaultBinding::Str("none"),
        description: "Navigate to last user message",
    },
    Definition {
        name: "messages_copy",
        default: DefaultBinding::Str("<leader>y"),
        description: "Copy message",
    },
    Definition {
        name: "messages_undo",
        default: DefaultBinding::Str("<leader>u"),
        description: "Undo message",
    },
    Definition {
        name: "messages_redo",
        default: DefaultBinding::Str("<leader>r"),
        description: "Redo message",
    },
    Definition {
        name: "messages_toggle_conceal",
        default: DefaultBinding::Str("<leader>h"),
        description: "Toggle code block concealment in messages",
    },
    Definition {
        name: "tool_details",
        default: DefaultBinding::Str("none"),
        description: "Toggle tool details visibility",
    },
    Definition {
        name: "display_thinking",
        default: DefaultBinding::Str("none"),
        description: "Toggle thinking blocks visibility",
    },
    Definition {
        name: "prompt_submit",
        default: DefaultBinding::Str("none"),
        description: "Submit prompt",
    },
    Definition {
        name: "prompt_editor_context_clear",
        default: DefaultBinding::Str("none"),
        description: "Clear editor context",
    },
    Definition {
        name: "prompt_skills",
        default: DefaultBinding::Str("none"),
        description: "Open skill selector",
    },
    Definition {
        name: "prompt_stash",
        default: DefaultBinding::Str("none"),
        description: "Stash prompt",
    },
    Definition {
        name: "prompt_stash_pop",
        default: DefaultBinding::Str("none"),
        description: "Pop stashed prompt",
    },
    Definition {
        name: "prompt_stash_list",
        default: DefaultBinding::Str("none"),
        description: "List stashed prompts",
    },
    Definition {
        name: "workspace_set",
        default: DefaultBinding::Str("none"),
        description: "Set workspace",
    },
    Definition {
        name: "input_clear",
        default: DefaultBinding::Str("ctrl+c"),
        description: "Clear input field",
    },
    Definition {
        name: "input_paste",
        default: DefaultBinding::Paste,
        description: "Paste from clipboard",
    },
    Definition {
        name: "input_submit",
        default: DefaultBinding::Str("return"),
        description: "Submit input",
    },
    Definition {
        name: "input_newline",
        default: DefaultBinding::Str("shift+return,ctrl+return,alt+return,ctrl+j"),
        description: "Insert newline in input",
    },
    Definition {
        name: "input_move_left",
        default: DefaultBinding::Str("left,ctrl+b"),
        description: "Move cursor left in input",
    },
    Definition {
        name: "input_move_right",
        default: DefaultBinding::Str("right,ctrl+f"),
        description: "Move cursor right in input",
    },
    Definition {
        name: "input_move_up",
        default: DefaultBinding::Str("up"),
        description: "Move cursor up in input",
    },
    Definition {
        name: "input_move_down",
        default: DefaultBinding::Str("down"),
        description: "Move cursor down in input",
    },
    Definition {
        name: "input_select_left",
        default: DefaultBinding::Str("shift+left"),
        description: "Select left in input",
    },
    Definition {
        name: "input_select_right",
        default: DefaultBinding::Str("shift+right"),
        description: "Select right in input",
    },
    Definition {
        name: "input_select_up",
        default: DefaultBinding::Str("shift+up"),
        description: "Select up in input",
    },
    Definition {
        name: "input_select_down",
        default: DefaultBinding::Str("shift+down"),
        description: "Select down in input",
    },
    Definition {
        name: "input_line_home",
        default: DefaultBinding::Str("ctrl+a"),
        description: "Move to start of line in input",
    },
    Definition {
        name: "input_line_end",
        default: DefaultBinding::Str("ctrl+e"),
        description: "Move to end of line in input",
    },
    Definition {
        name: "input_select_line_home",
        default: DefaultBinding::Str("ctrl+shift+a"),
        description: "Select to start of line in input",
    },
    Definition {
        name: "input_select_line_end",
        default: DefaultBinding::Str("ctrl+shift+e"),
        description: "Select to end of line in input",
    },
    Definition {
        name: "input_visual_line_home",
        default: DefaultBinding::Str("alt+a"),
        description: "Move to start of visual line in input",
    },
    Definition {
        name: "input_visual_line_end",
        default: DefaultBinding::Str("alt+e"),
        description: "Move to end of visual line in input",
    },
    Definition {
        name: "input_select_visual_line_home",
        default: DefaultBinding::Str("alt+shift+a"),
        description: "Select to start of visual line in input",
    },
    Definition {
        name: "input_select_visual_line_end",
        default: DefaultBinding::Str("alt+shift+e"),
        description: "Select to end of visual line in input",
    },
    Definition {
        name: "input_buffer_home",
        default: DefaultBinding::Str("home"),
        description: "Move to start of buffer in input",
    },
    Definition {
        name: "input_buffer_end",
        default: DefaultBinding::Str("end"),
        description: "Move to end of buffer in input",
    },
    Definition {
        name: "input_select_buffer_home",
        default: DefaultBinding::Str("shift+home"),
        description: "Select to start of buffer in input",
    },
    Definition {
        name: "input_select_buffer_end",
        default: DefaultBinding::Str("shift+end"),
        description: "Select to end of buffer in input",
    },
    Definition {
        name: "input_delete_line",
        default: DefaultBinding::Str("ctrl+shift+d"),
        description: "Delete line in input",
    },
    Definition {
        name: "input_delete_to_line_end",
        default: DefaultBinding::Str("ctrl+k"),
        description: "Delete to end of line in input",
    },
    Definition {
        name: "input_delete_to_line_start",
        default: DefaultBinding::Str("ctrl+u"),
        description: "Delete to start of line in input",
    },
    Definition {
        name: "input_backspace",
        default: DefaultBinding::Str("backspace,shift+backspace"),
        description: "Backspace in input",
    },
    Definition {
        name: "input_delete",
        default: DefaultBinding::Str("ctrl+d,delete,shift+delete"),
        description: "Delete character in input",
    },
    Definition {
        name: "input_undo",
        default: DefaultBinding::Str("ctrl+-,super+z"),
        description: "Undo in input",
    },
    Definition {
        name: "input_redo",
        default: DefaultBinding::Str("ctrl+.,super+shift+z"),
        description: "Redo in input",
    },
    Definition {
        name: "input_word_forward",
        default: DefaultBinding::Str("alt+f,alt+right,ctrl+right"),
        description: "Move word forward in input",
    },
    Definition {
        name: "input_word_backward",
        default: DefaultBinding::Str("alt+b,alt+left,ctrl+left"),
        description: "Move word backward in input",
    },
    Definition {
        name: "input_select_word_forward",
        default: DefaultBinding::Str("alt+shift+f,alt+shift+right"),
        description: "Select word forward in input",
    },
    Definition {
        name: "input_select_word_backward",
        default: DefaultBinding::Str("alt+shift+b,alt+shift+left"),
        description: "Select word backward in input",
    },
    Definition {
        name: "input_delete_word_forward",
        default: DefaultBinding::Str("alt+d,alt+delete,ctrl+delete"),
        description: "Delete word forward in input",
    },
    Definition {
        name: "input_delete_word_backward",
        default: DefaultBinding::Str("ctrl+w,ctrl+backspace,alt+backspace"),
        description: "Delete word backward in input",
    },
    Definition {
        name: "input_select_all",
        default: DefaultBinding::Str("super+a"),
        description: "Select all in input",
    },
    Definition {
        name: "history_previous",
        default: DefaultBinding::Str("up"),
        description: "Previous history item",
    },
    Definition {
        name: "history_next",
        default: DefaultBinding::Str("down"),
        description: "Next history item",
    },
    Definition {
        name: "dialog.select.prev",
        default: DefaultBinding::Str("up,ctrl+p"),
        description: "Move to previous dialog item",
    },
    Definition {
        name: "dialog.select.next",
        default: DefaultBinding::Str("down,ctrl+n"),
        description: "Move to next dialog item",
    },
    Definition {
        name: "dialog.select.page_up",
        default: DefaultBinding::Str("pageup"),
        description: "Move up one page in dialog",
    },
    Definition {
        name: "dialog.select.page_down",
        default: DefaultBinding::Str("pagedown"),
        description: "Move down one page in dialog",
    },
    Definition {
        name: "dialog.select.home",
        default: DefaultBinding::Str("home"),
        description: "Move to first dialog item",
    },
    Definition {
        name: "dialog.select.end",
        default: DefaultBinding::Str("end"),
        description: "Move to last dialog item",
    },
    Definition {
        name: "dialog.select.submit",
        default: DefaultBinding::Str("return"),
        description: "Submit selected dialog item",
    },
    Definition {
        name: "dialog.prompt.submit",
        default: DefaultBinding::Str("return"),
        description: "Submit dialog prompt",
    },
    Definition {
        name: "dialog.mcp.toggle",
        default: DefaultBinding::Str("space"),
        description: "Toggle MCP in MCP dialog",
    },
    Definition {
        name: "dialog.move_session.new",
        default: DefaultBinding::Str("ctrl+m"),
        description: "New project copy",
    },
    Definition {
        name: "dialog.move_session.delete",
        default: DefaultBinding::Str("ctrl+d"),
        description: "Delete project copy",
    },
    Definition {
        name: "dialog.move_session.refresh",
        default: DefaultBinding::Str("ctrl+r"),
        description: "Refresh project copies",
    },
    Definition {
        name: "prompt.autocomplete.prev",
        default: DefaultBinding::Str("up,ctrl+p"),
        description: "Move to previous autocomplete item",
    },
    Definition {
        name: "prompt.autocomplete.next",
        default: DefaultBinding::Str("down,ctrl+n"),
        description: "Move to next autocomplete item",
    },
    Definition {
        name: "prompt.autocomplete.hide",
        default: DefaultBinding::Str("escape"),
        description: "Hide autocomplete",
    },
    Definition {
        name: "prompt.autocomplete.select",
        default: DefaultBinding::Str("return"),
        description: "Select autocomplete item",
    },
    Definition {
        name: "prompt.autocomplete.complete",
        default: DefaultBinding::Str("tab"),
        description: "Complete autocomplete item",
    },
    Definition {
        name: "permission.prompt.fullscreen",
        default: DefaultBinding::Str("ctrl+f"),
        description: "Toggle permission prompt fullscreen",
    },
    Definition {
        name: "plugins.toggle",
        default: DefaultBinding::Str("space"),
        description: "Toggle plugin",
    },
    Definition {
        name: "dialog.plugins.install",
        default: DefaultBinding::Str("shift+i"),
        description: "Install plugin from plugin dialog",
    },
    Definition {
        name: "terminal_suspend",
        default: DefaultBinding::Str("ctrl+z"),
        description: "Suspend terminal",
    },
    Definition {
        name: "terminal_title_toggle",
        default: DefaultBinding::Str("none"),
        description: "Toggle terminal title",
    },
    Definition {
        name: "tips_toggle",
        default: DefaultBinding::Str("<leader>h"),
        description: "Toggle tips on home screen",
    },
    Definition {
        name: "plugin_manager",
        default: DefaultBinding::Str("none"),
        description: "Open plugin manager dialog",
    },
    Definition {
        name: "plugin_install",
        default: DefaultBinding::Str("none"),
        description: "Install plugin",
    },
    Definition {
        name: "which_key_toggle",
        default: DefaultBinding::Str("ctrl+alt+k"),
        description: "Toggle which-key panel",
    },
    Definition {
        name: "which_key_layout_toggle",
        default: DefaultBinding::Str("ctrl+alt+shift+k"),
        description: "Switch which-key layout",
    },
    Definition {
        name: "which_key_pending_toggle",
        default: DefaultBinding::Str("ctrl+alt+shift+p"),
        description: "Toggle which-key pending preview",
    },
    Definition {
        name: "which_key_group_previous",
        default: DefaultBinding::Str("ctrl+alt+left,ctrl+alt+["),
        description: "Previous which-key group",
    },
    Definition {
        name: "which_key_group_next",
        default: DefaultBinding::Str("ctrl+alt+right,ctrl+alt+]"),
        description: "Next which-key group",
    },
    Definition {
        name: "which_key_scroll_up",
        default: DefaultBinding::Str("ctrl+alt+up,ctrl+alt+p"),
        description: "Scroll which-key up",
    },
    Definition {
        name: "which_key_scroll_down",
        default: DefaultBinding::Str("ctrl+alt+down,ctrl+alt+n"),
        description: "Scroll which-key down",
    },
    Definition {
        name: "which_key_page_up",
        default: DefaultBinding::Str("ctrl+alt+pageup"),
        description: "Page which-key up",
    },
    Definition {
        name: "which_key_page_down",
        default: DefaultBinding::Str("ctrl+alt+pagedown"),
        description: "Page which-key down",
    },
    Definition {
        name: "which_key_home",
        default: DefaultBinding::Str("ctrl+alt+home"),
        description: "Jump to first which-key binding",
    },
    Definition {
        name: "which_key_end",
        default: DefaultBinding::Str("ctrl+alt+end"),
        description: "Jump to last which-key binding",
    },
];

/// Command map: keybind name → command string.
pub const COMMAND_MAP: &[(&str, &str)] = &[
    ("app_exit", "app.exit"),
    ("app_debug", "app.debug"),
    ("app_console", "app.console"),
    ("app_heap_snapshot", "app.heap_snapshot"),
    ("app_toggle_animations", "app.toggle.animations"),
    ("app_toggle_file_context", "app.toggle.file_context"),
    ("app_toggle_diffwrap", "app.toggle.diffwrap"),
    ("app_toggle_paste_summary", "app.toggle.paste_summary"),
    (
        "app_toggle_session_directory_filter",
        "app.toggle.session_directory_filter",
    ),
    ("command_list", "command.palette.show"),
    ("help_show", "help.show"),
    ("docs_open", "docs.open"),
    ("diff_open", "diff.open"),
    ("diff_close", "diff.close"),
    ("diff_toggle", "diff.toggle"),
    ("diff_expand", "diff.expand"),
    ("diff_expand_all", "diff.expand_all"),
    ("diff_collapse", "diff.collapse"),
    ("diff_switch_focus", "diff.switch_focus"),
    ("diff_next_hunk", "diff.next_hunk"),
    ("diff_previous_hunk", "diff.previous_hunk"),
    ("diff_next_file", "diff.next_file"),
    ("diff_previous_file", "diff.previous_file"),
    ("diff_toggle_file_tree", "diff.toggle_file_tree"),
    ("diff_single_patch", "diff.single_patch"),
    ("diff_switch_source", "diff.switch_source"),
    ("diff_toggle_view", "diff.toggle_view"),
    ("diff_help", "diff.help"),
    ("editor_open", "prompt.editor"),
    ("theme_list", "theme.switch"),
    ("theme_switch_mode", "theme.switch_mode"),
    ("theme_mode_lock", "theme.mode.lock"),
    ("sidebar_toggle", "session.sidebar.toggle"),
    ("scrollbar_toggle", "session.toggle.scrollbar"),
    ("status_view", "opencode.status"),
    ("debug_view", "opencode.debug"),
    ("session_export", "session.export"),
    ("session_copy", "session.copy"),
    ("session_move", "session.move"),
    ("session_new", "session.new"),
    ("session_list", "session.list"),
    ("session_timeline", "session.timeline"),
    ("session_fork", "session.fork"),
    ("session_rename", "session.rename"),
    ("session_delete", "session.delete"),
    ("session_share", "session.share"),
    ("session_unshare", "session.unshare"),
    ("session_interrupt", "session.interrupt"),
    ("session_background", "session.background"),
    ("session_compact", "session.compact"),
    ("session_toggle_timestamps", "session.toggle.timestamps"),
    (
        "session_toggle_generic_tool_output",
        "session.toggle.generic_tool_output",
    ),
    ("session_queued_prompts", "session.queued_prompts"),
    ("session_child_first", "session.child.first"),
    ("session_child_cycle", "session.child.next"),
    ("session_child_cycle_reverse", "session.child.previous"),
    ("session_parent", "session.parent"),
    ("session_pin_toggle", "session.pin.toggle"),
    ("session_quick_switch_1", "session.quick_switch.1"),
    ("session_quick_switch_2", "session.quick_switch.2"),
    ("session_quick_switch_3", "session.quick_switch.3"),
    ("session_quick_switch_4", "session.quick_switch.4"),
    ("session_quick_switch_5", "session.quick_switch.5"),
    ("session_quick_switch_6", "session.quick_switch.6"),
    ("session_quick_switch_7", "session.quick_switch.7"),
    ("session_quick_switch_8", "session.quick_switch.8"),
    ("session_quick_switch_9", "session.quick_switch.9"),
    ("stash_delete", "stash.delete"),
    ("model_provider_list", "model.dialog.provider"),
    ("model_favorite_toggle", "model.dialog.favorite"),
    ("model_list", "model.list"),
    ("model_cycle_recent", "model.cycle_recent"),
    ("model_cycle_recent_reverse", "model.cycle_recent_reverse"),
    ("model_cycle_favorite", "model.cycle_favorite"),
    (
        "model_cycle_favorite_reverse",
        "model.cycle_favorite_reverse",
    ),
    ("mcp_list", "mcp.list"),
    ("provider_connect", "provider.connect"),
    ("console_org_switch", "console.org.switch"),
    ("agent_list", "agent.list"),
    ("agent_cycle", "agent.cycle"),
    ("agent_cycle_reverse", "agent.cycle.reverse"),
    ("variant_cycle", "variant.cycle"),
    ("variant_list", "variant.list"),
    ("messages_page_up", "session.page.up"),
    ("messages_page_down", "session.page.down"),
    ("messages_line_up", "session.line.up"),
    ("messages_line_down", "session.line.down"),
    ("messages_half_page_up", "session.half.page.up"),
    ("messages_half_page_down", "session.half.page.down"),
    ("messages_first", "session.first"),
    ("messages_last", "session.last"),
    ("messages_next", "session.message.next"),
    ("messages_previous", "session.message.previous"),
    ("messages_last_user", "session.messages_last_user"),
    ("messages_copy", "messages.copy"),
    ("messages_undo", "session.undo"),
    ("messages_redo", "session.redo"),
    ("messages_toggle_conceal", "session.toggle.conceal"),
    ("tool_details", "session.toggle.actions"),
    ("display_thinking", "session.toggle.thinking"),
    ("prompt_submit", "prompt.submit"),
    ("prompt_editor_context_clear", "prompt.editor_context.clear"),
    ("prompt_skills", "prompt.skills"),
    ("prompt_stash", "prompt.stash"),
    ("prompt_stash_pop", "prompt.stash.pop"),
    ("prompt_stash_list", "prompt.stash.list"),
    ("workspace_set", "workspace.set"),
    ("input_clear", "prompt.clear"),
    ("input_paste", "prompt.paste"),
    ("input_submit", "input.submit"),
    ("input_newline", "input.newline"),
    ("input_move_left", "input.move.left"),
    ("input_move_right", "input.move.right"),
    ("input_move_up", "input.move.up"),
    ("input_move_down", "input.move.down"),
    ("input_select_left", "input.select.left"),
    ("input_select_right", "input.select.right"),
    ("input_select_up", "input.select.up"),
    ("input_select_down", "input.select.down"),
    ("input_line_home", "input.line.home"),
    ("input_line_end", "input.line.end"),
    ("input_select_line_home", "input.select.line.home"),
    ("input_select_line_end", "input.select.line.end"),
    ("input_visual_line_home", "input.visual.line.home"),
    ("input_visual_line_end", "input.visual.line.end"),
    (
        "input_select_visual_line_home",
        "input.select.visual.line.home",
    ),
    (
        "input_select_visual_line_end",
        "input.select.visual.line.end",
    ),
    ("input_buffer_home", "input.buffer.home"),
    ("input_buffer_end", "input.buffer.end"),
    ("input_select_buffer_home", "input.select.buffer.home"),
    ("input_select_buffer_end", "input.select.buffer.end"),
    ("input_delete_line", "input.delete.line"),
    ("input_delete_to_line_end", "input.delete.to.line.end"),
    ("input_delete_to_line_start", "input.delete.to.line.start"),
    ("input_backspace", "input.backspace"),
    ("input_delete", "input.delete"),
    ("input_undo", "input.undo"),
    ("input_redo", "input.redo"),
    ("input_word_forward", "input.word.forward"),
    ("input_word_backward", "input.word.backward"),
    ("input_select_word_forward", "input.select.word.forward"),
    ("input_select_word_backward", "input.select.word.backward"),
    ("input_delete_word_forward", "input.delete.word.forward"),
    ("input_delete_word_backward", "input.delete.word.backward"),
    ("input_select_all", "input.select.all"),
    ("history_previous", "prompt.history.previous"),
    ("history_next", "prompt.history.next"),
    ("terminal_suspend", "terminal.suspend"),
    ("terminal_title_toggle", "terminal.title.toggle"),
    ("tips_toggle", "tips.toggle"),
    ("plugin_manager", "plugins.list"),
    ("plugin_install", "plugins.install"),
    ("which_key_toggle", "which-key.toggle"),
    ("which_key_layout_toggle", "which-key.layout.toggle"),
    ("which_key_pending_toggle", "which-key.pending.toggle"),
    ("which_key_group_previous", "which-key.group.previous"),
    ("which_key_group_next", "which-key.group.next"),
    ("which_key_scroll_up", "which-key.scroll.up"),
    ("which_key_scroll_down", "which-key.scroll.down"),
    ("which_key_page_up", "which-key.page.up"),
    ("which_key_page_down", "which-key.page.down"),
    ("which_key_home", "which-key.home"),
    ("which_key_end", "which-key.end"),
];

/// A resolved binding in the lookup.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    pub key: BindingKey,
    pub event: Option<String>,
    pub prevent_default: Option<bool>,
    pub fallthrough: Option<bool>,
    pub desc: Option<String>,
}

/// The binding lookup view (mirrors `BindingLookupView`).
#[derive(Debug, Clone, Default)]
pub struct BindingLookup {
    map: HashMap<String, Vec<Binding>>,
}

impl BindingLookup {
    /// Build a lookup from a keybind config map.
    pub fn new(config: &HashMap<String, BindingValue>) -> Self {
        let mut map: HashMap<String, Vec<Binding>> = HashMap::new();
        for (name, value) in config {
            let command = COMMAND_MAP
                .iter()
                .find(|(k, _)| *k == name.as_str())
                .map(|(_, v)| v.to_string())
                .unwrap_or_else(|| name.clone());
            let bindings = value_to_bindings(value, &command);
            if !bindings.is_empty() {
                map.entry(command).or_default().extend(bindings);
            }
        }
        Self { map }
    }

    /// Get bindings for a command.
    pub fn get(&self, command: &str) -> &[Binding] {
        self.map.get(command).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Check if a command has any bindings.
    pub fn has(&self, command: &str) -> bool {
        !self.get(command).is_empty()
    }

    /// Gather bindings for multiple commands.
    pub fn gather(&self, _name: &str, commands: &[&str]) -> Vec<Binding> {
        let mut out = Vec::new();
        for cmd in commands {
            out.extend(self.get(cmd).iter().cloned());
        }
        out
    }

    /// Pick bindings for commands (same as gather at this layer).
    pub fn pick(&self, name: &str, commands: &[&str]) -> Vec<Binding> {
        self.gather(name, commands)
    }

    /// Omit bindings for commands (empty at this layer).
    pub fn omit(&self, _name: &str, _commands: &[&str]) -> Vec<Binding> {
        Vec::new()
    }
}

fn value_to_bindings(value: &BindingValue, command: &str) -> Vec<Binding> {
    let desc = command_descriptions().get(command).cloned();
    match value {
        BindingValue::Disabled | BindingValue::None => Vec::new(),
        BindingValue::Item(item) => item_to_bindings(item, desc),
        BindingValue::Items(items) => items
            .iter()
            .flat_map(|item| item_to_bindings(item, desc.clone()))
            .collect(),
    }
}

fn item_to_bindings(item: &BindingItem, desc: Option<String>) -> Vec<Binding> {
    match item {
        BindingItem::Key(s) => vec![Binding {
            key: BindingKey::Str(s.clone()),
            event: None,
            prevent_default: None,
            fallthrough: None,
            desc,
        }],
        BindingItem::Stroke(stroke) => vec![Binding {
            key: BindingKey::Stroke(stroke.clone()),
            event: None,
            prevent_default: None,
            fallthrough: None,
            desc,
        }],
        BindingItem::Object(obj) => vec![Binding {
            key: obj.key.clone(),
            event: obj.event.clone(),
            prevent_default: obj.prevent_default,
            fallthrough: obj.fallthrough,
            desc,
        }],
    }
}

/// Descriptions for each keybind name.
pub fn descriptions() -> HashMap<&'static str, &'static str> {
    DEFINITIONS
        .iter()
        .map(|d| (d.name, d.description))
        .collect()
}

/// Command descriptions: command → description.
pub fn command_descriptions() -> HashMap<String, String> {
    DEFINITIONS
        .iter()
        .map(|d| {
            let cmd = COMMAND_MAP
                .iter()
                .find(|(k, _)| *k == d.name)
                .map(|(_, v)| v.to_string())
                .unwrap_or_else(|| d.name.to_string());
            (cmd, d.description.to_string())
        })
        .collect()
}

/// Default value for a keybind name.
pub fn default_value(name: &str) -> Option<&'static DefaultBinding> {
    DEFINITIONS
        .iter()
        .find(|d| d.name == name)
        .map(|d| &d.default)
}

/// Unknown keybind names in an overrides map.
pub fn unknown_keys(input: &HashMap<String, BindingValue>) -> Vec<String> {
    input
        .keys()
        .filter(|k| !DEFINITIONS.iter().any(|d| d.name == k.as_str()))
        .cloned()
        .collect()
}

/// Parse keybind overrides into a full config map.
pub fn parse(
    overrides: &HashMap<String, BindingValue>,
) -> Result<HashMap<String, BindingValue>, String> {
    let invalid = unknown_keys(overrides);
    if !invalid.is_empty() {
        let suffix = if invalid.len() == 1 { "" } else { "s" };
        return Err(format!(
            "Unrecognized keybind{}: {}",
            suffix,
            invalid.join(", ")
        ));
    }
    let mut result = HashMap::new();
    for def in DEFINITIONS {
        let value = overrides
            .get(def.name)
            .cloned()
            .unwrap_or_else(|| default_binding_value(&def.default));
        result.insert(def.name.to_string(), value);
    }
    Ok(result)
}

/// Convert a keybind config to a binding config (identity in source).
pub fn to_binding_config(
    keybinds: &HashMap<String, BindingValue>,
) -> HashMap<String, BindingValue> {
    keybinds.clone()
}

/// Binding defaults callback (mirrors `bindingDefaults`).
pub fn binding_defaults() -> impl Fn(&str, &Binding) -> Option<String> {
    let descs = command_descriptions();
    move |command: &str, binding: &Binding| {
        if binding.desc.is_some() {
            return None;
        }
        descs.get(command).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definitions_count_matches_source() {
        assert_eq!(DEFINITIONS.len(), 183);
    }

    #[test]
    fn command_map_count_matches_source() {
        assert_eq!(COMMAND_MAP.len(), 163);
    }

    #[test]
    fn leader_default() {
        assert_eq!(LEADER_DEFAULT, "ctrl+x");
    }

    #[test]
    fn parse_empty_gives_defaults() {
        let result = parse(&HashMap::new()).unwrap();
        assert_eq!(result.len(), 183);
        assert_eq!(
            result.get("app_exit").unwrap(),
            &BindingValue::Item(BindingItem::Key("ctrl+c,ctrl+d,<leader>q".to_string()))
        );
    }

    #[test]
    fn parse_rejects_unknown_keys() {
        let mut overrides = HashMap::new();
        overrides.insert("nonexistent_key".to_string(), BindingValue::None);
        let err = parse(&overrides).unwrap_err();
        assert!(err.contains("Unrecognized keybind"));
        assert!(err.contains("nonexistent_key"));
    }

    #[test]
    fn parse_applies_overrides() {
        let mut overrides = HashMap::new();
        overrides.insert(
            "session_list".to_string(),
            BindingValue::Item(BindingItem::Key("ctrl+l".to_string())),
        );
        let result = parse(&overrides).unwrap();
        assert_eq!(
            result.get("session_list").unwrap(),
            &BindingValue::Item(BindingItem::Key("ctrl+l".to_string()))
        );
    }

    #[test]
    fn lookup_maps_commands() {
        let mut config = HashMap::new();
        config.insert(
            "session_move".to_string(),
            BindingValue::Item(BindingItem::Key("ctrl+o".to_string())),
        );
        let lookup = BindingLookup::new(&config);
        assert!(lookup.has("session.move"));
        assert!(!lookup.has("session.list"));
        let bindings = lookup.get("session.move");
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings[0].key, BindingKey::Str("ctrl+o".to_string()));
    }

    #[test]
    fn lookup_none_produces_no_bindings() {
        let mut config = HashMap::new();
        config.insert("terminal_suspend".to_string(), BindingValue::None);
        let lookup = BindingLookup::new(&config);
        assert!(!lookup.has("terminal.suspend"));
    }

    #[test]
    fn default_value_input_undo() {
        let dv = default_value("input_undo").unwrap();
        assert_eq!(dv, &DefaultBinding::Str("ctrl+-,super+z"));
    }

    #[test]
    fn default_value_input_paste_is_object() {
        let dv = default_value("input_paste").unwrap();
        assert_eq!(dv, &DefaultBinding::Paste);
        match default_binding_value(dv) {
            BindingValue::Item(BindingItem::Object(obj)) => {
                assert_eq!(obj.key, BindingKey::Str("ctrl+v".to_string()));
                assert_eq!(obj.prevent_default, Some(false));
            }
            _ => panic!("expected object default"),
        }
    }

    #[test]
    fn unknown_keys_filters() {
        let mut input = HashMap::new();
        input.insert("bad_key".to_string(), BindingValue::None);
        input.insert("app_exit".to_string(), BindingValue::None);
        let unknown = unknown_keys(&input);
        assert_eq!(unknown, vec!["bad_key"]);
    }
}
