// source: packages/tui/src/util/presentation.ts (38 lines, v1.18.30)
// 1:1 port — ANSI wordmark + session epilogue, escape codes verbatim.

#![allow(dead_code)]

const LOGO_LEFT: [&str; 4] = [
    "                   ",
    "█▀▀█ █▀▀█ █▀▀█ █▀▀▄",
    "█__█ █__█ █^^^ █__█",
    "▀▀▀▀ █▀▀▀ ▀▀▀▀ ▀~~▀",
];

const LOGO_RIGHT: [&str; 4] = [
    "             ▄     ",
    "█▀▀▀ █▀▀█ █▀▀█ █▀▀█",
    "█___ █__█ █__█ █^^^",
    "▀▀▀▀ ▀▀▀▀ ▀▀▀▀ ▀▀▀▀",
];

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[90m";
const SHADOW_LEFT: &str = "\x1b[38;5;235m";
const BG_LEFT: &str = "\x1b[48;5;235m";
const SHADOW_RIGHT: &str = "\x1b[38;5;238m";
const BG_RIGHT: &str = "\x1b[48;5;238m";

fn draw(line: &str, fg: &str, shadow: &str, bg: &str) -> String {
    line.chars()
        .map(|char| match char {
            '_' => format!("{bg} {RESET}"),
            '^' => format!("{fg}{bg}▀{RESET}"),
            '~' => format!("{shadow}▀{RESET}"),
            ' ' => " ".to_string(),
            _ => format!("{fg}{char}{RESET}"),
        })
        .collect::<Vec<_>>()
        .join("")
}

fn wordmark(pad: &str) -> Vec<String> {
    LOGO_LEFT
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let left = draw(line, DIM, SHADOW_LEFT, BG_LEFT);
            let right = draw(
                LOGO_RIGHT.get(index).copied().unwrap_or(""),
                RESET,
                SHADOW_RIGHT,
                BG_RIGHT,
            );
            format!("{pad}{left} {right}")
        })
        .collect()
}

fn pad_end_10(text: &str) -> String {
    let width: usize = text.chars().count();
    if width >= 10 {
        text.to_string()
    } else {
        text.to_string() + &" ".repeat(10 - width)
    }
}

/// Mirrors `sessionEpilogue`.
pub fn session_epilogue(title: &str, session_id: Option<&str>) -> String {
    let mut lines = wordmark("  ");
    lines.push(String::new());
    lines.push(format!(
        "  {DIM}{}{RESET}{BOLD}{title}{RESET}",
        pad_end_10("Session")
    ));
    lines.push(format!(
        "  {DIM}{}{RESET}{BOLD}opencode -s {}{RESET}",
        pad_end_10("Continue"),
        session_id.unwrap_or_default()
    ));
    lines.push(String::new());
    lines.join("\n")
}
