// source: packages/web/config.mjs
//
// 1:1 port. `SST_STAGE` selects between the production origin and the per-stage subdomain.

/// Environment variable read by the source module.
pub const STAGE_ENV: &str = "SST_STAGE";

/// Stage used when `SST_STAGE` is unset.
pub const DEFAULT_STAGE: &str = "dev";

/// Stage that pins the production origin.
pub const PRODUCTION_STAGE: &str = "production";

/// Origin for `PRODUCTION_STAGE`.
pub const PRODUCTION_URL: &str = "https://opencode.ai";

/// Auth console for `PRODUCTION_STAGE`.
pub const PRODUCTION_CONSOLE: &str = "https://opencode.ai/auth";

/// Support address exported to every doc page as `email`.
pub const EMAIL: &str = "help@anoma.ly";

/// Base URL of the social-card image service.
pub const SOCIAL_CARD: &str = "https://social-cards.sst.dev";

/// Repository the docs edit links point at.
pub const GITHUB: &str = "https://github.com/anomalyco/opencode";

/// Community invite link shown in the footer.
pub const DISCORD: &str = "https://opencode.ai/discord";

/// Path suffix appended to [`GITHUB`] to build the Starlight `editLink.baseUrl`.
pub const EDIT_LINK_SUFFIX: &str = "/edit/dev/packages/web/";

/// One entry of `config.headerLinks`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderLink {
    /// i18n key rendered as the link text.
    pub name: &'static str,
    /// Link target.
    pub url: &'static str,
}

/// `config.headerLinks`, verbatim.
pub const HEADER_LINKS: [HeaderLink; 2] = [
    HeaderLink {
        name: "app.header.home",
        url: "/",
    },
    HeaderLink {
        name: "app.header.docs",
        url: "/docs/",
    },
];

/// The resolved `config.mjs` default export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// Value of `SST_STAGE`, or `DEFAULT_STAGE` when unset.
    pub stage: String,
    /// Site origin handed to `astro.config.mjs` as `site`.
    pub url: String,
    /// Auth console exported to doc pages as `console`.
    pub console: String,
    /// Support address exported to doc pages as `email`.
    pub email: &'static str,
    /// Social-card image service base URL.
    pub social_card: &'static str,
    /// Repository URL, also the source of the edit link.
    pub github: &'static str,
    /// Discord invite URL.
    pub discord: &'static str,
    /// Header navigation entries.
    pub header_links: &'static [HeaderLink],
}

impl Config {
    /// The configuration for the default (`dev`) stage.
    pub fn new() -> Self {
        Self::for_stage(DEFAULT_STAGE)
    }

    /// The configuration for `stage`.
    pub fn for_stage(stage: &str) -> Self {
        let production = stage == PRODUCTION_STAGE;
        Self {
            stage: stage.to_string(),
            url: if production {
                PRODUCTION_URL.to_string()
            } else {
                format!("https://{}.opencode.ai", stage)
            },
            console: if production {
                PRODUCTION_CONSOLE.to_string()
            } else {
                format!("https://{}.opencode.ai/auth", stage)
            },
            email: EMAIL,
            social_card: SOCIAL_CARD,
            github: GITHUB,
            discord: DISCORD,
            header_links: &HEADER_LINKS,
        }
    }

    /// Starlight `editLink.baseUrl`.
    pub fn edit_link_base(&self) -> String {
        format!("{}{}", self.github, EDIT_LINK_SUFFIX)
    }

    /// Social-card URL for a docs page, as built by `src/components/Head.astro`.
    pub fn docs_social_card(&self, encoded_title: &str, encoded_description: &str) -> String {
        format!(
            "{}/opencode-docs/{}.png?desc={}",
            self.social_card, encoded_title, encoded_description
        )
    }

    /// Social-card URL for a share page, as built by `src/pages/s/[id].astro`.
    pub fn share_social_card(
        &self,
        encoded_title: &str,
        model_param: &str,
        version: &str,
        id: &str,
    ) -> String {
        format!(
            "{}/opencode-share/{}.png?model={}&version={}&id={}",
            self.social_card, encoded_title, model_param, version, id
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
