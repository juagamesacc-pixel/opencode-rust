//! Rust port of `src/renderer/i18n/es.ts` (opencode v1.18.30).
//!
//! Verbatim locale string table. Values and placeholders are preserved byte-for-byte.
//! Original file: `packages/desktop/src/renderer/i18n/es.ts`

/// Mirrors `export const dict` as a verbatim flattened string table.
pub const ENTRIES: &[(&str, &str)] = &[
    ("desktop.menu.checkForUpdates", "Buscar actualizaciones..."),
    ("desktop.menu.installCli", "Instalar CLI..."),
    ("desktop.menu.reloadWebview", "Recargar vista web"),
    ("desktop.menu.restart", "Reiniciar"),
    ("desktop.dialog.chooseFolder", "Elegir una carpeta"),
    ("desktop.dialog.chooseFile", "Elegir un archivo"),
    ("desktop.dialog.saveFile", "Guardar archivo"),
    ("desktop.updater.checkFailed.title", "Comprobación de actualizaciones fallida"),
    ("desktop.updater.checkFailed.message", "No se pudieron buscar actualizaciones"),
    ("desktop.updater.none.title", "No hay actualizaciones disponibles"),
    ("desktop.updater.none.message", "Ya estás usando la versión más reciente de OpenCode"),
    ("desktop.updater.downloadFailed.title", "Actualización fallida"),
    ("desktop.updater.downloadFailed.message", "No se pudo descargar la actualización"),
    ("desktop.updater.downloaded.title", "Actualización descargada"),
    ("desktop.updater.downloaded.prompt", "Se ha descargado la versión {{version}} de OpenCode. ¿Quieres instalarla y reiniciar?"),
    ("desktop.updater.installFailed.title", "Actualización fallida"),
    ("desktop.updater.installFailed.message", "No se pudo instalar la actualización"),
    ("desktop.cli.installed.title", "CLI instalada"),
    ("desktop.cli.installed.message", "CLI instalada en {{path}}\n\nReinicia tu terminal para usar el comando 'opencode'."),
    ("desktop.cli.failed.title", "Instalación fallida"),
    ("desktop.cli.failed.message", "No se pudo instalar la CLI: {{error}}"),
    ("desktop.error.dev.rootNotFound", "Elemento raíz no encontrado. ¿Olvidaste añadirlo a tu index.html? ¿O tal vez el atributo id está mal escrito?"),
];
