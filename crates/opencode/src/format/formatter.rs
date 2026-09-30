// source: src/format/formatter.ts — exports: Context, Info, gofmt, mix,
// prettier, oxfmt, biome, zig, clang, ktlint, ruff, rlang, uvformat, rubocop,
// standardrb, htmlbeautifier, dart, ocamlformat, terraform, latexindent, gleam,
// shfmt, nixfmt, rustfmt, pint, ormolu, cljfmt, dfmt
// PROVISIONAL pending crates/core (npm, util/which) + @/util/*: Info table
// (names, env, extensions, command vectors, config-file probes) verbatim.

use serde::{Deserialize, Serialize};

/// source: Context { directory, worktree, experimentalOxfmt } — verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub directory: String,
    pub worktree: String,
    pub experimental_oxfmt: bool,
}

/// source: Info { name, environment?, extensions, enabled } — verbatim.
// code-only table (enabled() is a function, not wire) — no Serialize/Deserialize; keep &'static consts verbatim per source
#[derive(Debug, Clone)]
pub struct Info {
    pub name: &'static str,
    pub environment: Option<&'static [(&'static str, &'static str)]>,
    pub extensions: &'static [&'static str],
}

/// source: BUN_BE_BUN env — verbatim.
pub const BUN_BE_BUN: (&str, &str) = ("BUN_BE_BUN", "1");

/// source: per-formatter argv tails (`$FILE` placeholder) — verbatim.
pub fn argv(name: &str) -> Option<&'static [&'static str]> {
    Some(match name {
        "gofmt" => &["-w", "$FILE"],
        "mix" => &["format", "$FILE"],
        "prettier" => &["--write", "$FILE"],
        "oxfmt" => &["$FILE"],
        "biome" => &["format", "--write", "$FILE"],
        "zig" => &["fmt", "$FILE"],
        "clang-format" => &["-i", "$FILE"],
        "ktlint" => &["-F", "$FILE"],
        "ruff" => &["format", "$FILE"],
        "air" => &["format", "$FILE"],
        "uv" => &["format", "--", "$FILE"],
        "rubocop" => &["--autocorrect", "$FILE"],
        "standardrb" => &["--fix", "$FILE"],
        "htmlbeautifier" => &["$FILE"],
        "dart" => &["format", "$FILE"],
        "ocamlformat" => &["-i", "$FILE"],
        "terraform" => &["fmt", "$FILE"],
        "latexindent" => &["-w", "-s", "$FILE"],
        "gleam" => &["format", "$FILE"],
        "shfmt" => &["-w", "$FILE"],
        "nixfmt" => &["$FILE"],
        "rustfmt" => &["$FILE"],
        "pint" => &["$FILE"],
        "ormolu" => &["-i", "$FILE"],
        "cljfmt" => &["fix", "--quiet", "$FILE"],
        "dfmt" => &["-i", "$FILE"],
        _ => return None,
    })
}

/// source: formatter table in source order — verbatim names + extensions.
pub const FORMATTERS: &[Info] = &[
    Info {
        name: "gofmt",
        environment: None,
        extensions: &[".go"],
    },
    Info {
        name: "mix",
        environment: None,
        extensions: &[".ex", ".exs", ".eex", ".heex", ".leex", ".neex", ".sface"],
    },
    Info {
        name: "prettier",
        environment: Some(&[BUN_BE_BUN]),
        extensions: &[
            ".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx", ".mts", ".cts", ".html", ".htm", ".css",
            ".scss", ".sass", ".less", ".vue", ".svelte", ".json", ".jsonc", ".yaml", ".yml",
            ".toml", ".xml", ".md", ".mdx", ".graphql", ".gql",
        ],
    },
    Info {
        name: "oxfmt",
        environment: Some(&[BUN_BE_BUN]),
        extensions: &[".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx", ".mts", ".cts"],
    },
    Info {
        name: "biome",
        environment: Some(&[BUN_BE_BUN]),
        extensions: &[
            ".js", ".jsx", ".mjs", ".cjs", ".ts", ".tsx", ".mts", ".cts", ".html", ".htm", ".css",
            ".scss", ".sass", ".less", ".vue", ".svelte", ".json", ".jsonc", ".yaml", ".yml",
            ".toml", ".xml", ".md", ".mdx", ".graphql", ".gql",
        ],
    },
    Info {
        name: "zig",
        environment: None,
        extensions: &[".zig", ".zon"],
    },
    Info {
        name: "clang-format",
        environment: None,
        extensions: &[
            ".c", ".cc", ".cpp", ".cxx", ".c++", ".h", ".hh", ".hpp", ".hxx", ".h++", ".ino", ".C",
            ".H",
        ],
    },
    Info {
        name: "ktlint",
        environment: None,
        extensions: &[".kt", ".kts"],
    },
    Info {
        name: "ruff",
        environment: None,
        extensions: &[".py", ".pyi"],
    },
    Info {
        name: "air",
        environment: None,
        extensions: &[".R"],
    },
    Info {
        name: "uv",
        environment: None,
        extensions: &[".py", ".pyi"],
    },
    Info {
        name: "rubocop",
        environment: None,
        extensions: &[".rb", ".rake", ".gemspec", ".ru"],
    },
    Info {
        name: "standardrb",
        environment: None,
        extensions: &[".rb", ".rake", ".gemspec", ".ru"],
    },
    Info {
        name: "htmlbeautifier",
        environment: None,
        extensions: &[".erb", ".html.erb"],
    },
    Info {
        name: "dart",
        environment: None,
        extensions: &[".dart"],
    },
    Info {
        name: "ocamlformat",
        environment: None,
        extensions: &[".ml", ".mli"],
    },
    Info {
        name: "terraform",
        environment: None,
        extensions: &[".tf", ".tfvars"],
    },
    Info {
        name: "latexindent",
        environment: None,
        extensions: &[".tex"],
    },
    Info {
        name: "gleam",
        environment: None,
        extensions: &[".gleam"],
    },
    Info {
        name: "shfmt",
        environment: None,
        extensions: &[".sh", ".bash"],
    },
    Info {
        name: "nixfmt",
        environment: None,
        extensions: &[".nix"],
    },
    Info {
        name: "rustfmt",
        environment: None,
        extensions: &[".rs"],
    },
    Info {
        name: "pint",
        environment: None,
        extensions: &[".php"],
    },
    Info {
        name: "ormolu",
        environment: None,
        extensions: &[".hs"],
    },
    Info {
        name: "cljfmt",
        environment: None,
        extensions: &[".clj", ".cljs", ".cljc", ".edn"],
    },
    Info {
        name: "dfmt",
        environment: None,
        extensions: &[".d"],
    },
];

/// source: config-file probes per formatter — verbatim.
pub fn config_probes(name: &str) -> &'static [&'static str] {
    match name {
        "biome" => &["biome.json", "biome.jsonc"],
        "ruff" => &["pyproject.toml", "ruff.toml", ".ruff.toml"],
        "clang-format" => &[".clang-format"],
        "ocamlformat" => &[".ocamlformat"],
        _ => &[],
    }
}

/// source: "[tool.ruff]" pyproject marker — verbatim.
pub const RUFF_PYPROJECT_MARKER: &str = "[tool.ruff]";
/// source: "./vendor/bin/pint" — verbatim.
pub const PINT_BIN: &str = "./vendor/bin/pint";
/// source: pint composer markers — verbatim.
pub const PINT_COMPOSER_KEYS: &[&str] = &["require", "require-dev"];
pub const PINT_PACKAGE: &str = "laravel/pint";
/// source: rlang help gate — "R language" + "formatter" on first line, code 0. Verbatim.
pub const RLANG_HELP_R: &str = "R language";
pub const RLANG_HELP_FORMATTER: &str = "formatter";
/// source: ruff/uv linked disable — disabling either disables both. Verbatim.
pub const RUFF_LINKED: &[&str] = &["ruff", "uv"];
/// source: "$FILE" placeholder — verbatim.
pub const FILE_PLACEHOLDER: &str = "$FILE";
