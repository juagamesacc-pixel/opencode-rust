// source: packages/tui/src/parsers-config.ts (386 lines, v1.18.30)
// 1:1 port — tree-sitter parser table verbatim (URLs, aliases, and the
// commented-out entries preserved as data comments). Markdown,
// javascript and typescript use the renderer built-ins (no entries).

#![allow(dead_code)]

/// One query-file list per role.
#[derive(Debug, Clone, Default)]
pub struct ParserQueries {
    pub highlights: &'static [&'static str],
    pub locals: &'static [&'static str],
}

/// One parser entry (mirrors the config objects verbatim).
#[derive(Debug, Clone)]
pub struct ParserEntry {
    pub filetype: &'static str,
    pub aliases: &'static [&'static str],
    pub wasm: &'static str,
    pub queries: ParserQueries,
}

/// Parser table verbatim (order preserved).
pub const PARSERS: &[ParserEntry] = &[
    ParserEntry {
        filetype: "python",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-python/releases/download/v0.23.6/tree-sitter-python.wasm",
        queries: ParserQueries {
            // NOTE: the nvim-treesitter python highlights query is broken
            // with this parser (stale "except" nodes) — intentionally omitted.
            highlights: &[
                "https://github.com/tree-sitter/tree-sitter-python/raw/refs/heads/master/queries/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/python/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "rust",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-rust/releases/download/v0.24.0/tree-sitter-rust.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/rust/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/rust/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "go",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-go/releases/download/v0.25.0/tree-sitter-go.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/go/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "cpp",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-cpp/releases/download/v0.23.4/tree-sitter-cpp.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/cpp/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "csharp",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-c-sharp/releases/download/v0.23.1/tree-sitter-c_sharp.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/c_sharp/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "bash",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-bash/releases/download/v0.25.0/tree-sitter-bash.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/bash/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "c",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-c/releases/download/v0.24.1/tree-sitter-c.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/c/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/c/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "java",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-java/releases/download/v0.23.5/tree-sitter-java.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/java/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "kotlin",
        aliases: &[],
        wasm: "https://github.com/fwcd/tree-sitter-kotlin/releases/download/0.3.8/tree-sitter-kotlin.wasm",
        queries: ParserQueries {
            highlights: &["https://raw.githubusercontent.com/fwcd/tree-sitter-kotlin/0.3.8/queries/highlights.scm"],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/master/queries/kotlin/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "ruby",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-ruby/releases/download/v0.23.1/tree-sitter-ruby.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/ruby/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/ruby/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "php",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-php/releases/download/v0.24.2/tree-sitter-php.wasm",
        queries: ParserQueries {
            highlights: &[
                // NOTE: nvim-treesitter php highlights are broken with this
                // parser — intentionally omitted.
                "https://github.com/tree-sitter/tree-sitter-php/raw/refs/heads/master/queries/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "scala",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-scala/releases/download/v0.24.0/tree-sitter-scala.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/scala/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "html",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-html/releases/download/v0.23.2/tree-sitter-html.wasm",
        queries: ParserQueries {
            highlights: &[
                // NOTE: nvim-treesitter html highlights are broken with this
                // parser — intentionally omitted.
                "https://github.com/tree-sitter/tree-sitter-html/raw/refs/heads/master/queries/highlights.scm",
                // TODO: injections not working:
                // "https://github.com/tree-sitter/tree-sitter-html/raw/refs/heads/master/queries/injections.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "vue",
        aliases: &[],
        wasm: "https://github.com/anomalyco/tree-sitter-vue/releases/download/v0.1.2/tree-sitter-vue.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/anomalyco/tree-sitter-vue/v0.1.2/queries/html_tags/highlights.scm",
                "https://raw.githubusercontent.com/anomalyco/tree-sitter-vue/v0.1.2/queries/vue/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "hcl",
        aliases: &[],
        wasm: "https://github.com/tree-sitter-grammars/tree-sitter-hcl/releases/download/v1.2.0/tree-sitter-hcl.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/master/queries/hcl/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "json",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-json/releases/download/v0.24.8/tree-sitter-json.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/json/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "yaml",
        aliases: &[],
        wasm: "https://github.com/tree-sitter-grammars/tree-sitter-yaml/releases/download/v0.7.2/tree-sitter-yaml.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/yaml/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "haskell",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-haskell/releases/download/v0.23.1/tree-sitter-haskell.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/haskell/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "css",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-css/releases/download/v0.25.0/tree-sitter-css.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/css/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "julia",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-julia/releases/download/v0.23.1/tree-sitter-julia.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/julia/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "lua",
        aliases: &[],
        wasm: "https://github.com/tree-sitter-grammars/tree-sitter-lua/releases/download/v0.5.0/tree-sitter-lua.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-lua/v0.5.0/queries/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-lua/v0.5.0/queries/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "ocaml",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-ocaml/releases/download/v0.24.2/tree-sitter-ocaml.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/ocaml/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "clojure",
        aliases: &[],
        // Temporarily using a fork to fix issues.
        wasm: "https://github.com/anomalyco/tree-sitter-clojure/releases/download/v0.0.1/tree-sitter-clojure.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/clojure/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "swift",
        aliases: &[],
        wasm: "https://github.com/alex-pinkus/tree-sitter-swift/releases/download/0.7.1/tree-sitter-swift.wasm",
        queries: ParserQueries {
            highlights: &[
                // NOTE: parser-repo queries (nvim-treesitter uses
                // incompatible #lua-match? predicates) — intentionally used.
                "https://raw.githubusercontent.com/alex-pinkus/tree-sitter-swift/main/queries/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/swift/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "toml",
        aliases: &[],
        wasm: "https://github.com/tree-sitter-grammars/tree-sitter-toml/releases/download/v0.7.0/tree-sitter-toml.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/master/queries/toml/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "nix",
        aliases: &[],
        // TODO: replace with the official tree-sitter-nix WASM when
        // published (nix-community/tree-sitter-nix#66).
        wasm: "https://github.com/ast-grep/ast-grep.github.io/raw/40b84530640aa83a0d34a20a2b0623d7b8e5ea97/website/public/parsers/tree-sitter-nix.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/nix/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/nix/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "diff",
        aliases: &["udiff", "patch"],
        wasm: "https://github.com/tree-sitter-grammars/tree-sitter-diff/releases/download/v0.1.0/tree-sitter-diff.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/tree-sitter-grammars/tree-sitter-diff/2520c3f934b3179bb540d23e0ef45f75304b5fed/queries/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "elixir",
        aliases: &[],
        wasm: "https://github.com/elixir-lang/tree-sitter-elixir/releases/download/v0.3.5/tree-sitter-elixir.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/elixir/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/elixir/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "fsharp",
        aliases: &[],
        wasm: "https://github.com/ionide/tree-sitter-fsharp/releases/download/0.3.0/tree-sitter-fsharp.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/fsharp/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "r",
        aliases: &[],
        wasm: "https://github.com/r-lib/tree-sitter-r/releases/download/v1.2.0/tree-sitter-r.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/r/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/r/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "make",
        aliases: &["makefile"],
        wasm: "https://github.com/tree-sitter-grammars/tree-sitter-make/releases/download/v1.1.1/tree-sitter-make.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/make/highlights.scm",
            ],
            locals: &[],
        },
    },
    ParserEntry {
        filetype: "vim",
        aliases: &[],
        wasm: "https://github.com/tree-sitter-grammars/tree-sitter-vim/releases/download/v0.8.1/tree-sitter-vim.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/vim/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/vim/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "xml",
        aliases: &[],
        wasm: "https://github.com/tree-sitter-grammars/tree-sitter-xml/releases/download/v0.7.0/tree-sitter-xml.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/xml/highlights.scm",
            ],
            locals: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/xml/locals.scm",
            ],
        },
    },
    ParserEntry {
        filetype: "agda",
        aliases: &[],
        wasm: "https://github.com/tree-sitter/tree-sitter-agda/releases/download/v1.3.3/tree-sitter-agda.wasm",
        queries: ParserQueries {
            highlights: &[
                "https://raw.githubusercontent.com/nvim-treesitter/nvim-treesitter/refs/heads/master/queries/agda/highlights.scm",
            ],
            locals: &[],
        },
    },
];

/// Lookup by filetype or alias.
pub fn parser_for(filetype: &str) -> Option<&'static ParserEntry> {
    PARSERS
        .iter()
        .find(|entry| entry.filetype == filetype || entry.aliases.contains(&filetype))
}
