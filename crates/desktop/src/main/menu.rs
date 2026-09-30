//! Rust port of `src/main/menu.ts` (opencode v1.18.30), including the
//! `@opencode-ai/app/desktop-menu` types and `DESKTOP_MENU` data it consumes.
//!
//! Fully ported: `DesktopMenuPlatform`/`DesktopMenuRole`/`DesktopMenuAction`
//! (`DesktopMenuAction` lives in `crate::preload::types`), the item/separator
//! entry shapes, the `DESKTOP_MENU` const (byte-for-byte field parity with
//! `packages/app/src/desktop-menu.ts`), `desktopMenuVisible`, and the
//! `createMenu` template mapping (`build_template`): role-menus, visibility
//! filtering, `native_t(menu.labelKey)` labels, `accelerator?.macos`, and the
//! `enabled: entry.enabled === "updater" ? UPDATER_ENABLED : undefined` gate.
//! PROVISIONAL: `Menu.buildFromTemplate` / `Menu.setApplicationMenu` /
//! `BrowserWindow.getFocusedWindow` (Electron) and the click wiring in
//! [`create_menu`]; it is darwin-only, exactly as the source.
//!
//! Original files: `packages/desktop/src/main/menu.ts` +
//! `packages/app/src/desktop-menu.ts`

use crate::main::native_translations::native_t;
use crate::preload::types::DesktopMenuAction;

/// Mirrors `type DesktopMenuPlatform = "macos" | "windows"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopMenuPlatform {
    Macos,
    Windows,
}

impl DesktopMenuPlatform {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopMenuPlatform::Macos => "macos",
            DesktopMenuPlatform::Windows => "windows",
        }
    }

    pub fn from_str(value: &str) -> Option<DesktopMenuPlatform> {
        match value {
            "macos" => Some(DesktopMenuPlatform::Macos),
            "windows" => Some(DesktopMenuPlatform::Windows),
            _ => None,
        }
    }
}

/// Mirrors `type DesktopMenuRole` (the string union in
/// `packages/app/src/desktop-menu.ts`; `role` is passed through verbatim to
/// Electron, hence the lowercase `togglefullscreen`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopMenuRole {
    About,
    Close,
    Copy,
    Cut,
    Hide,
    HideOthers,
    Paste,
    Quit,
    Redo,
    Reload,
    ResetZoom,
    SelectAll,
    ToggleDevTools,
    ToggleFullscreen,
    Undo,
    Unhide,
    WindowMenu,
    ZoomIn,
    ZoomOut,
}

impl DesktopMenuRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            DesktopMenuRole::About => "about",
            DesktopMenuRole::Close => "close",
            DesktopMenuRole::Copy => "copy",
            DesktopMenuRole::Cut => "cut",
            DesktopMenuRole::Hide => "hide",
            DesktopMenuRole::HideOthers => "hideOthers",
            DesktopMenuRole::Paste => "paste",
            DesktopMenuRole::Quit => "quit",
            DesktopMenuRole::Redo => "redo",
            DesktopMenuRole::Reload => "reload",
            DesktopMenuRole::ResetZoom => "resetZoom",
            DesktopMenuRole::SelectAll => "selectAll",
            DesktopMenuRole::ToggleDevTools => "toggleDevTools",
            DesktopMenuRole::ToggleFullscreen => "togglefullscreen",
            DesktopMenuRole::Undo => "undo",
            DesktopMenuRole::Unhide => "unhide",
            DesktopMenuRole::WindowMenu => "windowMenu",
            DesktopMenuRole::ZoomIn => "zoomIn",
            DesktopMenuRole::ZoomOut => "zoomOut",
        }
    }

    pub fn from_str(value: &str) -> Option<DesktopMenuRole> {
        match value {
            "about" => Some(DesktopMenuRole::About),
            "close" => Some(DesktopMenuRole::Close),
            "copy" => Some(DesktopMenuRole::Copy),
            "cut" => Some(DesktopMenuRole::Cut),
            "hide" => Some(DesktopMenuRole::Hide),
            "hideOthers" => Some(DesktopMenuRole::HideOthers),
            "paste" => Some(DesktopMenuRole::Paste),
            "quit" => Some(DesktopMenuRole::Quit),
            "redo" => Some(DesktopMenuRole::Redo),
            "reload" => Some(DesktopMenuRole::Reload),
            "resetZoom" => Some(DesktopMenuRole::ResetZoom),
            "selectAll" => Some(DesktopMenuRole::SelectAll),
            "toggleDevTools" => Some(DesktopMenuRole::ToggleDevTools),
            "togglefullscreen" => Some(DesktopMenuRole::ToggleFullscreen),
            "undo" => Some(DesktopMenuRole::Undo),
            "unhide" => Some(DesktopMenuRole::Unhide),
            "windowMenu" => Some(DesktopMenuRole::WindowMenu),
            "zoomIn" => Some(DesktopMenuRole::ZoomIn),
            "zoomOut" => Some(DesktopMenuRole::ZoomOut),
            _ => None,
        }
    }
}

/// Mirrors `entry.enabled?: "updater"`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopMenuEnabled {
    Updater,
}

/// Mirrors `type DesktopMenuItem` (the `accelerator` `PartialRecord` becomes
/// the two per-platform fields, and `labelKey`/`command`/`href` are static
/// strings in the const data).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesktopMenuItem {
    pub kind: DesktopMenuEntryKind,
    pub label_key: Option<&'static str>,
    pub command: Option<&'static str>,
    pub action: Option<DesktopMenuAction>,
    pub role: Option<DesktopMenuRole>,
    pub href: Option<&'static str>,
    pub accelerator_macos: Option<&'static str>,
    pub accelerator_windows: Option<&'static str>,
    pub enabled: Option<DesktopMenuEnabled>,
    pub platforms: Option<&'static [DesktopMenuPlatform]>,
}

/// Mirrors `type DesktopMenuEntry = DesktopMenuItem | DesktopMenuSeparator`
/// (the `type` discriminator).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopMenuEntryKind {
    Item,
    Separator,
}

/// Mirrors `type DesktopMenu`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesktopMenu {
    pub id: &'static str,
    pub label_key: &'static str,
    pub role: Option<DesktopMenuRole>,
    pub items: &'static [DesktopMenuItem],
    pub platforms: Option<&'static [DesktopMenuPlatform]>,
}

const fn item(label_key: Option<&'static str>) -> DesktopMenuItem {
    DesktopMenuItem {
        kind: DesktopMenuEntryKind::Item,
        label_key,
        command: None,
        action: None,
        role: None,
        href: None,
        accelerator_macos: None,
        accelerator_windows: None,
        enabled: None,
        platforms: None,
    }
}

/// Mirrors `export const DESKTOP_MENU: DesktopMenu[]` in
/// `packages/app/src/desktop-menu.ts`, field-for-field.
pub const DESKTOP_MENU: [DesktopMenu; 7] = [
    DesktopMenu {
        id: "app",
        label_key: "desktop.menu.app",
        role: None,
        platforms: Some(&[DesktopMenuPlatform::Macos]),
        items: &[
            DesktopMenuItem {
                role: Some(DesktopMenuRole::About),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.checkForUpdates"),
                action: Some(DesktopMenuAction::AppCheckForUpdates),
                enabled: Some(DesktopMenuEnabled::Updater),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.settings"),
                command: Some("settings.open"),
                accelerator_macos: Some("Cmd+,"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.reloadWebview"),
                action: Some(DesktopMenuAction::ViewReload),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.restart"),
                action: Some(DesktopMenuAction::AppRelaunch),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.exportLogs"),
                command: Some("logs.export"),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                role: Some(DesktopMenuRole::Hide),
                ..item(None)
            },
            DesktopMenuItem {
                role: Some(DesktopMenuRole::HideOthers),
                ..item(None)
            },
            DesktopMenuItem {
                role: Some(DesktopMenuRole::Unhide),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                role: Some(DesktopMenuRole::Quit),
                ..item(None)
            },
        ],
    },
    DesktopMenu {
        id: "file",
        label_key: "desktop.menu.file",
        role: None,
        platforms: None,
        items: &[
            DesktopMenuItem {
                label_key: Some("desktop.menu.newSession"),
                command: Some("session.new"),
                accelerator_macos: Some("Shift+Cmd+S"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.openProject"),
                command: Some("project.open"),
                accelerator_macos: Some("Cmd+O"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.settings"),
                command: Some("settings.open"),
                accelerator_windows: Some("Ctrl+,"),
                platforms: Some(&[DesktopMenuPlatform::Windows]),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.newWindow"),
                action: Some(DesktopMenuAction::WindowNew),
                accelerator_macos: Some("Cmd+Shift+N"),
                accelerator_windows: Some("Ctrl+Shift+N"),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.closeWindow"),
                action: Some(DesktopMenuAction::WindowClose),
                role: Some(DesktopMenuRole::Close),
                ..item(None)
            },
        ],
    },
    DesktopMenu {
        id: "edit",
        label_key: "desktop.menu.edit",
        role: None,
        platforms: None,
        items: &[
            DesktopMenuItem {
                label_key: Some("desktop.menu.undo"),
                action: Some(DesktopMenuAction::EditUndo),
                role: Some(DesktopMenuRole::Undo),
                accelerator_windows: Some("Ctrl+Z"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.redo"),
                action: Some(DesktopMenuAction::EditRedo),
                role: Some(DesktopMenuRole::Redo),
                accelerator_windows: Some("Ctrl+Y"),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.cut"),
                action: Some(DesktopMenuAction::EditCut),
                role: Some(DesktopMenuRole::Cut),
                accelerator_windows: Some("Ctrl+X"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.copy"),
                action: Some(DesktopMenuAction::EditCopy),
                role: Some(DesktopMenuRole::Copy),
                accelerator_windows: Some("Ctrl+C"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.paste"),
                action: Some(DesktopMenuAction::EditPaste),
                role: Some(DesktopMenuRole::Paste),
                accelerator_windows: Some("Ctrl+V"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.delete"),
                action: Some(DesktopMenuAction::EditDelete),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.selectAll"),
                action: Some(DesktopMenuAction::EditSelectAll),
                role: Some(DesktopMenuRole::SelectAll),
                accelerator_windows: Some("Ctrl+A"),
                ..item(None)
            },
        ],
    },
    DesktopMenu {
        id: "view",
        label_key: "desktop.menu.view",
        role: None,
        platforms: None,
        items: &[
            DesktopMenuItem {
                label_key: Some("desktop.menu.toggleSidebar"),
                command: Some("sidebar.toggle"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.toggleTerminal"),
                command: Some("terminal.toggle"),
                accelerator_macos: Some("Ctrl+`"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.toggleFileTree"),
                command: Some("fileTree.toggle"),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.reload"),
                action: Some(DesktopMenuAction::ViewReload),
                role: Some(DesktopMenuRole::Reload),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.toggleDeveloperTools"),
                action: Some(DesktopMenuAction::ViewToggleDevTools),
                role: Some(DesktopMenuRole::ToggleDevTools),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.actualSize"),
                action: Some(DesktopMenuAction::ViewResetZoom),
                role: Some(DesktopMenuRole::ResetZoom),
                accelerator_windows: Some("Ctrl+0"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.zoomIn"),
                action: Some(DesktopMenuAction::ViewZoomIn),
                role: Some(DesktopMenuRole::ZoomIn),
                accelerator_windows: Some("Ctrl++"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.zoomOut"),
                action: Some(DesktopMenuAction::ViewZoomOut),
                role: Some(DesktopMenuRole::ZoomOut),
                accelerator_windows: Some("Ctrl+-"),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.toggleFullScreen"),
                action: Some(DesktopMenuAction::ViewToggleFullscreen),
                role: Some(DesktopMenuRole::ToggleFullscreen),
                ..item(None)
            },
        ],
    },
    DesktopMenu {
        id: "go",
        label_key: "desktop.menu.go",
        role: None,
        platforms: None,
        items: &[
            DesktopMenuItem {
                label_key: Some("desktop.menu.back"),
                command: Some("common.goBack"),
                accelerator_macos: Some("Cmd+["),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.forward"),
                command: Some("common.goForward"),
                accelerator_macos: Some("Cmd+]"),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.previousSession"),
                command: Some("session.previous"),
                accelerator_macos: Some("Option+Up"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.nextSession"),
                command: Some("session.next"),
                accelerator_macos: Some("Option+Down"),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.previousProject"),
                command: Some("project.previous"),
                accelerator_macos: Some("Cmd+Option+Up"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.nextProject"),
                command: Some("project.next"),
                accelerator_macos: Some("Cmd+Option+Down"),
                ..item(None)
            },
        ],
    },
    DesktopMenu {
        id: "window",
        label_key: "desktop.menu.window",
        role: Some(DesktopMenuRole::WindowMenu),
        platforms: None,
        items: &[
            DesktopMenuItem {
                label_key: Some("desktop.menu.minimize"),
                action: Some(DesktopMenuAction::WindowMinimize),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.maximize"),
                action: Some(DesktopMenuAction::WindowToggleMaximize),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.closeWindow"),
                action: Some(DesktopMenuAction::WindowClose),
                ..item(None)
            },
        ],
    },
    DesktopMenu {
        id: "help",
        label_key: "desktop.menu.help",
        role: None,
        platforms: None,
        items: &[
            DesktopMenuItem {
                label_key: Some("desktop.menu.documentation"),
                href: Some("https://opencode.ai/docs"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.supportForum"),
                href: Some("https://discord.com/invite/opencode"),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.exportLogs"),
                command: Some("logs.export"),
                ..item(None)
            },
            DesktopMenuItem {
                kind: DesktopMenuEntryKind::Separator,
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.shareFeedback"),
                href: Some(
                    "https://github.com/anomalyco/opencode/issues/new?template=feature_request.yml",
                ),
                ..item(None)
            },
            DesktopMenuItem {
                label_key: Some("desktop.menu.reportBug"),
                href: Some(
                    "https://github.com/anomalyco/opencode/issues/new?template=bug_report.yml",
                ),
                ..item(None)
            },
        ],
    },
];

/// Mirrors `desktopMenuVisible(item, platform)`:
/// `!item.platforms || item.platforms.includes(platform)`.
pub fn desktop_menu_visible(
    platforms: Option<&[DesktopMenuPlatform]>,
    platform: DesktopMenuPlatform,
) -> bool {
    match platforms {
        None => true,
        Some(platforms) => platforms.contains(&platform),
    }
}

/// The click behavior of one template item, mirroring the three branches in
/// `nativeItem` (`command` → `deps.trigger`, `action` →
/// `runDesktopMenuAction`, `href` → `openExternalURL`). PROVISIONAL: the
/// Electron `MenuItemConstructorOptions["click"]` closure has no binding;
/// [`create_menu`] is where the wiring would land.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MenuClick {
    Command(String),
    Action(DesktopMenuAction),
    Href(&'static str),
}

/// One built `MenuItemConstructorOptions` shape, as produced by `nativeItem`
/// minus the click closure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateMenuItem {
    pub kind: DesktopMenuEntryKind,
    pub label: Option<String>,
    pub accelerator: Option<String>,
    pub enabled: Option<bool>,
    pub click: Option<MenuClick>,
}

/// One built top-level template entry, as produced by `createMenu`'s
/// `DESKTOP_MENU.filter(...).map(...)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateMenu {
    pub label: Option<String>,
    pub role: Option<DesktopMenuRole>,
    pub items: Option<Vec<TemplateMenuItem>>,
}

/// Mirrors `nativeItem(entry, deps)`: separator passthrough, role items with
/// an optional label, then the custom item (`accelerator?.macos`, the
/// `enabled: "updater"` gate against `UPDATER_ENABLED`, and the command /
/// action / href click branch). Labels resolve through `nativeT(labelKey)`.
pub fn native_item(entry: &DesktopMenuItem, updater_enabled: bool) -> TemplateMenuItem {
    if entry.kind == DesktopMenuEntryKind::Separator {
        return TemplateMenuItem {
            kind: DesktopMenuEntryKind::Separator,
            label: None,
            accelerator: None,
            enabled: None,
            click: None,
        };
    }
    if let Some(role) = entry.role {
        return TemplateMenuItem {
            kind: DesktopMenuEntryKind::Item,
            label: entry.label_key.map(|key| native_t(key, &[])),
            accelerator: None,
            enabled: None,
            click: None,
        };
    }
    let click = if let Some(command) = entry.command {
        Some(MenuClick::Command(command.to_string()))
    } else if let Some(action) = entry.action {
        Some(MenuClick::Action(action))
    } else if let Some(href) = entry.href {
        Some(MenuClick::Href(href))
    } else {
        None
    };
    TemplateMenuItem {
        kind: DesktopMenuEntryKind::Item,
        label: entry.label_key.map(|key| native_t(key, &[])),
        accelerator: entry.accelerator_macos.map(str::to_string),
        enabled: match entry.enabled {
            Some(DesktopMenuEnabled::Updater) => Some(updater_enabled),
            None => None,
        },
        click,
    }
}

/// Mirrors the `createMenu` template mapping for `platform`:
/// `DESKTOP_MENU.filter(...).map(...)` with `desktopMenuVisible` on menus and
/// entries. `createMenu` itself is darwin-only, so `accelerator` carries the
/// `.macos` value, exactly as `nativeItem` reads it.
pub fn build_template(platform: DesktopMenuPlatform, updater_enabled: bool) -> Vec<TemplateMenu> {
    DESKTOP_MENU
        .iter()
        .filter(|menu| desktop_menu_visible(menu.platforms, platform))
        .map(|menu| {
            if let Some(role) = menu.role {
                return TemplateMenu {
                    label: Some(native_t(menu.label_key, &[])),
                    role: Some(role),
                    items: None,
                };
            }
            TemplateMenu {
                label: Some(native_t(menu.label_key, &[])),
                role: None,
                items: Some(
                    menu.items
                        .iter()
                        .filter(|entry| desktop_menu_visible(entry.platforms, platform))
                        .map(|entry| native_item(entry, updater_enabled))
                        .collect(),
                ),
            }
        })
        .collect()
}

/// Mirrors `type Deps` in `menu.ts`. PROVISIONAL: the three deps are the
/// renderer/binding callbacks (`trigger`, `checkForUpdates`, `relaunch`);
/// `None` matches no Electron wiring being bound yet.
#[derive(Default)]
pub struct MenuDeps {
    pub trigger: Option<Box<dyn FnMut(String)>>,
    pub check_for_updates: Option<Box<dyn FnMut()>>,
    pub relaunch: Option<Box<dyn FnMut()>>,
}

/// Mirrors `export function createMenu(deps)`: darwin-only guard, then
/// `Menu.buildFromTemplate` + `Menu.setApplicationMenu`.
///
/// PROVISIONAL(packages/desktop/src/main/menu.ts): the Electron `Menu`
/// binding is absent; [`build_template`] carries the full template
/// construction (including the click data via [`MenuClick`]), and the
/// `runDesktopMenuAction` wiring would call `DesktopMenuActionHandlers`
/// with the `BrowserWindow.getFocusedWindow()` stand-in.
pub fn create_menu(_deps: &mut MenuDeps) {
    if !cfg!(target_os = "macos") {
        return;
    }
    unimplemented!("Electron Menu.buildFromTemplate/setApplicationMenu binding")
}

#[cfg(test)]
mod tests {
    // There is no `menu.test.ts` in the source; these pin the template
    // mapping to the `DESKTOP_MENU` data and `createMenu`'s pipeline.
    use super::*;

    #[test]
    fn visibility_is_default_true_and_platform_gated() {
        assert!(desktop_menu_visible(None, DesktopMenuPlatform::Macos));
        assert!(desktop_menu_visible(
            Some(&[DesktopMenuPlatform::Macos, DesktopMenuPlatform::Windows]),
            DesktopMenuPlatform::Windows
        ));
        assert!(!desktop_menu_visible(
            Some(&[DesktopMenuPlatform::Windows]),
            DesktopMenuPlatform::Macos
        ));
    }

    #[test]
    fn macos_template_matches_the_source_menu_structure() {
        let template = build_template(DesktopMenuPlatform::Macos, true);
        assert_eq!(template.len(), 7, "app, file, edit, view, go, window, help");

        // The app menu is macos-only and starts with the about role.
        let app = &template[0];
        assert_eq!(
            app.label.as_deref(),
            Some(native_t("desktop.menu.app", &[]).as_str())
        );
        assert_eq!(app.role, None);
        let items = app.items.as_ref().expect("app menu items");
        assert_eq!(items.len(), 12, "about..quit incl. two separators");
        assert_eq!(items[0].click, None);
        assert_eq!(items[0].label, None, "the about item has no labelKey");

        // checkForUpdates gates on UPDATER_ENABLED.
        assert_eq!(
            items[1].click,
            Some(MenuClick::Action(DesktopMenuAction::AppCheckForUpdates))
        );
        assert_eq!(items[1].enabled, Some(true));

        // The window menu is a windowMenu role menu with a submenu.
        let window = template
            .iter()
            .find(|menu| menu.role == Some(DesktopMenuRole::WindowMenu))
            .expect("window menu");
        assert_eq!(
            window.label.as_deref(),
            Some(native_t("desktop.menu.window", &[]).as_str())
        );
        assert_eq!(window.items.as_ref().expect("window items").len(), 4);
    }

    #[test]
    fn updater_gate_follows_the_flag() {
        let template = build_template(DesktopMenuPlatform::Macos, false);
        let app = &template[0];
        let items = app.items.as_ref().expect("app menu items");
        assert_eq!(
            items[1].enabled,
            Some(false),
            "entry.enabled === \"updater\" ? UPDATER_ENABLED : undefined"
        );
    }

    #[test]
    fn windows_template_drops_the_app_menu_and_includes_file_settings() {
        let template = build_template(DesktopMenuPlatform::Windows, true);
        assert_eq!(template.len(), 6, "no app menu on windows");
        let ids: Vec<&str> = DESKTOP_MENU
            .iter()
            .filter(|menu| desktop_menu_visible(menu.platforms, DesktopMenuPlatform::Windows))
            .map(|menu| menu.id)
            .collect();
        assert_eq!(ids, vec!["file", "edit", "view", "go", "window", "help"]);

        // The windows-only Settings item under File.
        let file = &template[0];
        let items = file.items.as_ref().expect("file items");
        let settings = items
            .iter()
            .find(|item| item.click == Some(MenuClick::Command("settings.open".to_string())))
            .expect("settings item");
        assert_eq!(
            settings.accelerator.as_deref(),
            None,
            "windows accelerator is not read on macos builds"
        );
    }

    #[test]
    fn action_and_href_clicks_resolve() {
        let template = build_template(DesktopMenuPlatform::Macos, true);
        let help = template.last().expect("help menu");
        assert_eq!(
            help.label.as_deref(),
            Some(native_t("desktop.menu.help", &[]).as_str())
        );
        let items = help.items.as_ref().expect("help items");
        assert_eq!(
            items[0].click,
            Some(MenuClick::Href("https://opencode.ai/docs"))
        );
        assert!(template
            .iter()
            .flat_map(|menu| menu.items.as_ref().expect("items"))
            .any(|item| item.click == Some(MenuClick::Command("logs.export".to_string()))));
    }
}
