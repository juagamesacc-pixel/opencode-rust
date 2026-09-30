//! Rust port of `packages/app/src/hooks/provider-catalog.ts` (opencode v1.18.30).
//!
//! Source 37 lines. Exports: `selectProviderCatalog`, `resolveDefaultModel`.
//! 1:1 faithful - same names/behavior/edge cases.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProviderEntry {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct NormalizedProviderListResponse {
    pub all: HashMap<String, ProviderEntry>,
    pub connected: Vec<String>,
    pub default: HashMap<String, String>,
    pub defaultModel: Option<DefaultModel>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DefaultModel {
    pub providerID: String,
    pub modelID: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectoryCatalog {
    pub ready: bool,
    pub providers: NormalizedProviderListResponse,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ProviderCatalogInput {
    Explicit {
        directory: Option<String>,
        catalog: Option<DirectoryCatalog>,
    },
    WithGlobal {
        directory: Option<String>,
        catalog: Option<DirectoryCatalog>,
        global: NormalizedProviderListResponse,
    },
}

fn emptyProviderCatalog() -> NormalizedProviderListResponse {
    NormalizedProviderListResponse {
        all: HashMap::new(),
        connected: Vec::new(),
        default: HashMap::new(),
        defaultModel: None,
    }
}

#[allow(non_snake_case)]
pub fn selectProviderCatalog(input: ProviderCatalogInput) -> NormalizedProviderListResponse {
    // Mirrors TS exactly:
    //   if (input.directory && input.catalog?.ready) return input.catalog.providers
    //   if (input.explicit) return emptyProviderCatalog
    //   return input.global
    let (directory, catalog_ready_providers, explicit, global) = match &input {
        ProviderCatalogInput::Explicit { directory, catalog } => (
            directory.clone(),
            catalog.as_ref().and_then(|cat| {
                if cat.ready {
                    Some(cat.providers.clone())
                } else {
                    None
                }
            }),
            true,
            None,
        ),
        ProviderCatalogInput::WithGlobal {
            directory,
            catalog,
            global,
        } => (
            directory.clone(),
            catalog.as_ref().and_then(|cat| {
                if cat.ready {
                    Some(cat.providers.clone())
                } else {
                    None
                }
            }),
            false,
            Some(global.clone()),
        ),
    };
    if let Some(dir) = directory {
        if !dir.is_empty() {
            if let Some(providers) = catalog_ready_providers {
                return providers;
            }
        }
    }
    if explicit {
        return emptyProviderCatalog();
    }
    global.unwrap_or_else(emptyProviderCatalog)
}

#[allow(non_snake_case)]
pub fn resolveDefaultModel(
    current: Option<Option<DefaultModel>>,
    legacy: Option<&str>,
) -> Option<DefaultModel> {
    // JS: if (current !== undefined) return current ?? undefined
    // In Rust, we model `Option<Option<DefaultModel>>` where outer None = undefined, Some(None)=null, Some(Some(x))=value
    if let Some(inner) = current {
        return inner;
    }
    if let Some(leg) = legacy {
        if leg.is_empty() {
            return None;
        }
        let mut parts = leg.splitn(2, '/');
        let providerID = parts.next().unwrap_or("").to_string();
        let modelID = parts.next().unwrap_or("").to_string();
        return Some(DefaultModel {
            providerID,
            modelID,
        });
    }
    None
}
