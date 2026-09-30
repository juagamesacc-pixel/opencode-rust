#![allow(clippy::all)]
// hermetic: models-dev catalog fetch shape with fixture strings

#[test]
fn catalog_url_and_user_agent() {
    assert_eq!(
        core::models_dev::fetch_url("https://models.opencode.ai"),
        "https://models.opencode.ai/api.json"
    );
    assert_eq!(
        core::models_dev::fetch_url("https://models.opencode.ai/"),
        "https://models.opencode.ai/api.json"
    );
    let req = core::models_dev::build_fetch_request(
        "https://models.opencode.ai",
        "opencode/test/1/client",
    )
    .unwrap();
    assert_eq!(req.url().as_str(), "https://models.opencode.ai/api.json");
    assert_eq!(
        req.headers().get("User-Agent").unwrap().to_str().unwrap(),
        "opencode/test/1/client"
    );
}

#[test]
fn catalog_cache_filepath() {
    let p = core::models_dev::cache_filepath("/tmp/cache", "https://models.opencode.ai");
    assert_eq!(p, "/tmp/cache/models.json");
    let p2 = core::models_dev::cache_filepath("/tmp/cache", "https://custom.example.com");
    assert!(p2.starts_with("/tmp/cache/models-"));
    assert!(p2.ends_with(".json"));
    assert_ne!(p, p2);
}

#[test]
fn catalog_parse_fixture() {
    let json = r#"{
        "openai":{"id":"openai","name":"OpenAI","env":["OPENAI_API_KEY"],"models":{
            "gpt-4":{"id":"gpt-4","name":"GPT-4","release_date":"2024-01-01","attachment":false,"reasoning":false,"temperature":true,"tool_call":true,"limit":{"context":128000,"output":4096}}
        }},
        "anthropic":{"id":"anthropic","name":"Anthropic","env":["ANTHROPIC_API_KEY"],"api":"https://api.anthropic.com","models":{}}
    }"#;
    let cat = core::models_dev::parse_catalog_json(json).unwrap();
    assert!(cat.contains_key("openai"));
    assert!(cat.contains_key("anthropic"));
    assert_eq!(cat["openai"].models["gpt-4"].name, "GPT-4");
    assert_eq!(cat["openai"].models["gpt-4"].limit.context, 128000.0);
}

#[test]
fn user_agent_format() {
    assert_eq!(
        core::models_dev::user_agent("production", "1.18.30", "cli"),
        "opencode/production/1.18.30/cli"
    );
}
