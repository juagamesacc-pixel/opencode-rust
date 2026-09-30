//! Rust port of `packages/app/src/hooks/provider-catalog.test.ts` (opencode v1.18.30).
//!
//! Source 77 lines, 7 tests. 1:1 parity — same inputs/assertions.
//! `toBe` (identity) is modeled as equality on cloned values; `toEqual`
//! (structural) as equality against a fresh empty catalog.

use app::hooks::provider_catalog::{
    resolveDefaultModel, selectProviderCatalog, DefaultModel, DirectoryCatalog,
    NormalizedProviderListResponse, ProviderCatalogInput, ProviderEntry,
};
use std::collections::HashMap;

fn catalog(id: &str) -> NormalizedProviderListResponse {
    NormalizedProviderListResponse {
        all: HashMap::from([(
            id.to_string(),
            ProviderEntry {
                id: id.to_string(),
                name: id.to_string(),
            },
        )]),
        connected: vec![id.to_string()],
        default: HashMap::from([(id.to_string(), format!("{id}-model"))]),
        defaultModel: None,
    }
}

fn empty_catalog() -> NormalizedProviderListResponse {
    NormalizedProviderListResponse {
        all: HashMap::new(),
        connected: Vec::new(),
        default: HashMap::new(),
        defaultModel: None,
    }
}

#[test]
fn selects_the_ready_catalog_for_an_explicit_directory() {
    let directory = catalog("directory");
    assert_eq!(
        selectProviderCatalog(ProviderCatalogInput::Explicit {
            directory: Some("/repo".to_string()),
            catalog: Some(DirectoryCatalog {
                ready: true,
                providers: directory.clone()
            }),
        }),
        directory
    );
}

#[test]
fn returns_an_empty_catalog_while_an_explicit_directory_is_unresolved() {
    assert_eq!(
        selectProviderCatalog(ProviderCatalogInput::Explicit {
            directory: None,
            catalog: None
        }),
        empty_catalog()
    );
    assert_eq!(
        selectProviderCatalog(ProviderCatalogInput::Explicit {
            directory: Some("/repo".to_string()),
            catalog: Some(DirectoryCatalog {
                ready: false,
                providers: catalog("directory")
            }),
        }),
        empty_catalog()
    );
}

#[test]
fn uses_the_route_catalog_when_it_is_ready() {
    let directory = catalog("directory");
    assert_eq!(
        selectProviderCatalog(ProviderCatalogInput::WithGlobal {
            directory: Some("/repo".to_string()),
            catalog: Some(DirectoryCatalog {
                ready: true,
                providers: directory.clone()
            }),
            global: catalog("global"),
        }),
        directory
    );
}

#[test]
fn falls_back_to_the_global_catalog_for_route_consumers() {
    let global = catalog("global");
    assert_eq!(
        selectProviderCatalog(ProviderCatalogInput::WithGlobal {
            directory: None,
            catalog: None,
            global: global.clone(),
        }),
        global
    );
    assert_eq!(
        selectProviderCatalog(ProviderCatalogInput::WithGlobal {
            directory: Some("/repo".to_string()),
            catalog: Some(DirectoryCatalog {
                ready: false,
                providers: catalog("directory")
            }),
            global: global.clone(),
        }),
        global
    );
}

#[test]
fn uses_the_current_server_default_model() {
    assert_eq!(
        resolveDefaultModel(
            Some(Some(DefaultModel {
                providerID: "openai".to_string(),
                modelID: "gpt-5".to_string(),
            })),
            Some("anthropic/claude"),
        ),
        Some(DefaultModel {
            providerID: "openai".to_string(),
            modelID: "gpt-5".to_string()
        })
    );
}

#[test]
fn does_not_use_legacy_config_when_the_current_server_has_no_default() {
    // Mirrors `resolveDefaultModel(null, "anthropic/claude")` → undefined.
    assert_eq!(
        resolveDefaultModel(Some(None), Some("anthropic/claude")),
        None
    );
}

#[test]
fn uses_config_for_legacy_servers() {
    // Mirrors `resolveDefaultModel(undefined, "anthropic/claude")`.
    assert_eq!(
        resolveDefaultModel(None, Some("anthropic/claude")),
        Some(DefaultModel {
            providerID: "anthropic".to_string(),
            modelID: "claude".to_string()
        })
    );
}
