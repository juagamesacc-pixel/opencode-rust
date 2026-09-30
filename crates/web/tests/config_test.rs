// Parity tests for `config.mjs`.
use web::config;

#[test]
fn verbatim_constants_match_source() {
    assert_eq!(config::STAGE_ENV, "SST_STAGE");
    assert_eq!(config::DEFAULT_STAGE, "dev");
    assert_eq!(config::PRODUCTION_STAGE, "production");
    assert_eq!(config::PRODUCTION_URL, "https://opencode.ai");
    assert_eq!(config::PRODUCTION_CONSOLE, "https://opencode.ai/auth");
    assert_eq!(config::EMAIL, "help@anoma.ly");
    assert_eq!(config::SOCIAL_CARD, "https://social-cards.sst.dev");
    assert_eq!(config::GITHUB, "https://github.com/anomalyco/opencode");
    assert_eq!(config::DISCORD, "https://opencode.ai/discord");
    assert_eq!(config::EDIT_LINK_SUFFIX, "/edit/dev/packages/web/");
    assert_eq!(config::HEADER_LINKS.len(), 2);
    assert_eq!(config::HEADER_LINKS[0].name, "app.header.home");
    assert_eq!(config::HEADER_LINKS[0].url, "/");
    assert_eq!(config::HEADER_LINKS[1].name, "app.header.docs");
    assert_eq!(config::HEADER_LINKS[1].url, "/docs/");
}

#[test]
fn dev_and_production_origins_match_source() {
    let dev = config::Config::new();
    assert_eq!(dev, config::Config::default());
    assert_eq!(dev.stage, "dev");
    assert_eq!(dev.url, "https://dev.opencode.ai");
    assert_eq!(dev.console, "https://dev.opencode.ai/auth");

    let production = config::Config::for_stage("production");
    assert_eq!(production.url, "https://opencode.ai");
    assert_eq!(production.console, "https://opencode.ai/auth");

    let preview = config::Config::for_stage("preview");
    assert_eq!(preview.url, "https://preview.opencode.ai");
    assert_eq!(preview.console, "https://preview.opencode.ai/auth");
}

#[test]
fn derived_urls_match_source_pages() {
    let dev = config::Config::new();
    assert_eq!(
        dev.edit_link_base(),
        "https://github.com/anomalyco/opencode/edit/dev/packages/web/"
    );
    assert_eq!(
        dev.docs_social_card("TITLE", "DESC"),
        "https://social-cards.sst.dev/opencode-docs/TITLE.png?desc=DESC"
    );
    assert_eq!(
        dev.share_social_card("TITLE", "MODEL", "v1.0.0", "abc"),
        "https://social-cards.sst.dev/opencode-share/TITLE.png?model=MODEL&version=v1.0.0&id=abc"
    );
}
