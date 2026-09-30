// source: src/config/managed.ts — exports: managedConfigDir,
// parseManagedPlist, readManagedPreferences, ConfigManaged
// (platform dirs, plist domain/paths, meta-key strip verbatim).

use serde_json::Value;

/// source: MANAGED_PLIST_DOMAIN — verbatim.
pub const MANAGED_PLIST_DOMAIN: &str = "ai.opencode.managed";

/// source: PLIST_META keys — verbatim set.
pub const PLIST_META: &[&str] = &[
    "PayloadDisplayName",
    "PayloadIdentifier",
    "PayloadType",
    "PayloadUUID",
    "PayloadVersion",
    "_manualProfile",
];

/// source: systemManagedConfigDir() — darwin/win32/default, verbatim.
pub fn system_managed_config_dir(platform: &str, program_data: Option<&str>) -> String {
    match platform {
        "darwin" => "/Library/Application Support/opencode".to_string(),
        "win32" => format!("{}/opencode", program_data.unwrap_or("C:\\ProgramData")),
        _ => "/etc/opencode".to_string(),
    }
}

/// source: OPENCODE_TEST_MANAGED_CONFIG_DIR override — verbatim key.
pub const MANAGED_CONFIG_DIR_ENV: &str = "OPENCODE_TEST_MANAGED_CONFIG_DIR";

/// source: managed plist paths — verbatim templates.
pub fn managed_plist_paths(user: &str) -> [String; 2] {
    [
        format!(
            "/Library/Managed Preferences/{}/{}.plist",
            user, MANAGED_PLIST_DOMAIN
        ),
        format!(
            "/Library/Managed Preferences/{}.plist",
            MANAGED_PLIST_DOMAIN
        ),
    ]
}

/// source: plutil argv — verbatim.
pub fn plutil_argv(plist: &str) -> Vec<String> {
    vec![
        "plutil".into(),
        "-convert".into(),
        "json".into(),
        "-o".into(),
        "-".into(),
        plist.into(),
    ]
}

/// source: parseManagedPlist() — strip PLIST_META, re-stringify. Verbatim.
pub fn parse_managed_plist(json: &str) -> Result<String, String> {
    let mut raw: Value = serde_json::from_str(json).map_err(|e| e.to_string())?;
    if let Some(obj) = raw.as_object_mut() {
        for key in PLIST_META {
            obj.remove(*key);
        }
    }
    serde_json::to_string(&raw).map_err(|e| e.to_string())
}

/// source: mobileconfig source prefix — verbatim.
pub fn mobileconfig_source(plist: &str) -> String {
    format!("mobileconfig:{}", plist)
}
