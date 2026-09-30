//! Port of `src/openapi/index.ts`.
//!
//! Orchestration only: `fromSpec(options) → Result`. Pure spec parsing lives
//! in `openapi_spec`; execution lives in `openapi_runtime`.
//!
//! The Effect `HttpClient.HttpClient` service is represented by the sync
//! [`HttpTransport`](crate::openapi_runtime::HttpTransport) handle, which the
//! host supplies (same placement: the generated tools require it at call
//! time). Auth resolution keeps the source shape (`auth.resolve` called per
//! scheme at call time); the resolver is shared across per-operation plans
//! via `Arc` with identical observable semantics.

use crate::openapi_runtime::HttpTransport;
use crate::openapi_spec::{
    component_definitions, input_schema, is_method, non_empty_string, operation_input,
    operation_output, operation_path, operation_security_requirements, security_requirements,
    security_schemes, spec_server_url, validate_base_url,
};
use crate::openapi_types::{
    AdapterResult, AuthConfig, Operation, Options, Parsed, Plan, Skipped, ToolNode, Tools,
};
use std::collections::BTreeSet;

/// Builds a CodeMode tool subtree from an OpenAPI 3.x document, one tool per
/// operation. Auth is resolved host-side via `auth.resolve` and never
/// model-visible. Tools require an [`HttpTransport`]; unrepresentable
/// operations land in `skipped`.
///
/// Mirrors `fromSpec(options): Result`.
pub fn from_spec(options: Options, transport: HttpTransport) -> AdapterResult {
    let document = &options.spec;
    let schemes = security_schemes(document);
    let default_security = security_requirements(document.get("security"));
    let definitions = component_definitions(document);
    let paths = document
        .get("paths")
        .and_then(|p| p.as_object())
        .cloned()
        .unwrap_or_default();
    let mut used: BTreeSet<String> = BTreeSet::new();
    let mut namespaces: BTreeSet<String> = BTreeSet::new();
    let mut skipped: Vec<Skipped> = vec![];
    let mut tools = Tools::default();

    for (path, path_value) in &paths {
        let path_item = match path_value.as_object() {
            Some(o) => o,
            None => continue,
        };
        for (method, operation_value) in path_item {
            if !is_method(method) {
                continue;
            }
            let operation_obj = match operation_value.as_object() {
                Some(o) => o,
                None => continue,
            };
            let segments = operation_path(method, path, operation_obj, &used, &namespaces);
            let operation = Operation {
                operation_id: non_empty_string(operation_obj.get("operationId")),
                method: method.to_ascii_uppercase(),
                path: path.clone(),
                summary: non_empty_string(operation_obj.get("summary")),
                description: non_empty_string(operation_obj.get("description")),
            };
            let output_schema = match operation_output(document, operation_obj, &definitions) {
                Parsed::Ok(v) => v,
                Parsed::Err(reason) => {
                    skipped.push(Skipped {
                        method: operation.method.clone(),
                        path: path.clone(),
                        reason,
                    });
                    continue;
                }
            };
            let resolved_base_url = if let Some(base) = options.base_url.as_deref() {
                validate_base_url(base)
            } else if operation_obj.contains_key("servers") {
                spec_server_url(operation_obj)
            } else if path_item.contains_key("servers") {
                spec_server_url(path_item)
            } else {
                match document.as_object() {
                    Some(map) => spec_server_url(map),
                    None => Parsed::Err("spec declares no servers; pass baseUrl".to_string()),
                }
            };
            let base_url = match resolved_base_url {
                Parsed::Ok(v) => v,
                Parsed::Err(reason) => {
                    skipped.push(Skipped {
                        method: operation.method.clone(),
                        path: path.clone(),
                        reason,
                    });
                    continue;
                }
            };
            let input = match operation_input(document, path_item, operation_obj) {
                Parsed::Ok(v) => v,
                Parsed::Err(reason) => {
                    skipped.push(Skipped {
                        method: operation.method.clone(),
                        path: path.clone(),
                        reason,
                    });
                    continue;
                }
            };
            let security_value = match operation_security_requirements(
                operation_obj.get("security"),
                &default_security,
                &schemes,
            ) {
                Parsed::Ok(v) => v,
                Parsed::Err(reason) => {
                    skipped.push(Skipped {
                        method: operation.method.clone(),
                        path: path.clone(),
                        reason,
                    });
                    continue;
                }
            };
            let plan = Plan {
                operation: operation.clone(),
                url: format!("{}{}", base_url.trim_end_matches('/'), path),
                fields: input.fields.clone(),
                body: input.body.clone(),
                security: security_value,
                schemes: schemes.clone(),
                auth: options.auth.as_ref().map(|a| AuthConfig {
                    resolve: a.resolve.clone(),
                }),
                headers: options.headers.clone(),
            };
            used.insert(segments.join("."));
            for index in 0..segments.len().saturating_sub(1) {
                namespaces.insert(segments[..index + 1].join("."));
            }
            let description = operation
                .description
                .clone()
                .or_else(|| operation.summary.clone())
                .unwrap_or_else(|| format!("{} {}", operation.method, path));
            let schema = input_schema(&input.fields, &definitions);
            let run_transport = transport.clone();
            set_tool(
                &mut tools,
                &segments,
                crate::tool::make_tool(crate::tool::ToolOptions {
                    description,
                    input: crate::tool::SchemaType::Json(schema),
                    output: output_schema.map(crate::tool::SchemaType::Json),
                    run: Box::new(move |input| {
                        crate::openapi_runtime::invoke(&plan, input, &run_transport)
                            .map_err(|e| e.into_tool_failure())
                    }),
                }),
            );
        }
    }

    AdapterResult { tools, skipped }
}

fn set_tool(tools: &mut Tools, path: &[String], definition: crate::tool::Definition) {
    let (head, rest) = match path.split_first() {
        Some(pair) => pair,
        None => return,
    };
    if rest.is_empty() {
        tools.insert(head.clone(), ToolNode::Tool(definition));
        return;
    }
    // A colliding leaf becomes a namespace (mirrors the `!isRecord(child) ||
    // child._tag === "CodeModeTool"` reset).
    if !matches!(tools.get(head), Some(ToolNode::Namespace(_))) {
        tools.remove(head);
        tools.insert(head.clone(), ToolNode::Namespace(Tools::default()));
    }
    if let Some(ToolNode::Namespace(sub)) = tools.get_mut(head) {
        set_tool(sub, rest, definition);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn skips_operations_without_servers() {
        let options = Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "paths": {"/a": {"get": {"operationId": "a"}}}
            }),
            base_url: None,
            auth: None,
            headers: Default::default(),
        };
        let result = from_spec(options, HttpTransport::none());
        assert_eq!(result.skipped.len(), 1);
        assert!(result.skipped[0].reason.contains("pass baseUrl"));
    }

    #[test]
    fn base_url_override_places_tools() {
        let options = Options {
            spec: json!({
                "openapi": "3.1.0",
                "info": {"title": "t", "version": "1"},
                "paths": {"/a": {"get": {"operationId": "a", "responses": {"200": {"description": "ok"}}}}}
            }),
            base_url: Some("https://api.example.test".to_string()),
            auth: None,
            headers: Default::default(),
        };
        let result = from_spec(options, HttpTransport::none());
        assert!(result.skipped.is_empty());
        assert!(result.tools.contains_key("a"));
    }
}
