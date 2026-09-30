//! Rust port of `src/logo.ts` (opencode v1.18.30).
//!
//! Verbatim ASCII art. `marks` lists the block characters the renderer
//! recolors per row.
//!
//! Original file: `packages/tui/src/logo.ts`

/// One ASCII logo: four text rows, left and right halves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Logo {
    pub left: [&'static str; 4],
    pub right: [&'static str; 4],
}

/// Mirrors `export const logo`.
pub const LOGO: Logo = Logo {
    left: [
        "                   ",
        "█▀▀█ █▀▀█ █▀▀█ █▀▀▄",
        "█__█ █__█ █^^^ █__█",
        "▀▀▀▀ █▀▀▀ ▀▀▀▀ ▀~~▀",
    ],
    right: [
        "             ▄     ",
        "█▀▀▀ █▀▀█ █▀▀█ █▀▀█",
        "█___ █__█ █__█ █^^^",
        "▀▀▀▀ ▀▀▀▀ ▀▀▀▀ ▀▀▀▀",
    ],
};

/// Mirrors `export const go`.
pub const GO: Logo = Logo {
    left: ["    ", "█▀▀▀", "█_^█", "▀▀▀▀"],
    right: ["    ", "█▀▀█", "█__█", "▀▀▀▀"],
};

/// Mirrors `export const marks`.
pub const MARKS: &str = "_^~,";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logos_have_four_rows_each() {
        assert_eq!(LOGO.left.len(), 4);
        assert_eq!(LOGO.right.len(), 4);
        assert_eq!(GO.left.len(), 4);
        assert_eq!(GO.right.len(), 4);
    }

    #[test]
    fn logo_rows_match_source() {
        assert_eq!(LOGO.left[1], "█▀▀█ █▀▀█ █▀▀█ █▀▀▄");
        assert_eq!(LOGO.right[0], "             ▄     ");
        assert_eq!(GO.left[2], "█_^█");
        assert_eq!(GO.right[1], "█▀▀█");
    }

    #[test]
    fn marks_are_exact() {
        assert_eq!(MARKS, "_^~,");
    }
}
