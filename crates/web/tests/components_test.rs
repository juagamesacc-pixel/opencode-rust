// Parity tests for `src/components`.
use web::components;

#[test]
fn component_table_matches_source_tree() {
    assert_eq!(components::COMPONENTS.len(), 28);
    let framework_count = |framework| {
        components::COMPONENTS
            .iter()
            .filter(|item| item.framework == framework)
            .count()
    };
    assert_eq!(
        framework_count(components::Framework::Astro),
        7,
        "seven .astro components"
    );
    assert_eq!(
        framework_count(components::Framework::Solid),
        12,
        "twelve .tsx islands"
    );
    assert_eq!(
        framework_count(components::Framework::Css),
        9,
        "nine stylesheets"
    );

    let part = components::find("share/part.tsx").expect("part component");
    assert_eq!(part.lines, 817);
    assert!(part.exports.contains(&"Part"));

    let share = components::find("Share.tsx").expect("Share island");
    assert_eq!(share.lines, 647);
    assert!(share.exports.contains(&"Share"));

    assert!(components::find("share/missing.tsx").is_none());
}

#[test]
fn export_lists_match_source_modules() {
    assert_eq!(components::SHARE_TSX_EXPORTS, ["Share", "fromV1"]);
    assert_eq!(
        components::SHARE_COMMON_TSX_EXPORTS,
        [
            "ShareMessages",
            "ShareI18nProvider",
            "useShareMessages",
            "normalizeLocale",
            "formatNumber",
            "formatCurrency",
            "formatCount",
            "AnchorIcon",
            "createOverflow",
            "formatDuration"
        ]
    );
    assert_eq!(
        components::ICONS_CUSTOM_TSX_EXPORTS,
        [
            "IconOpenAI",
            "IconAnthropic",
            "IconGemini",
            "IconOpencode",
            "IconMeta",
            "IconRobot",
            "IconBrain"
        ]
    );
    assert_eq!(components::SHARE_PART_TSX_EXPORTS.len(), 14);
    assert_eq!(components::SHARE_PART_TSX_EXPORTS[0], "PartProps");
    assert_eq!(components::ICONS_INDEX_EXPORTS.len(), 299);
    assert_eq!(components::ICONS_INDEX_EXPORTS[0], "IconAcademicCap");
    assert!(components::ICONS_INDEX_EXPORTS.contains(&"IconCommand"));

    let mut total = 0;
    for component in components::COMPONENTS {
        total += component.exports.len();
    }
    assert_eq!(total, components::TOTAL_EXPORTS);
    assert_eq!(components::TOTAL_EXPORTS, 339);
}

#[test]
fn starlight_overrides_match_astro_config() {
    assert_eq!(components::STARLIGHT_OVERRIDES.len(), 6);
    let slots: Vec<&str> = components::STARLIGHT_OVERRIDES
        .iter()
        .map(|item| item.0)
        .collect();
    assert_eq!(
        slots,
        [
            "Hero",
            "Head",
            "Header",
            "Footer",
            "LanguageSelect",
            "SiteTitle"
        ]
    );
    for (_, file) in components::STARLIGHT_OVERRIDES {
        let rel = file
            .strip_prefix("./src/components/")
            .expect("components path");
        assert!(components::find(rel).is_some(), "override file {}", file);
    }
}
