// source: packages/web/src/content/i18n (Astro content collection `i18n`)
//
// Every locale bundle is copied byte-for-byte under `content/i18n/`. The keys below are the
// verbatim `Object.keys(en.json)` list that `content.config.ts` turns into the zod schema
// (`z.object(custom).catchall(z.string())`) for the `i18n` collection.

/// Locale bundles of the `i18n` content collection, by source file name.
pub const BUNDLES: [&str; 18] = [
    "ar.json",
    "bs.json",
    "da.json",
    "de.json",
    "en.json",
    "es.json",
    "fr.json",
    "it.json",
    "ja.json",
    "ko.json",
    "nb.json",
    "pl.json",
    "pt-BR.json",
    "ru.json",
    "th.json",
    "tr.json",
    "zh-CN.json",
    "zh-TW.json",
];

/// Path of each bundle relative to the crate root.
pub fn bundle_path(file: &str) -> String {
    format!("content/i18n/{}", file)
}

/// Message key count per bundle, aligned with [`BUNDLES`].
pub const BUNDLE_KEY_COUNTS: [(&str, usize); 18] = [
    ("ar.json", 73),
    ("bs.json", 73),
    ("da.json", 73),
    ("de.json", 73),
    ("en.json", 73),
    ("es.json", 73),
    ("fr.json", 73),
    ("it.json", 73),
    ("ja.json", 73),
    ("ko.json", 73),
    ("nb.json", 73),
    ("pl.json", 73),
    ("pt-BR.json", 73),
    ("ru.json", 73),
    ("th.json", 73),
    ("tr.json", 73),
    ("zh-CN.json", 73),
    ("zh-TW.json", 73),
];

/// Number of message keys in the English bundle; the schema is derived from these.
pub const EN_KEY_COUNT: usize = 73;

/// Verbatim `Object.keys(en.json)`: the keys the `i18n` collection schema is built from.
pub const EN_KEYS: [&str; 73] = [
    "app.head.titleSuffix",
    "app.header.home",
    "app.header.docs",
    "app.footer.issueLink",
    "app.footer.discordLink",
    "app.lander.hero.title",
    "app.lander.cta.getStarted",
    "app.lander.features.native_tui.title",
    "app.lander.features.native_tui.description",
    "app.lander.features.lsp_enabled.title",
    "app.lander.features.lsp_enabled.description",
    "app.lander.features.multi_session.title",
    "app.lander.features.multi_session.description",
    "app.lander.features.shareable_links.title",
    "app.lander.features.shareable_links.description",
    "app.lander.features.github_copilot.description",
    "app.lander.features.chatgpt_plus_pro.description",
    "app.lander.features.use_any_model.title",
    "app.lander.features.use_any_model.prefix",
    "app.lander.features.use_any_model.suffix",
    "app.lander.images.tui.caption",
    "app.lander.images.tui.alt",
    "app.lander.images.vscode.caption",
    "app.lander.images.vscode.alt",
    "app.lander.images.github.caption",
    "app.lander.images.github.alt",
    "share.meta_description",
    "share.not_found",
    "share.link_to_message",
    "share.copied",
    "share.copy",
    "share.show_more",
    "share.show_less",
    "share.show_results",
    "share.hide_results",
    "share.show_details",
    "share.hide_details",
    "share.show_preview",
    "share.hide_preview",
    "share.show_contents",
    "share.hide_contents",
    "share.show_output",
    "share.hide_output",
    "share.error",
    "share.waiting_for_messages",
    "share.status_connected_waiting",
    "share.status_connecting",
    "share.status_disconnected",
    "share.status_reconnecting",
    "share.status_error",
    "share.status_unknown",
    "share.error_id_not_found",
    "share.error_api_url_not_found",
    "share.error_connection_failed",
    "share.opencode_version",
    "share.opencode_name",
    "share.models",
    "share.cost",
    "share.input_tokens",
    "share.output_tokens",
    "share.reasoning_tokens",
    "share.scroll_to_bottom",
    "share.attachment",
    "share.thinking",
    "share.thinking_pending",
    "share.creating_plan",
    "share.completing_plan",
    "share.updating_plan",
    "share.match_one",
    "share.match_other",
    "share.result_one",
    "share.result_other",
    "share.debug_key",
];

/// `true` when `key` is one of the keys the `i18n` collection schema declares.
pub fn is_known_key(key: &str) -> bool {
    EN_KEYS.contains(&key)
}

/// Message key count for a bundle, or `None` when the bundle is unknown.
pub fn key_count(file: &str) -> Option<usize> {
    BUNDLE_KEY_COUNTS
        .iter()
        .find(|(name, _)| *name == file)
        .map(|(_, count)| *count)
}
