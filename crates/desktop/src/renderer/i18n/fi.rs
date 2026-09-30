//! Rust port of `src/renderer/i18n/fi.ts` (opencode v1.18.30).
//!
//! Verbatim locale string table. Values and placeholders are preserved byte-for-byte.
//! Original file: `packages/desktop/src/renderer/i18n/fi.ts`

/// Mirrors `export const dict` as a verbatim flattened string table.
pub const ENTRIES: &[(&str, &str)] = &[
    ("desktop.menu.checkForUpdates", "Tarkista päivitykset..."),
    ("desktop.menu.installCli", "Asenna CLI..."),
    ("desktop.menu.reloadWebview", "Lataa verkkonäkymä uudelleen"),
    ("desktop.menu.restart", "Käynnistä uudelleen"),
    ("desktop.dialog.chooseFolder", "Valitse kansio"),
    ("desktop.dialog.chooseFile", "Valitse tiedosto"),
    ("desktop.dialog.saveFile", "Tallenna tiedosto"),
    ("desktop.updater.checkFailed.title", "Päivitystarkistus epäonnistui"),
    ("desktop.updater.checkFailed.message", "Päivitysten tarkistaminen epäonnistui"),
    ("desktop.updater.none.title", "Päivitystä ei ole saatavilla"),
    ("desktop.updater.none.message", "Käytät jo OpenCoden uusinta versiota"),
    ("desktop.updater.downloadFailed.title", "Päivitys epäonnistui"),
    ("desktop.updater.downloadFailed.message", "Päivityksen lataaminen epäonnistui"),
    ("desktop.updater.downloaded.title", "Päivitys ladattu"),
    ("desktop.updater.downloaded.prompt", "OpenCoden versio {{version}} on ladattu. Haluatko asentaa sen ja käynnistää OpenCoden uudelleen?"),
    ("desktop.updater.installFailed.title", "Päivitys epäonnistui"),
    ("desktop.updater.installFailed.message", "Päivityksen asentaminen epäonnistui"),
    ("desktop.cli.installed.title", "CLI on asennettu"),
    ("desktop.cli.installed.message", "CLI on asennettu polkuun {{path}}\n\nKäynnistä terminaali uudelleen, jotta voit käyttää 'opencode'-komentoa."),
    ("desktop.cli.failed.title", "Asennus epäonnistui"),
    ("desktop.cli.failed.message", "CLI:n asennus epäonnistui: {{error}}"),
    ("desktop.error.dev.rootNotFound", "Juurielementtiä ei löydy. Unohditko lisätä sen index.html-tiedostoosi? Tai ehkä id-attribuutti on kirjoitettu väärin?"),
];
