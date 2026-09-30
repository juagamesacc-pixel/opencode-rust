//! Rust port of `packages/app/src/desktop-menu.ts` (opencode v1.18.30).
//!
//! Source 302 lines: `DesktopMenuPlatform/Action/Role/Item/Separator/Entry/Menu`
//! types, `DESKTOP_MENU` table, `desktopMenuVisible`.
//! Pure data — ported verbatim (i18n keys, commands, actions, roles,
//! accelerators, hrefs preserved).
//! Original file: `packages/app/src/desktop-menu.ts`

#![allow(dead_code)]

/// Mirrors `DesktopMenuPlatform`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DesktopMenuPlatform {
    Macos,
    Windows,
}

/// Mirrors `DesktopMenuAction` union (verbatim strings via serde).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DesktopMenuAction {
    #[serde(rename = "app.checkForUpdates")]
    AppCheckForUpdates,
    #[serde(rename = "app.relaunch")]
    AppRelaunch,
    #[serde(rename = "edit.undo")]
    EditUndo,
    #[serde(rename = "edit.redo")]
    EditRedo,
    #[serde(rename = "edit.cut")]
    EditCut,
    #[serde(rename = "edit.copy")]
    EditCopy,
    #[serde(rename = "edit.paste")]
    EditPaste,
    #[serde(rename = "edit.delete")]
    EditDelete,
    #[serde(rename = "edit.selectAll")]
    EditSelectAll,
    #[serde(rename = "view.reload")]
    ViewReload,
    #[serde(rename = "view.toggleDevTools")]
    ViewToggleDevTools,
    #[serde(rename = "view.resetZoom")]
    ViewResetZoom,
    #[serde(rename = "view.zoomIn")]
    ViewZoomIn,
    #[serde(rename = "view.zoomOut")]
    ViewZoomOut,
    #[serde(rename = "view.toggleFullscreen")]
    ViewToggleFullscreen,
    #[serde(rename = "window.new")]
    WindowNew,
    #[serde(rename = "window.close")]
    WindowClose,
    #[serde(rename = "window.minimize")]
    WindowMinimize,
    #[serde(rename = "window.toggleMaximize")]
    WindowToggleMaximize,
}

/// Mirrors `DesktopMenuRole` union (verbatim).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
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
    #[serde(rename = "togglefullscreen")]
    Togglefullscreen,
    Undo,
    Unhide,
    WindowMenu,
    ZoomIn,
    ZoomOut,
}

/// Mirrors `DesktopMenuItem`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DesktopMenuItem {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(rename = "labelKey", skip_serializing_if = "Option::is_none")]
    pub label_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<DesktopMenuAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<DesktopMenuRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator: Option<Accelerator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<DesktopMenuPlatform>>,
}

/// Mirrors `accelerator?: Partial<Record<DesktopMenuPlatform, string>>`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Accelerator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub macos: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows: Option<String>,
}

/// Mirrors `DesktopMenuSeparator`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DesktopMenuSeparator {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<DesktopMenuPlatform>>,
}

/// Mirrors `DesktopMenuEntry` union.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type")]
pub enum DesktopMenuEntry {
    #[serde(rename = "item")]
    Item(DesktopMenuItemBody),
    #[serde(rename = "separator")]
    Separator {
        #[serde(skip_serializing_if = "Option::is_none")]
        platforms: Option<Vec<DesktopMenuPlatform>>,
    },
}

/// Body of an `item` entry (tag stripped by serde).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DesktopMenuItemBody {
    #[serde(rename = "labelKey", skip_serializing_if = "Option::is_none")]
    pub label_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<DesktopMenuAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<DesktopMenuRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accelerator: Option<Accelerator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<DesktopMenuPlatform>>,
}

/// Mirrors `DesktopMenu`.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DesktopMenu {
    pub id: String,
    #[serde(rename = "labelKey")]
    pub label_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<DesktopMenuRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DesktopMenuEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<DesktopMenuPlatform>>,
}

#[allow(clippy::too_many_arguments)] // 1:1 source has 8 params
fn item(
    label_key: Option<&str>,
    command: Option<&str>,
    action: Option<DesktopMenuAction>,
    role: Option<DesktopMenuRole>,
    href: Option<&str>,
    accelerator: Option<Accelerator>,
    enabled: Option<&str>,
    platforms: Option<Vec<DesktopMenuPlatform>>,
) -> DesktopMenuEntry {
    DesktopMenuEntry::Item(DesktopMenuItemBody {
        label_key: label_key.map(str::to_string),
        command: command.map(str::to_string),
        action,
        role,
        href: href.map(str::to_string),
        accelerator,
        enabled: enabled.map(str::to_string),
        platforms,
    })
}

fn sep(platforms: Option<Vec<DesktopMenuPlatform>>) -> DesktopMenuEntry {
    DesktopMenuEntry::Separator { platforms }
}

fn acc(macos: Option<&str>, windows: Option<&str>) -> Option<Accelerator> {
    if macos.is_none() && windows.is_none() {
        return None;
    }
    Some(Accelerator {
        macos: macos.map(str::to_string),
        windows: windows.map(str::to_string),
    })
}

/// Mirrors `DESKTOP_MENU` (order + values verbatim).
pub fn desktop_menu() -> Vec<DesktopMenu> {
    use DesktopMenuAction as A;
    use DesktopMenuPlatform as P;
    use DesktopMenuRole as R;
    vec![
        DesktopMenu {
            id: "app".to_string(),
            label_key: "desktop.menu.app".to_string(),
            role: None,
            items: Some(vec![
                item(None, None, None, Some(R::About), None, None, None, None),
                item(
                    Some("desktop.menu.checkForUpdates"),
                    None,
                    Some(A::AppCheckForUpdates),
                    None,
                    None,
                    None,
                    Some("updater"),
                    None,
                ),
                item(
                    Some("desktop.menu.settings"),
                    Some("settings.open"),
                    None,
                    None,
                    None,
                    acc(Some("Cmd+,"), None),
                    None,
                    None,
                ),
                item(Some("desktop.menu.reloadWebview"), None, Some(A::ViewReload), None, None, None, None, None),
                item(Some("desktop.menu.restart"), None, Some(A::AppRelaunch), None, None, None, None, None),
                item(Some("desktop.menu.exportLogs"), Some("logs.export"), None, None, None, None, None, None),
                sep(None),
                item(None, None, None, Some(R::Hide), None, None, None, None),
                item(None, None, None, Some(R::HideOthers), None, None, None, None),
                item(None, None, None, Some(R::Unhide), None, None, None, None),
                sep(None),
                item(None, None, None, Some(R::Quit), None, None, None, None),
            ]),
            platforms: Some(vec![P::Macos]),
        },
        DesktopMenu {
            id: "file".to_string(),
            label_key: "desktop.menu.file".to_string(),
            role: None,
            items: Some(vec![
                item(
                    Some("desktop.menu.newSession"),
                    Some("session.new"),
                    None,
                    None,
                    None,
                    acc(Some("Shift+Cmd+S"), None),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.openProject"),
                    Some("project.open"),
                    None,
                    None,
                    None,
                    acc(Some("Cmd+O"), None),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.settings"),
                    Some("settings.open"),
                    None,
                    None,
                    None,
                    acc(None, Some("Ctrl+,")),
                    None,
                    Some(vec![P::Windows]),
                ),
                item(
                    Some("desktop.menu.newWindow"),
                    None,
                    Some(A::WindowNew),
                    None,
                    None,
                    acc(Some("Cmd+Shift+N"), Some("Ctrl+Shift+N")),
                    None,
                    None,
                ),
                sep(None),
                item(
                    Some("desktop.menu.closeWindow"),
                    None,
                    Some(A::WindowClose),
                    Some(R::Close),
                    None,
                    None,
                    None,
                    None,
                ),
            ]),
            platforms: None,
        },
        DesktopMenu {
            id: "edit".to_string(),
            label_key: "desktop.menu.edit".to_string(),
            role: None,
            items: Some(vec![
                item(
                    Some("desktop.menu.undo"),
                    None,
                    Some(A::EditUndo),
                    Some(R::Undo),
                    None,
                    acc(None, Some("Ctrl+Z")),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.redo"),
                    None,
                    Some(A::EditRedo),
                    Some(R::Redo),
                    None,
                    acc(None, Some("Ctrl+Y")),
                    None,
                    None,
                ),
                sep(None),
                item(
                    Some("desktop.menu.cut"),
                    None,
                    Some(A::EditCut),
                    Some(R::Cut),
                    None,
                    acc(None, Some("Ctrl+X")),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.copy"),
                    None,
                    Some(A::EditCopy),
                    Some(R::Copy),
                    None,
                    acc(None, Some("Ctrl+C")),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.paste"),
                    None,
                    Some(A::EditPaste),
                    Some(R::Paste),
                    None,
                    acc(None, Some("Ctrl+V")),
                    None,
                    None,
                ),
                item(Some("desktop.menu.delete"), None, Some(A::EditDelete), None, None, None, None, None),
                item(
                    Some("desktop.menu.selectAll"),
                    None,
                    Some(A::EditSelectAll),
                    Some(R::SelectAll),
                    None,
                    acc(None, Some("Ctrl+A")),
                    None,
                    None,
                ),
            ]),
            platforms: None,
        },
        DesktopMenu {
            id: "view".to_string(),
            label_key: "desktop.menu.view".to_string(),
            role: None,
            items: Some(vec![
                item(Some("desktop.menu.toggleSidebar"), Some("sidebar.toggle"), None, None, None, None, None, None),
                item(
                    Some("desktop.menu.toggleTerminal"),
                    Some("terminal.toggle"),
                    None,
                    None,
                    None,
                    acc(Some("Ctrl+`"), None),
                    None,
                    None,
                ),
                item(Some("desktop.menu.toggleFileTree"), Some("fileTree.toggle"), None, None, None, None, None, None),
                sep(None),
                item(
                    Some("desktop.menu.reload"),
                    None,
                    Some(A::ViewReload),
                    Some(R::Reload),
                    None,
                    None,
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.toggleDeveloperTools"),
                    None,
                    Some(A::ViewToggleDevTools),
                    Some(R::ToggleDevTools),
                    None,
                    None,
                    None,
                    None,
                ),
                sep(None),
                item(
                    Some("desktop.menu.actualSize"),
                    None,
                    Some(A::ViewResetZoom),
                    Some(R::ResetZoom),
                    None,
                    acc(None, Some("Ctrl+0")),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.zoomIn"),
                    None,
                    Some(A::ViewZoomIn),
                    Some(R::ZoomIn),
                    None,
                    acc(None, Some("Ctrl++")),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.zoomOut"),
                    None,
                    Some(A::ViewZoomOut),
                    Some(R::ZoomOut),
                    None,
                    acc(None, Some("Ctrl+-")),
                    None,
                    None,
                ),
                sep(None),
                item(
                    Some("desktop.menu.toggleFullScreen"),
                    None,
                    Some(A::ViewToggleFullscreen),
                    Some(R::Togglefullscreen),
                    None,
                    None,
                    None,
                    None,
                ),
            ]),
            platforms: None,
        },
        DesktopMenu {
            id: "go".to_string(),
            label_key: "desktop.menu.go".to_string(),
            role: None,
            items: Some(vec![
                item(
                    Some("desktop.menu.back"),
                    Some("common.goBack"),
                    None,
                    None,
                    None,
                    acc(Some("Cmd+["), None),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.forward"),
                    Some("common.goForward"),
                    None,
                    None,
                    None,
                    acc(Some("Cmd+]"), None),
                    None,
                    None,
                ),
                sep(None),
                item(
                    Some("desktop.menu.previousSession"),
                    Some("session.previous"),
                    None,
                    None,
                    None,
                    acc(Some("Option+Up"), None),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.nextSession"),
                    Some("session.next"),
                    None,
                    None,
                    None,
                    acc(Some("Option+Down"), None),
                    None,
                    None,
                ),
                sep(None),
                item(
                    Some("desktop.menu.previousProject"),
                    Some("project.previous"),
                    None,
                    None,
                    None,
                    acc(Some("Cmd+Option+Up"), None),
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.nextProject"),
                    Some("project.next"),
                    None,
                    None,
                    None,
                    acc(Some("Cmd+Option+Down"), None),
                    None,
                    None,
                ),
            ]),
            platforms: None,
        },
        DesktopMenu {
            id: "window".to_string(),
            label_key: "desktop.menu.window".to_string(),
            role: Some(R::WindowMenu),
            items: Some(vec![
                item(Some("desktop.menu.minimize"), None, Some(A::WindowMinimize), None, None, None, None, None),
                item(
                    Some("desktop.menu.maximize"),
                    None,
                    Some(A::WindowToggleMaximize),
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                sep(None),
                item(Some("desktop.menu.closeWindow"), None, Some(A::WindowClose), None, None, None, None, None),
            ]),
            platforms: None,
        },
        DesktopMenu {
            id: "help".to_string(),
            label_key: "desktop.menu.help".to_string(),
            role: None,
            items: Some(vec![
                item(
                    Some("desktop.menu.documentation"),
                    None,
                    None,
                    None,
                    Some("https://opencode.ai/docs"),
                    None,
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.supportForum"),
                    None,
                    None,
                    None,
                    Some("https://discord.com/invite/opencode"),
                    None,
                    None,
                    None,
                ),
                item(Some("desktop.menu.exportLogs"), Some("logs.export"), None, None, None, None, None, None),
                sep(None),
                item(
                    Some("desktop.menu.shareFeedback"),
                    None,
                    None,
                    None,
                    Some("https://github.com/anomalyco/opencode/issues/new?template=feature_request.yml"),
                    None,
                    None,
                    None,
                ),
                item(
                    Some("desktop.menu.reportBug"),
                    None,
                    None,
                    None,
                    Some("https://github.com/anomalyco/opencode/issues/new?template=bug_report.yml"),
                    None,
                    None,
                    None,
                ),
            ]),
            platforms: None,
        },
    ]
}

/// Back-compat alias mirroring the `DESKTOP_MENU` const name.
pub fn desktop_menu_table() -> Vec<DesktopMenu> {
    desktop_menu()
}

/// Mirrors `desktopMenuVisible(item, platform)`.
pub fn desktop_menu_visible(
    platforms: Option<&[DesktopMenuPlatform]>,
    platform: DesktopMenuPlatform,
) -> bool {
    match platforms {
        None => true,
        Some(list) => list.contains(&platform),
    }
}
