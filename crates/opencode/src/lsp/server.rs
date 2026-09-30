// source: src/lsp/server.ts — exports: Handle, Info, Deno, Typescript, Vue,
// ESLint, Oxlint, Biome, Gopls, Rubocop, Ty, Pyright, ElixirLS, Zls, CSharp,
// Razor, FSharp, SourceKit, RustAnalyzer, Clangd, Svelte, Astro, JDTLS,
// KotlinLS, YamlLS, LuaLS, PHPIntelephense, Prisma, Dart, Ocaml, BashLS,
// TerraformLS, TexLab, DockerfileLS, Gleam, Clojure, Nixd, Tinymist, HLS, JuliaLS
// PROVISIONAL (1983-line server table): Info shape { id, extensions,
// global?, root, spawn }, NearestRoot vs StrictNearestRoot fallback rules
// (missing → directory vs undefined), server id table verbatim.

use serde::{Deserialize, Serialize};

/// source: Info { id, extensions, global?, root, spawn } — verbatim shape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub id: String,
    pub extensions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
}

/// source: server id table — verbatim ids in source order.
pub const SERVER_IDS: &[&str] = &[
    "deno",
    "typescript",
    "vue",
    "eslint",
    "oxlint",
    "biome",
    "gopls",
    "ruby-lsp",
    "ty",
    "pyright",
    "elixir-ls",
    "zls",
    "csharp",
    "razor",
    "fsharp",
    "sourcekit-lsp",
    "rust",
    "clangd",
    "svelte",
    "astro",
    "jdtls",
    "kotlin-ls",
    "yaml-ls",
    "lua-ls",
    "php intelephense",
    "prisma",
    "dart",
    "ocaml-lsp",
    "bash",
    "terraform",
    "texlab",
    "dockerfile",
    "gleam",
    "clojure-lsp",
    "nixd",
    "tinymist",
    "haskell-language-server",
    "julials",
];

/// source: NearestRoot — excluded → undefined; missing → ctx.directory. Verbatim.
pub const NEAREST_MISSING_FALLBACK: &str = "directory";
/// source: StrictNearestRoot — missing → undefined. Verbatim.
pub const STRICT_MISSING_FALLBACK: &str = "undefined";
