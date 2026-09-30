//! Rust port of `src/keymap.tsx` (opencode v1.18.30).
//!
//! The mode stack, key aliases, and input-command table are engine-free and
//! ported here in full. The `@opentui/keymap` engine surface
//! (`KeymapProvider`, binding expanders, timed leader, managed textarea
//! layer, Solid selectors) has no Rust equivalent in the dependency set;
//! those registrations land with the prompt/routes turn against an explicit
//! engine seam. `TuiKeybind.LeaderDefault` (from `config/keybind`, unported)
//! gates the leader display helpers, which wait with it.
//!
//! Original file: `packages/tui/src/keymap.tsx`

pub const LEADER_TOKEN: &str = "leader";
pub const OPENCODE_BASE_MODE: &str = "base";
pub const COMMAND_PALETTE_COMMAND: &str = "command.palette.show";

pub const OPENCODE_MODE_KEY: &str = "opencode.mode";

/// Mirrors `KEY_ALIASES` (alias → key).
pub const KEY_ALIASES: [(&str, &str); 4] = [
    ("enter", "return"),
    ("esc", "escape"),
    ("pgdown", "pagedown"),
    ("pgup", "pageup"),
];

fn is_boundary_before(previous: Option<char>) -> bool {
    match previous {
        // `(^|[+,\s>])`
        None => true,
        Some(value) => value == '+' || value == ',' || value.is_whitespace() || value == '>',
    }
}

fn is_boundary_after(next: Option<char>) -> bool {
    match next {
        // `(?=$|[+,\s<])`
        None => true,
        Some(value) => value == '+' || value == ',' || value.is_whitespace() || value == '<',
    }
}

fn expand_one(input: &str, alias: &str, key: &str) -> String {
    // Case-insensitive left-to-right rewrite, like `String.replace` with the
    // `gi` flags. Aliases are ASCII, so byte length equals char length.
    let chars: Vec<char> = input.chars().collect();
    let mut out = String::with_capacity(input.len());
    let mut index = 0;
    while index < chars.len() {
        let rest: String = chars[index..].iter().collect();
        let matches = rest.len() >= alias.len()
            && rest[..alias.len()].eq_ignore_ascii_case(alias)
            && is_boundary_before(if index == 0 {
                None
            } else {
                Some(chars[index - 1])
            })
            && is_boundary_after(chars.get(index + alias.len()).copied());
        if matches {
            out.push_str(key);
            index += alias.len();
        } else {
            out.push(chars[index]);
            index += 1;
        }
    }
    out
}

/// Mirrors `expandKeyAliases`: rewrite every alias, or `None` when the input
/// is unchanged.
pub fn expand_key_aliases(input: &str) -> Option<String> {
    let mut result = input.to_string();
    for (alias, key) in KEY_ALIASES {
        result = expand_one(&result, alias, key);
    }
    if result == input {
        return None;
    }
    Some(result)
}

/// Mirrors `inputCommands`: commands captured by the managed textarea layer.
pub const INPUT_COMMANDS: [&str; 36] = [
    "input.move.left",
    "input.move.right",
    "input.move.up",
    "input.move.down",
    "input.select.left",
    "input.select.right",
    "input.select.up",
    "input.select.down",
    "input.line.home",
    "input.line.end",
    "input.select.line.home",
    "input.select.line.end",
    "input.visual.line.home",
    "input.visual.line.end",
    "input.select.visual.line.home",
    "input.select.visual.line.end",
    "input.buffer.home",
    "input.buffer.end",
    "input.select.buffer.home",
    "input.select.buffer.end",
    "input.delete.line",
    "input.delete.to.line.end",
    "input.delete.to.line.start",
    "input.backspace",
    "input.delete",
    "input.newline",
    "input.undo",
    "input.redo",
    "input.word.forward",
    "input.word.backward",
    "input.select.word.forward",
    "input.select.word.backward",
    "input.delete.word.forward",
    "input.delete.word.backward",
    "input.select.all",
    "input.submit",
];

/// The fields of a palette command `isVisiblePaletteCommand` reads.
pub struct PaletteCommandRef {
    pub name: String,
    pub hidden: bool,
}

/// Mirrors `isVisiblePaletteCommand`.
pub fn is_visible_palette_command(command: &PaletteCommandRef) -> bool {
    !command.hidden && command.name != COMMAND_PALETTE_COMMAND
}

// ---------------------------------------------------------------------------
// Leader display + format helpers (engine seam)
// ---------------------------------------------------------------------------

/// Format options for key sequence display (mirrors `formatOptions`).
#[derive(Debug, Clone)]
pub struct FormatOptions {
    pub token_display: std::collections::HashMap<String, String>,
    pub key_name_aliases: std::collections::HashMap<String, String>,
    pub modifier_aliases: std::collections::HashMap<String, String>,
}

/// The leader key display string (mirrors `leaderDisplay`).
pub fn leader_display(config: &crate::config::keybind::BindingLookup) -> String {
    let bindings = config.get(LEADER_TOKEN);
    let key = bindings.first().map(|b| &b.key);
    match key {
        Some(crate::config::keybind::BindingKey::Str(s)) => s.clone(),
        Some(crate::config::keybind::BindingKey::Stroke(stroke)) => stroke_name(stroke),
        None => crate::config::keybind::LEADER_DEFAULT.to_string(),
    }
}

/// The leader key binding (mirrors `leaderKey`).
pub fn leader_key(
    config: &crate::config::keybind::BindingLookup,
) -> Option<crate::config::keybind::BindingKey> {
    config.get(LEADER_TOKEN).first().map(|b| b.key.clone())
}

/// Build format options from a config (mirrors `formatOptions`).
pub fn format_options(config: &crate::config::keybind::BindingLookup) -> FormatOptions {
    let mut token_display = std::collections::HashMap::new();
    token_display.insert(LEADER_TOKEN.to_string(), leader_display(config));
    let mut key_name_aliases = std::collections::HashMap::new();
    key_name_aliases.insert("pageup".to_string(), "pgup".to_string());
    key_name_aliases.insert("pagedown".to_string(), "pgdn".to_string());
    key_name_aliases.insert("delete".to_string(), "del".to_string());
    let mut modifier_aliases = std::collections::HashMap::new();
    modifier_aliases.insert("meta".to_string(), "alt".to_string());
    FormatOptions {
        token_display,
        key_name_aliases,
        modifier_aliases,
    }
}

/// Format a key sequence for display (mirrors `formatKeySequence`).
pub fn format_key_sequence(
    parts: &[crate::config::keybind::KeyStroke],
    config: &crate::config::keybind::BindingLookup,
) -> String {
    let opts = format_options(config);
    parts
        .iter()
        .map(|stroke| format_stroke(stroke, &opts))
        .collect::<Vec<_>>()
        .join("")
}

/// Format key bindings for display (mirrors `formatKeyBindings`).
pub fn format_key_bindings(
    bindings: &[crate::config::keybind::Binding],
    config: &crate::config::keybind::BindingLookup,
) -> String {
    let opts = format_options(config);
    bindings
        .iter()
        .map(|b| match &b.key {
            crate::config::keybind::BindingKey::Str(s) => s.clone(),
            crate::config::keybind::BindingKey::Stroke(stroke) => format_stroke(stroke, &opts),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn stroke_name(stroke: &crate::config::keybind::KeyStroke) -> String {
    let mut parts = Vec::new();
    if stroke.ctrl == Some(true) {
        parts.push("ctrl");
    }
    if stroke.shift == Some(true) {
        parts.push("shift");
    }
    if stroke.meta == Some(true) {
        parts.push("meta");
    }
    if stroke.super_ == Some(true) {
        parts.push("super");
    }
    if stroke.hyper == Some(true) {
        parts.push("hyper");
    }
    parts.push(&stroke.name);
    parts.join("+")
}

fn format_stroke(stroke: &crate::config::keybind::KeyStroke, opts: &FormatOptions) -> String {
    let mut parts = Vec::new();
    if stroke.ctrl == Some(true) {
        parts.push("ctrl".to_string());
    }
    if stroke.shift == Some(true) {
        parts.push("shift".to_string());
    }
    if stroke.meta == Some(true) {
        parts.push(
            opts.modifier_aliases
                .get("meta")
                .cloned()
                .unwrap_or_else(|| "meta".to_string()),
        );
    }
    if stroke.super_ == Some(true) {
        parts.push("super".to_string());
    }
    if stroke.hyper == Some(true) {
        parts.push("hyper".to_string());
    }
    let name = opts
        .key_name_aliases
        .get(&stroke.name)
        .cloned()
        .unwrap_or_else(|| stroke.name.clone());
    parts.push(name);
    parts.join("+")
}

struct StackEntry {
    id: u64,
    mode: String,
}

/// Mirrors `createOpencodeModeStack`: a mode stack whose top wins, falling
/// back to [`OPENCODE_BASE_MODE`].
///
/// The source returns a removal closure per `push`; Rust cannot return a
/// self-borrowing closure, so `push` returns an id for [`Self::remove`]
/// (`None` once disposed, like the source's no-op disposer). Mode writes go
/// to `on_update` (the source's `keymap.setData(OPENCODE_MODE_KEY, …)`);
/// `dispose` additionally runs `on_unregister` once (the source's
/// `offFields` + registry delete).
pub struct OpencodeModeStack {
    entries: Vec<StackEntry>,
    next_id: u64,
    disposed: bool,
    on_update: Box<dyn FnMut(String)>,
    on_unregister: Option<Box<dyn FnOnce()>>,
}

impl OpencodeModeStack {
    pub fn new(
        on_update: impl FnMut(String) + 'static,
        on_unregister: impl FnOnce() + 'static,
    ) -> Self {
        let mut stack = Self {
            entries: Vec::new(),
            next_id: 1,
            disposed: false,
            on_update: Box::new(on_update),
            on_unregister: Some(Box::new(on_unregister)),
        };
        stack.update();
        stack
    }

    /// The top mode, or [`OPENCODE_BASE_MODE`] when the stack is empty.
    pub fn current(&self) -> String {
        self.entries
            .last()
            .map(|entry| entry.mode.clone())
            .unwrap_or_else(|| OPENCODE_BASE_MODE.to_string())
    }

    /// Push `mode`, returning its id (`None` once disposed).
    pub fn push(&mut self, mode: &str) -> Option<u64> {
        if self.disposed {
            return None;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.entries.push(StackEntry {
            id,
            mode: mode.to_string(),
        });
        self.update();
        Some(id)
    }

    /// Remove the entry pushed as `id` (the source's per-push disposer).
    pub fn remove(&mut self, id: u64) {
        if let Some(index) = self.entries.iter().position(|entry| entry.id == id) {
            self.entries.remove(index);
            self.update();
        }
    }

    /// Clear the stack, run the unregister hook once, and freeze pushes.
    pub fn dispose(&mut self) {
        if self.disposed {
            return;
        }
        self.disposed = true;
        self.entries.clear();
        if let Some(on_unregister) = self.on_unregister.take() {
            on_unregister();
        }
        self.update();
    }

    fn update(&mut self) {
        let current = self.current();
        (self.on_update)(current);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn aliases_expand_with_source_boundaries() {
        assert_eq!(expand_key_aliases("enter"), Some("return".to_string()));
        assert_eq!(
            expand_key_aliases("ctrl+enter"),
            Some("ctrl+return".to_string())
        );
        assert_eq!(expand_key_aliases("ENTER"), Some("return".to_string()));
        // The leading-class `>` may precede an alias, the trailing class may
        // not: `pgdown>x` is unchanged, `x>pgup` becomes `x>pageup`.
        assert_eq!(expand_key_aliases("pgdown>x"), None);
        assert_eq!(expand_key_aliases("x>pgup"), Some("x>pageup".to_string()));
        assert_eq!(
            expand_key_aliases("enter,esc"),
            Some("return,escape".to_string())
        );
        // `esc` inside `escape` is not alias-adjacent; plain words pass through.
        assert_eq!(expand_key_aliases("escape"), None);
        assert_eq!(expand_key_aliases("prevent"), None);
        assert_eq!(expand_key_aliases("ctrl+a"), None);
    }

    #[test]
    fn input_commands_match_source() {
        assert_eq!(INPUT_COMMANDS.len(), 36);
        assert_eq!(INPUT_COMMANDS[0], "input.move.left");
        assert_eq!(INPUT_COMMANDS[35], "input.submit");
        assert!(INPUT_COMMANDS.contains(&"input.backspace"));
    }

    #[test]
    fn palette_visibility_matches_source() {
        let visible = PaletteCommandRef {
            name: "session.new".to_string(),
            hidden: false,
        };
        assert!(is_visible_palette_command(&visible));
        assert!(!is_visible_palette_command(&PaletteCommandRef {
            name: COMMAND_PALETTE_COMMAND.to_string(),
            hidden: false,
        }));
        assert!(!is_visible_palette_command(&PaletteCommandRef {
            name: "session.new".to_string(),
            hidden: true,
        }));
    }

    #[allow(clippy::type_complexity)]
    fn tracked() -> (
        OpencodeModeStack,
        Rc<RefCell<Vec<String>>>,
        Rc<RefCell<usize>>,
    ) {
        let updates = Rc::new(RefCell::new(Vec::new()));
        let unregistered = Rc::new(RefCell::new(0usize));
        let next_updates = Rc::clone(&updates);
        let next_unregistered = Rc::clone(&unregistered);
        let stack = OpencodeModeStack::new(
            move |mode| next_updates.borrow_mut().push(mode),
            move || *next_unregistered.borrow_mut() += 1,
        );
        (stack, updates, unregistered)
    }

    #[test]
    fn mode_stack_top_wins_and_pops_by_id() {
        let (mut stack, updates, _) = tracked();
        assert_eq!(stack.current(), "base");
        let first = stack.push("dialog").expect("push id");
        assert_eq!(stack.current(), "dialog");
        let second = stack.push("palette").expect("push id");
        assert_eq!(stack.current(), "palette");
        stack.remove(first);
        assert_eq!(stack.current(), "palette");
        stack.remove(second);
        assert_eq!(stack.current(), "base");
        assert_eq!(
            *updates.borrow(),
            vec![
                "base".to_string(),
                "dialog".to_string(),
                "palette".to_string(),
                "palette".to_string(),
                "base".to_string()
            ]
        );
    }

    #[test]
    fn dispose_freezes_and_unregisters_once() {
        let (mut stack, updates, unregistered) = tracked();
        stack.push("dialog");
        stack.dispose();
        assert_eq!(stack.current(), "base");
        assert_eq!(*unregistered.borrow(), 1);
        assert_eq!(stack.push("palette"), None);
        stack.dispose();
        assert_eq!(*unregistered.borrow(), 1);
        assert_eq!(updates.borrow().last().map(String::as_str), Some("base"));
    }
}
