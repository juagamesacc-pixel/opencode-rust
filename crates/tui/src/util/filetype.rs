// source: packages/tui/src/util/filetype.ts (130 lines, v1.18.30)
// 1:1 port — extension table verbatim (order + odd entries preserved).

#![allow(dead_code)]

/// Mirrors `LANGUAGE_EXTENSIONS` verbatim, including the odd entries
/// (`.ets` → typescript, bare `makefile` key).
pub const LANGUAGE_EXTENSIONS: &[(&str, &str)] = &[
    (".abap", "abap"),
    (".bat", "bat"),
    (".bib", "bibtex"),
    (".bibtex", "bibtex"),
    (".clj", "clojure"),
    (".cljs", "clojure"),
    (".cljc", "clojure"),
    (".edn", "clojure"),
    (".coffee", "coffeescript"),
    (".c", "c"),
    (".cpp", "cpp"),
    (".cxx", "cpp"),
    (".cc", "cpp"),
    (".c++", "cpp"),
    (".cs", "csharp"),
    (".csx", "csharp"),
    (".css", "css"),
    (".d", "d"),
    (".pas", "pascal"),
    (".pascal", "pascal"),
    (".diff", "diff"),
    (".patch", "diff"),
    (".dart", "dart"),
    (".dockerfile", "dockerfile"),
    (".ex", "elixir"),
    (".exs", "elixir"),
    (".erl", "erlang"),
    (".ets", "typescript"),
    (".hrl", "erlang"),
    (".fs", "fsharp"),
    (".fsi", "fsharp"),
    (".fsx", "fsharp"),
    (".fsscript", "fsharp"),
    (".gitcommit", "git-commit"),
    (".gitrebase", "git-rebase"),
    (".go", "go"),
    (".groovy", "groovy"),
    (".gleam", "gleam"),
    (".hbs", "handlebars"),
    (".handlebars", "handlebars"),
    (".hs", "haskell"),
    (".lhs", "haskell"),
    (".html", "html"),
    (".htm", "html"),
    (".ini", "ini"),
    (".java", "java"),
    (".jl", "julia"),
    (".js", "javascript"),
    (".kt", "kotlin"),
    (".kts", "kotlin"),
    (".jsx", "javascriptreact"),
    (".json", "json"),
    (".tex", "latex"),
    (".latex", "latex"),
    (".less", "less"),
    (".lua", "lua"),
    (".makefile", "makefile"),
    ("makefile", "makefile"),
    (".md", "markdown"),
    (".markdown", "markdown"),
    (".m", "objective-c"),
    (".mm", "objective-cpp"),
    (".pl", "perl"),
    (".pm", "perl"),
    (".pm6", "perl6"),
    (".php", "php"),
    (".ps1", "powershell"),
    (".psm1", "powershell"),
    (".pug", "jade"),
    (".jade", "jade"),
    (".py", "python"),
    (".r", "r"),
    (".cshtml", "razor"),
    (".razor", "razor"),
    (".rb", "ruby"),
    (".rake", "ruby"),
    (".gemspec", "ruby"),
    (".ru", "ruby"),
    (".erb", "erb"),
    (".html.erb", "erb"),
    (".js.erb", "erb"),
    (".css.erb", "erb"),
    (".json.erb", "erb"),
    (".rs", "rust"),
    (".scss", "scss"),
    (".sass", "sass"),
    (".scala", "scala"),
    (".shader", "shaderlab"),
    (".sh", "shellscript"),
    (".bash", "shellscript"),
    (".zsh", "shellscript"),
    (".ksh", "shellscript"),
    (".sql", "sql"),
    (".svelte", "svelte"),
    (".swift", "swift"),
    (".ts", "typescript"),
    (".tsx", "typescriptreact"),
    (".mts", "typescript"),
    (".cts", "typescript"),
    (".mtsx", "typescriptreact"),
    (".ctsx", "typescriptreact"),
    (".xml", "xml"),
    (".xsl", "xsl"),
    (".yaml", "yaml"),
    (".yml", "yaml"),
    (".mjs", "javascript"),
    (".cjs", "javascript"),
    (".vue", "vue"),
    (".zig", "zig"),
    (".zon", "zig"),
    (".astro", "astro"),
    (".ml", "ocaml"),
    (".mli", "ocaml"),
    (".tf", "terraform"),
    (".tfvars", "terraform-vars"),
    (".hcl", "hcl"),
    (".nix", "nix"),
    (".typ", "typst"),
    (".typc", "typst"),
];

fn extname(input: &str) -> &str {
    let base = input.rsplit(['/', '\\']).next().unwrap_or(input);
    match base.rfind('.') {
        Some(0) | None => "",
        Some(dot) => &base[dot..],
    }
}

/// Mirrors `filetype` — `None`/empty → `"none"`; react/js flavors collapse
/// to `"typescript"`; unknown extensions stay unknown (`None`).
pub fn filetype(input: Option<&str>) -> Option<&'static str> {
    let input = input?;
    if input.is_empty() {
        return Some("none");
    }
    let language = LANGUAGE_EXTENSIONS
        .iter()
        .find(|(ext, _)| *ext == extname(input))
        .map(|(_, lang)| *lang)?;
    if matches!(
        language,
        "typescriptreact" | "javascriptreact" | "javascript"
    ) {
        return Some("typescript");
    }
    Some(language)
}
