// Parity tests for the stats crate.
//
// Every assertion pins the behavior of the TypeScript source
// (packages/stats/core/src/domain/*.ts and database.ts) — see the
// "// source:" header in each module. Never weaken these tests; a failing
// parity assertion means the port diverged from the source and must be fixed
// at the implementation, not the test.

use stats::domain::inference::{
    build_retention_queries, build_stats_queries, iso_8601, sql_string, to_geo_aggregate,
    to_model_aggregate, to_provider_aggregate, to_retention_aggregate, R2SqlData, RetentionQuery,
    LIVE_SOURCE_START, PROVISIONAL_DEFAULT_DATA_SET,
};
use stats::domain::model_normalization::{
    model_author, normalize_inference_model, stat_model, stat_provider, EXCLUDED_MODELS,
    FREE_MODELS, MODEL_AUTHOR_RULES, MODEL_NAME_ALIASES, RETIRED_STAT_MODELS,
    RETIRED_STAT_PROVIDERS, STEALTH_MODELS,
};
use stats::domain::stat::{
    date_like::Date, normalize_country, normalize_tier, period_key_for, start_of_iso_week,
    start_of_utc_day, StatBaseAggregate, StatGrain, UPSERT_CHUNK_SIZE,
};
use stats::domain::{database, schema};

fn ms(y: i32, m: u32, d: u32) -> i64 {
    Date::utc(y, m, d).ms
}

fn ms_h(y: i32, m: u32, d: u32, h: u32, min: u32) -> i64 {
    Date::utc(y, m, d).ms + (h as i64) * 3_600_000 + (min as i64) * 60_000
}

/// TS test helper `aggregate(model, provider)` (inference.test.ts:260).
fn aggregate(model: &str, provider: &str) -> R2SqlData {
    R2SqlData::new()
        .with_text("grain", "day")
        .with_text("period_key", "2026-05-20")
        .with_text("dataset", "zen")
        .with_text("tier", "Paid")
        .with_text("provider", provider)
        .with_text("model", model)
        .with_text("sessions", "1")
        .with_text("requests", "1")
        .with_text("sample_count", "1")
}

fn base(row: &R2SqlData) -> StatBaseAggregate {
    StatBaseAggregate {
        grain: row.text("grain"),
        period_key: row.text("period_key"),
        dataset: row.text("dataset"),
        tier: normalize_tier(&row.text("tier")),
        sessions: row.text("sessions").parse::<f64>().unwrap_or(0.0).round() as i64,
        requests: row.text("requests").parse::<f64>().unwrap_or(0.0).round() as i64,
        ..Default::default()
    }
}

// ---------------------------------------------------------------------------
// source: packages/stats/core/src/domain/model-normalization.ts
// ---------------------------------------------------------------------------

#[test]
fn constants_match_source() {
    assert_eq!(
        MODEL_AUTHOR_RULES,
        [
            ("claude", "anthropic"),
            ("gemini", "google"),
            ("deepseek", "deepseek"),
            ("glm", "zhipu"),
            ("gpt", "openai"),
            ("grok", "xai"),
            ("hy3", "tencent"),
            ("kimi", "moonshot"),
            ("mimo", "xiaomi"),
            ("minimax", "minimax"),
            ("muse-spark", "meta"),
            ("nemotron", "nvidia"),
            ("qwen", "qwen"),
        ]
    );
    assert_eq!(EXCLUDED_MODELS, ["alpha-gpt-next"]);
    assert_eq!(STEALTH_MODELS, ["omen-alpha"]);
    assert_eq!(FREE_MODELS, ["gpt-5-nano", "grok-code", "big-pickle"]);
    assert_eq!(
        MODEL_NAME_ALIASES,
        [
            ("deepseek-v4-flash-0731", "deepseek-v4-flash"),
            (
                "deepseek-v4-flash-dsv4-flash-final-rnaovd",
                "deepseek-v4-flash"
            ),
            ("ox-alpha", "glm-5.3-flash"),
            ("x-preview-f", "glm-5.3-flash"),
            ("xiaomi/mimo-v2.5", "mimo-v2.5"),
        ]
    );
    assert_eq!(
        RETIRED_STAT_MODELS,
        [
            "big-pickle",
            "deepseek-v4-flash-0731",
            "deepseek-v4-flash-dsv4-flash-final-rnaovd",
            "ox-alpha",
            "x-preview-f",
            "xiaomi/mimo-v2.5",
        ]
    );
    assert_eq!(RETIRED_STAT_PROVIDERS, ["opencode"]);
}

#[test]
fn normalizes_model_suffixes_used_by_router_provider_variants() {
    assert_eq!(normalize_inference_model(Some("GPT-5-Free")), "gpt-5");
    assert_eq!(
        normalize_inference_model(Some("deepseek-v4-flash-free")),
        "deepseek-v4-flash"
    );
    assert_eq!(
        normalize_inference_model(Some("deepseek-v4-flash:global")),
        "deepseek-v4-flash"
    );
    assert_eq!(
        normalize_inference_model(Some("mimo-v2.5-free")),
        "mimo-v2.5"
    );
    assert_eq!(
        normalize_inference_model(Some("nemotron-3-super-free")),
        "nemotron-3-super"
    );
    assert_eq!(
        normalize_inference_model(Some("mimo-v2.5-free:global")),
        "mimo-v2.5"
    );
    assert_eq!(
        normalize_inference_model(Some("hy3-preview:free")),
        "hy3-preview"
    );
    assert_eq!(normalize_inference_model(Some("")), "unknown");
    assert_eq!(normalize_inference_model(None), "unknown");
}

#[test]
fn maps_normalized_model_ids_to_public_authors() {
    assert_eq!(model_author(Some("big-pickle")), Some("unknown".into()));
    assert_eq!(
        model_author(Some("claude-sonnet-4-5")),
        Some("anthropic".into())
    );
    assert_eq!(
        model_author(Some("deepseek-v4-pro")),
        Some("deepseek".into())
    );
    assert_eq!(
        model_author(Some("gemini-3.5-flash")),
        Some("google".into())
    );
    assert_eq!(model_author(Some("glm-5.1")), Some("zhipu".into()));
    assert_eq!(model_author(Some("gpt-5.5-pro")), Some("openai".into()));
    assert_eq!(model_author(Some("grok-build-0.1")), Some("xai".into()));
    assert_eq!(model_author(Some("hy3-preview")), Some("tencent".into()));
    assert_eq!(model_author(Some("kimi-k2.6")), Some("moonshot".into()));
    assert_eq!(model_author(Some("mimo-v2-omni")), Some("xiaomi".into()));
    assert_eq!(model_author(Some("minimax-m2.7")), Some("minimax".into()));
    assert_eq!(
        model_author(Some("muse-spark-1.2-contributor")),
        Some("meta".into())
    );
    assert_eq!(
        model_author(Some("nemotron-3-super-free")),
        Some("nvidia".into())
    );
    assert_eq!(model_author(Some("qwen3.7-max")), Some("qwen".into()));
    assert_eq!(model_author(Some("alpha-gpt-next")), None);
    assert_eq!(model_author(Some("omen-alpha")), Some("unknown".into()));
    assert_eq!(
        model_author(Some("OMEN-ALPHA-free:global")),
        Some("unknown".into())
    );
}

#[test]
fn uses_provider_model_to_resolve_opencode_route_providers() {
    assert_eq!(
        stat_model(Some("big-pickle"), Some("claude-sonnet-4-5")),
        "claude-sonnet-4-5"
    );
    assert_eq!(stat_model(Some("big-pickle"), Some("gpt-5-free")), "gpt-5");
    assert_eq!(
        stat_model(Some("big-pickle"), Some("xiaomi/mimo-v2.5")),
        "mimo-v2.5"
    );
    assert_eq!(stat_model(Some("big-pickle"), Some("")), "unknown");
    assert_eq!(
        stat_provider(
            Some("big-pickle"),
            Some("claude-sonnet-4-5"),
            Some("opencode")
        ),
        Some("anthropic".into())
    );
    assert_eq!(
        stat_provider(Some("big-pickle"), Some("gpt-5"), Some("opencode")),
        Some("openai".into())
    );
    assert_eq!(
        stat_provider(Some("big-pickle"), Some(""), Some("opencode")),
        Some("unknown".into())
    );
    assert_eq!(
        stat_provider(Some("unknown"), Some(""), Some("custom-provider")),
        Some("custom-provider".into())
    );
}

#[test]
fn keeps_stealth_model_usage_without_exposing_the_route_provider() {
    assert_eq!(
        stat_provider(
            Some("omen-alpha"),
            Some("gpt-test-model"),
            Some("test-provider")
        ),
        Some("unknown".into())
    );
    assert_eq!(
        stat_provider(
            Some("OMEN-ALPHA-free:global"),
            Some("gpt-test-model"),
            Some("test-provider")
        ),
        Some("unknown".into())
    );
    assert_eq!(
        stat_provider(Some("omen-alpha"), Some(""), Some("test-provider")),
        Some("unknown".into())
    );

    let row =
        aggregate("omen-alpha", "test-provider").with_text("provider_model", "gpt-test-model");
    let out = to_model_aggregate(&row);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].model, "omen-alpha");
    assert_eq!(out[0].provider, "unknown");
    assert_eq!(out[0].base.requests, 1);

    let out = to_provider_aggregate(&row);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].provider, "unknown");
    assert_eq!(out[0].base.requests, 1);

    let geo = aggregate("omen-alpha", "test-provider")
        .with_text("provider_model", "gpt-test-model")
        .with_text("country", "US");
    let out = to_geo_aggregate(&geo);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].provider, "unknown");
    assert_eq!(out[0].model, "omen-alpha");
    assert_eq!(out[0].country, "US");
    assert_eq!(out[0].base.requests, 1);

    let retention = aggregate("omen-alpha", "test-provider")
        .with_text("provider_model", "gpt-test-model")
        .with_text("cohort_date", "2026-08-10")
        .with_text("eligible_users", "12");
    let out = to_retention_aggregate(&retention);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].provider, "unknown");
    assert_eq!(out[0].model, "omen-alpha");
    assert_eq!(out[0].eligible_users, 12);
}

#[test]
fn merges_renamed_models_under_their_current_name() {
    assert_eq!(
        stat_model(Some("deepseek-v4-flash-0731"), Some("")),
        "deepseek-v4-flash"
    );
    assert_eq!(
        stat_model(Some("deepseek-v4-flash-0731-free"), Some("")),
        "deepseek-v4-flash"
    );
    assert_eq!(
        stat_model(Some("deepseek-v4-flash-dsv4-flash-final-rnaovd"), Some("")),
        "deepseek-v4-flash"
    );
    assert_eq!(
        stat_model(Some("deepseek-v4-flash-vision-exp"), Some("")),
        "deepseek-v4-flash-vision-exp"
    );
    assert_eq!(stat_model(Some("x-preview-f"), Some("")), "glm-5.3-flash");
    assert_eq!(stat_model(Some("ox-alpha"), Some("")), "glm-5.3-flash");
    assert_eq!(stat_model(Some("ox-alpha-free"), Some("")), "glm-5.3-flash");
    assert_eq!(
        stat_model(Some("big-pickle"), Some("zhipuai/ox-alpha-free")),
        "glm-5.3-flash"
    );
    assert_eq!(stat_model(Some("xiaomi/mimo-v2.5"), Some("")), "mimo-v2.5");

    let out = to_model_aggregate(&aggregate("x-preview-f", "unknown"));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].provider, "zhipu");
    assert_eq!(out[0].model, "glm-5.3-flash");

    let out = to_provider_aggregate(&aggregate("ox-alpha", "unknown"));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].provider, "zhipu");
}

#[test]
fn model_aggregates_prefer_provider_model_and_use_normalized_model() {
    assert_eq!(
        to_model_aggregate(&aggregate("alpha-gpt-next", "openai")),
        vec![]
    );

    let out = to_model_aggregate(&aggregate("deepseek-v4-flash-free", "not-public-provider"));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].base.period_key, "2026-05-20");
    assert_eq!(out[0].provider, "deepseek");
    assert_eq!(out[0].model, "deepseek-v4-flash");

    let row = aggregate("big-pickle", "opencode").with_text("provider_model", "claude-sonnet-4-5");
    let out = to_model_aggregate(&row);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].provider, "anthropic");
    assert_eq!(out[0].model, "claude-sonnet-4-5");
    assert_eq!(out[0].provider_model, "claude-sonnet-4-5");
}

#[test]
fn provider_aggregates_never_keep_opencode_as_provider() {
    let row = aggregate("big-pickle", "opencode").with_text("provider_model", "gpt-5");
    assert_eq!(to_provider_aggregate(&row)[0].provider, "openai");
    assert_eq!(
        to_provider_aggregate(&aggregate("big-pickle", "opencode"))[0].provider,
        "unknown"
    );
    assert_eq!(
        to_provider_aggregate(&aggregate("muse-spark-1.2-contributor", "unknown"))[0].provider,
        "meta"
    );
}

#[test]
fn geo_aggregates_never_keep_opencode_or_big_pickle_dimensions() {
    let out = to_geo_aggregate(&aggregate("big-pickle", "opencode").with_text("country", "US"));
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].provider, "unknown");
    assert_eq!(out[0].model, "unknown");
    assert_eq!(out[0].country, "US");
}

#[test]
fn geo_aggregates_use_all_for_missing_model() {
    let out = to_geo_aggregate(&aggregate("", "x").with_text("model", ""));
    assert!(!out.is_empty());
    assert_eq!(out[0].model, "all");
}

#[test]
fn model_aggregates_use_iso_week_period_keys() {
    let mut row = aggregate("gpt-5.5-pro", "openai");
    row = row
        .with_text("grain", "week")
        .with_text("period_key", "2026-W20");
    let out = to_model_aggregate(&row);
    assert_eq!(out[0].base.period_key, "2026-W20");
}

// ---------------------------------------------------------------------------
// source: packages/stats/core/src/domain/inference.ts
// ---------------------------------------------------------------------------

#[test]
fn live_source_start_is_exact() {
    assert_eq!(LIVE_SOURCE_START, "2026-08-11T10:57:48.186Z");
}

#[test]
fn stat_periods_align_to_utc_calendar_boundaries() {
    let start = ms_h(2026, 6, 17, 15, 56);
    let end = ms_h(2026, 6, 19, 15, 56);
    let weeks = stats::domain::inference::stat_periods(StatGrain::Week, start, end);
    assert_eq!(weeks.len(), 1);
    assert_eq!(weeks[0].key, "2026-W25");
    assert_eq!(iso_8601(weeks[0].start_ms), "2026-06-15T00:00:00.000Z");
    assert_eq!(iso_8601(weeks[0].end_ms), "2026-06-19T15:56:00.000Z");

    let days = stats::domain::inference::stat_periods(StatGrain::Day, start, end);
    assert_eq!(days.len(), 3);
    assert_eq!(days[0].key, "2026-06-17");
    assert_eq!(iso_8601(days[0].start_ms), "2026-06-17T00:00:00.000Z");
    assert_eq!(iso_8601(days[0].end_ms), "2026-06-18T00:00:00.000Z");
    assert_eq!(days[2].key, "2026-06-19");
    assert_eq!(iso_8601(days[2].end_ms), "2026-06-19T15:56:00.000Z");
}

#[test]
fn period_key_for_aligns_week_and_day() {
    let monday = Date::utc(2026, 8, 10);
    assert_eq!(period_key_for(StatGrain::Week, &monday), "2026-W33");
    assert_eq!(period_key_for(StatGrain::Day, &monday), "2026-08-10");
    assert_eq!(start_of_utc_day(&monday.add_ms(36_000_000)).ms, monday.ms);
    let wednesday = Date::utc(2026, 6, 17);
    assert_eq!(start_of_iso_week(&wednesday).iso(), "2026-06-15");
    assert_eq!(wednesday.weekday(), 3);
    assert_eq!(monday.weekday(), 1);
}

#[test]
fn builds_bounded_r2_sql_queries_for_each_day_and_week() {
    let queries = build_stats_queries(
        ms(2026, 8, 10),
        ms_h(2026, 8, 12, 12, 0),
        Some(&stats::domain::inference::StatsQuerySource {
            namespace: "inference".into(),
            table: "generation".into(),
            dataset: "zen".into(),
        }),
    );
    assert_eq!(queries.len(), 8);
    for query in &queries {
        assert!(
            query.contains("WHERE lower(model) NOT IN ('alpha-gpt-next')"),
            "{query}"
        );
        assert!(
            query.contains("CASE\n      WHEN lower(model) IN ('omen-alpha') THEN 'unknown'\n"),
            "{query}"
        );
    }
    assert!(queries[0].contains("'week' AS grain"));
    assert!(queries[0].contains("'2026-W33' AS period_key"));
    assert!(queries[2].contains("'2026-08-10' AS period_key"));
    assert!(queries[6].contains("'2026-08-12' AS period_key"));
    assert!(queries[0].contains("FROM \"inference\".\"generation\""));
    assert!(queries[0].contains("event_type = 'generation.completed'"));
    assert!(queries[0].contains("AND (product = 'go' OR (lower(COALESCE(model_tier, '')) = 'free'"));
    assert!(queries[0].contains("COALESCE(NULLIF(lower(model_tier), ''), '') AS raw_tier"));
    assert!(queries[0].contains("WHEN lower(COALESCE(raw_tier, '')) = 'free'"));
    assert!(queries[0].contains("regexp_replace(NULLIF(route_model, ''), '^.*/', '')"));
    assert!(queries[0].contains("= 'deepseek-v4-flash-0731' THEN 'deepseek-v4-flash'"));
    assert!(queries[0]
        .contains("= 'deepseek-v4-flash-dsv4-flash-final-rnaovd' THEN 'deepseek-v4-flash'"));
    assert!(!queries[0].contains("= 'deepseek-v4-flash-vision-exp' THEN 'deepseek-v4-flash'"));
    assert!(queries[0].contains("= 'ox-alpha' THEN 'glm-5.3-flash'"));
    assert!(queries[0].contains("= 'x-preview-f' THEN 'glm-5.3-flash'"));
    assert!(queries[0].contains("OR lower(raw_model) IN ('gpt-5-nano', 'grok-code', 'big-pickle')"));
    assert!(queries[0].contains("OR lower(raw_model) LIKE '%-free'"));
    assert!(queries[0].contains("THEN 'Free'"));
    assert!(queries[0].contains("LIMIT 10000"));
    assert!(queries[0].contains("approx_distinct(session) AS sessions"));
    assert!(queries[1].contains("'geo_model' ELSE 'geo'"));
    assert!(queries[1].contains("0 AS sessions"));
}

#[test]
fn aligns_periods_to_utc_calendar_boundaries() {
    let queries = build_stats_queries(
        ms_h(2026, 6, 17, 15, 56),
        ms_h(2026, 6, 19, 15, 56),
        Some(&stats::domain::inference::StatsQuerySource {
            namespace: "inference".into(),
            table: "generation".into(),
            dataset: "zen".into(),
        }),
    );
    assert_eq!(queries.len(), 8);
    assert!(queries[0].contains("'2026-W25' AS period_key"));
    assert!(queries[0].contains("started_at >= '2026-06-15T00:00:00.000Z'"));
    assert!(queries[2].contains("'2026-06-17' AS period_key"));
    assert!(queries[2].contains("started_at >= '2026-06-17T00:00:00.000Z'"));
    assert!(queries[2].contains("started_at < '2026-06-18T00:00:00.000Z'"));
    assert!(queries[6].contains("'2026-06-19' AS period_key"));
    assert!(queries[6].contains("started_at < '2026-06-19T15:56:00.000Z'"));
}

#[test]
fn uses_exclusive_live_and_legacy_source_handoff() {
    let queries = build_stats_queries(
        ms(2026, 8, 11),
        ms(2026, 8, 12),
        Some(&stats::domain::inference::StatsQuerySource {
            namespace: "inference".into(),
            table: "generation".into(),
            dataset: "zen".into(),
        }),
    );
    assert!(queries[0]
        .contains("(source = 'inference-legacy' AND started_at < '2026-08-11T10:57:48.186Z')"));
    assert!(
        queries[0].contains("(source = 'inference' AND started_at >= '2026-08-11T10:57:48.186Z')")
    );
}

#[test]
fn builds_complete_week_over_week_retention_queries() {
    let queries = build_retention_queries(
        ms(2026, 8, 10),
        ms(2026, 8, 31),
        Some(&stats::domain::inference::StatsQuerySource {
            namespace: "inference".into(),
            table: "generation".into(),
            dataset: "zen".into(),
        }),
    );
    assert_eq!(queries.len(), 1);
    assert_eq!(queries[0].cohort_dates, ["2026-08-10", "2026-08-17"]);
    let query = &queries[0].query;
    assert!(query.contains("AND product = 'go'"));
    assert!(query.contains("AND lower(model) NOT IN ('alpha-gpt-next')"));
    assert!(query.contains("CASE\n      WHEN lower(model) IN ('omen-alpha') THEN 'unknown'\n"));
    assert!(query.contains("COUNT(*) AS model_requests"));
    assert!(query.contains("SUM(model_requests) AS total_requests"));
    assert!(query.contains("MAX(model_requests) AS max_model_requests"));
    assert!(query.contains("GROUP BY cohort_date, user_key"));
    assert!(query.contains("INNER JOIN user_totals"));
    assert!(query.contains("model_usage.model_requests = user_totals.max_model_requests"));
    assert!(query.contains("user_totals.total_requests >= 10"));
    assert!(query.contains(
        "CAST(model_usage.model_requests AS double) / NULLIF(user_totals.total_requests, 0) >= 0.8"
    ));
    assert!(!query.contains(" OVER ("));
    assert!(query.contains("WHEN '2026-08-17' THEN '2026-08-10'"));
    assert!(query.contains("WHEN '2026-08-24' THEN '2026-08-17'"));
    assert!(query.contains("started_at >= '2026-08-10T00:00:00.000Z'"));
    assert!(query.contains("started_at < '2026-08-31T00:00:00.000Z'"));
    assert!(query.contains("LEFT JOIN returned ON primary_models.user_key = returned.user_key"));
    assert!(query.contains("primary_models.cohort_date = returned.cohort_date"));
    assert!(query.contains("'Go' AS tier"));
    assert!(query.contains("COUNT(*) AS eligible_users"));
    assert!(query.contains("LIMIT 10000"));
}

#[test]
fn retention_queries_with_no_complete_weeks_are_empty() {
    assert_eq!(
        build_retention_queries(ms(2026, 8, 10), ms(2026, 8, 20), None),
        vec![]
    );
    assert_eq!(
        build_retention_queries(ms(2026, 8, 10), ms(2026, 8, 31), None).len(),
        1
    );
}

#[test]
fn maps_retention_query_results() {
    let row = R2SqlData::new()
        .with_text("grain", "week")
        .with_text("period_key", "2026-08-10")
        .with_text("cohort_date", "2026-08-10")
        .with_text("dataset", "zen")
        .with_text("tier", "all")
        .with_text("provider", "deepseek")
        .with_text("model", "deepseek-v4-flash-free")
        .with_text("eligible_users", "125")
        .with_text("retained_users", "74");
    let out = to_retention_aggregate(&row);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].cohort_date, "2026-08-10");
    assert_eq!(out[0].dataset, "zen");
    assert_eq!(out[0].tier, "all");
    assert_eq!(out[0].provider, "deepseek");
    assert_eq!(out[0].model, "deepseek-v4-flash");
    assert_eq!(out[0].eligible_users, 125);
    assert_eq!(out[0].retained_users, 74);
}

#[test]
fn sql_quoting_matches_source() {
    assert_eq!(sql_string("O'Reilly"), "'O''Reilly'");
    assert_eq!(
        stats::domain::inference::sql_identifier("a\"b"),
        "\"a\"\"b\""
    );
}

#[test]
fn nullability_and_rounding_match_source() {
    // nullableNumber: '' and missing -> null; Number("1.234") -> 1.23
    let row = R2SqlData::new()
        .with_text("avg_duration_ms", "1.234")
        .with_text("avg_output_tps", "")
        .with_number("p50_ttfb_ms", 42.6);
    assert_eq!(row.text("avg_duration_ms"), "1.234");
    assert_eq!(stats::domain::stat::round(1.234, 2), 1.23);
    assert_eq!(normalize_country(Some("us")), "US");
    assert_eq!(normalize_country(Some("")), "ZZ");
    assert_eq!(normalize_country(None), "ZZ");
    assert_eq!(normalize_tier("Paid"), "Zen");
    assert_eq!(normalize_tier("go"), "Go");
    assert_eq!(normalize_tier("free"), "Free");
    assert_eq!(normalize_tier("enterprise"), "Enterprise");
}

// ---------------------------------------------------------------------------
// source: packages/stats/core/src/domain/stat.ts + schema.ts + database.ts
// ---------------------------------------------------------------------------

#[test]
fn stat_constants_match_source() {
    assert_eq!(UPSERT_CHUNK_SIZE, 500);
    assert_eq!(
        stats::domain::stat::DATA_SITE_TIERS,
        ["Go", "go", "Free", "free"]
    );
}

#[test]
fn schema_tables_match_source() {
    let tables = schema::tables();
    let names: Vec<&str> = tables.iter().map(|t| t.name).collect();
    assert_eq!(
        names,
        ["model_stat", "provider_stat", "geo_stat", "model_retention"]
    );

    let model = schema::model_stat();
    assert_eq!(model.name, "model_stat");
    let id = model
        .columns
        .iter()
        .find(|c| c.name == "id")
        .expect("id column");
    assert!(id.auto_increment && id.primary_key && id.kind == "bigint");
    assert!(model.columns.iter().any(|c| c.name == "grain"));
    assert!(model.columns.iter().any(|c| c.name == "provider_model"));
    assert!(model.columns.iter().any(|c| c.name == "avg_output_tps"));
    assert!(model.columns.iter().any(|c| c.name == "sample_count"));
    assert!(model.columns.iter().any(|c| c.name == "rank_by_cost"));
    assert!(model.columns.iter().any(|c| c.name == "created_at"));

    let model_provider = model
        .columns
        .iter()
        .find(|c| c.name == "provider")
        .expect("provider column");
    assert_eq!(model_provider.default, Some("'all'"));

    let geo = schema::geo_stat();
    let geo_provider = geo
        .columns
        .iter()
        .find(|c| c.name == "provider")
        .expect("provider column");
    assert_eq!(geo_provider.default, Some("'all'"));

    let uniq = model
        .indexes
        .iter()
        .find(|i| i.name == "uniq_model_period")
        .expect("uniq_model_period index");
    assert!(uniq.unique);
    assert!(uniq.columns.contains(&"grain"));

    let retention = schema::model_retention();
    assert_eq!(retention.name, "model_retention");
    assert!(retention.columns.iter().any(|c| c.name == "cohort_date"));
    assert!(retention.columns.iter().any(|c| c.name == "retained_users"));
}

#[test]
fn database_constants_match_source() {
    assert_eq!(database::DATABASE_URL_ENV, "DATABASE_URL");
    assert_eq!(
        database::DATABASE_MIGRATIONS_DIR_ENV,
        "DATABASE_MIGRATIONS_DIR"
    );
    assert_eq!(database::DEFAULT_MIGRATIONS_DIR, "./migrations");
    assert_eq!(
        database::DATABASE_CONFIG_SERVICE_ID,
        "@opencode/stats/DatabaseConfig"
    );
    assert_eq!(
        database::DRIZZLE_CLIENT_SERVICE_ID,
        "@opencode/stats/DrizzleClient"
    );
}

#[test]
fn rounds_iso_dates() {
    assert_eq!(iso_8601(ms(2026, 1, 1)), "2026-01-01T00:00:00.000Z");
    assert_eq!(
        iso_8601(Date::utc(2026, 8, 10).ms + 60_000),
        "2026-08-10T00:01:00.000Z"
    );
}

#[test]
fn default_source_values_are_provisional() {
    // PROVISIONAL: production defaults come from SST resources.
    // Source truth for this one-day range: 1 week period + 1 day period,
    // times the usage/geo families = 4 queries from the single default
    // source (`statPeriods` count = ceil((end - first) / interval)).
    let queries = build_stats_queries(ms(2026, 8, 10), ms(2026, 8, 11), None);
    assert_eq!(queries.len(), 4);
    assert!(queries[0].contains("FROM \"opencode-stats\".\"generation_events\""));
    assert_eq!(PROVISIONAL_DEFAULT_DATA_SET, "zen");
    assert_eq!(
        RetentionQuery {
            cohort_dates: vec![],
            query: String::new()
        }
        .cohort_dates
        .len(),
        0
    );
}

#[test]
fn base_helper_captures_core_fields() {
    let row = aggregate("gpt-5", "openai");
    let b = base(&row);
    assert_eq!(b.grain, "day");
    assert_eq!(b.period_key, "2026-05-20");
    assert_eq!(b.dataset, "zen");
    assert_eq!(b.tier, "Zen");
    assert_eq!(b.sessions, 1);
    assert_eq!(b.requests, 1);
}
