use super::*;

#[test]
fn import_openapi3_simple_get() {
    let spec = r#"{
        "openapi": "3.0.0",
        "info": { "title": "Test", "version": "1.0" },
        "servers": [{ "url": "https://api.example.com" }],
        "paths": {
            "/users": {
                "get": {
                    "summary": "List users",
                    "tags": ["Users"]
                }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    assert_eq!(collections.len(), 1);
    assert_eq!(collections[0].name, "Users");
    assert_eq!(collections[0].requests.len(), 1);
    assert_eq!(collections[0].requests[0].name, "List users");
    assert_eq!(
        collections[0].requests[0].url,
        "https://api.example.com/users"
    );
    assert_eq!(collections[0].requests[0].method, HttpMethod::Get);
}

#[test]
fn import_openapi3_multiple_tags() {
    let spec = r#"{
        "openapi": "3.0.0",
        "info": { "title": "Test", "version": "1.0" },
        "servers": [{ "url": "https://api.example.com" }],
        "paths": {
            "/users": {
                "get": { "summary": "List users", "tags": ["Users"] },
                "post": { "summary": "Create user", "tags": ["Users"] }
            },
            "/posts": {
                "get": { "summary": "List posts", "tags": ["Posts"] }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    assert_eq!(collections.len(), 2);
    let users = collections.iter().find(|c| c.name == "Users").unwrap();
    let posts = collections.iter().find(|c| c.name == "Posts").unwrap();
    assert_eq!(users.requests.len(), 2);
    assert_eq!(posts.requests.len(), 1);
}

#[test]
fn import_openapi3_untagged_goes_to_default() {
    let spec = r#"{
        "openapi": "3.0.0",
        "info": { "title": "Test", "version": "1.0" },
        "paths": {
            "/health": {
                "get": { "summary": "Health check" }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    assert_eq!(collections[0].name, "Default");
}

#[test]
fn import_openapi3_with_query_params() {
    let spec = r#"{
        "openapi": "3.0.0",
        "info": { "title": "Test", "version": "1.0" },
        "paths": {
            "/search": {
                "get": {
                    "summary": "Search",
                    "parameters": [
                        { "name": "q", "in": "query", "required": true, "example": "hello" },
                        { "name": "limit", "in": "query", "schema": { "default": 10 } }
                    ]
                }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    let req = &collections[0].requests[0];
    assert_eq!(req.query_params.len(), 2);
    assert_eq!(req.query_params[0].key, "q");
    assert_eq!(req.query_params[0].value, "hello");
    assert!(req.query_params[0].is_enabled);
    assert_eq!(req.query_params[1].key, "limit");
    assert_eq!(req.query_params[1].value, "10");
    assert!(!req.query_params[1].is_enabled); // not required
}

#[test]
fn import_openapi3_with_json_body() {
    let spec = r#"{
        "openapi": "3.0.0",
        "info": { "title": "Test", "version": "1.0" },
        "paths": {
            "/users": {
                "post": {
                    "summary": "Create user",
                    "requestBody": {
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "example": { "name": "Alice", "email": "alice@example.com" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    match &collections[0].requests[0].body {
        RequestBody::Json(text) => {
            assert!(text.contains("Alice"));
        }
        _ => panic!("Expected JSON body"),
    }
}

#[test]
fn import_swagger2_with_host_and_basepath() {
    let spec = r#"{
        "swagger": "2.0",
        "info": { "title": "Test", "version": "1.0" },
        "host": "api.example.com",
        "basePath": "/v1",
        "schemes": ["https"],
        "paths": {
            "/users": {
                "get": { "summary": "List users", "tags": ["Users"] }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    assert_eq!(
        collections[0].requests[0].url,
        "https://api.example.com/v1/users"
    );
}

#[test]
fn import_swagger2_default_scheme() {
    let spec = r#"{
        "swagger": "2.0",
        "info": { "title": "Test", "version": "1.0" },
        "host": "api.example.com",
        "paths": {
            "/health": {
                "get": { "summary": "Health" }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    assert_eq!(
        collections[0].requests[0].url,
        "https://api.example.com/health"
    );
}

#[test]
fn import_yaml_openapi3() {
    let spec = r#"
openapi: "3.0.0"
info:
  title: YAML API
  version: "1.0"
servers:
  - url: https://yaml.example.com
paths:
  /items:
    get:
      summary: List items
      tags:
        - Items
"#;
    let collections = import_openapi(spec).unwrap();
    assert_eq!(collections[0].name, "Items");
    assert_eq!(
        collections[0].requests[0].url,
        "https://yaml.example.com/items"
    );
}

#[test]
fn import_openapi3_head_and_options() {
    let spec = r#"{
        "openapi": "3.0.0",
        "info": { "title": "Test", "version": "1.0" },
        "paths": {
            "/resource": {
                "head": { "summary": "Check resource" },
                "options": { "summary": "CORS preflight" }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    let methods: Vec<_> = collections[0]
        .requests
        .iter()
        .map(|r| r.method.clone())
        .collect();
    assert!(methods.contains(&HttpMethod::Head));
    assert!(methods.contains(&HttpMethod::Options));
}

#[test]
fn import_openapi3_uses_operation_id_as_fallback_name() {
    let spec = r#"{
        "openapi": "3.0.0",
        "info": { "title": "Test", "version": "1.0" },
        "paths": {
            "/items": {
                "get": { "operationId": "getItems" }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    assert_eq!(collections[0].requests[0].name, "getItems");
}

#[test]
fn import_missing_paths_returns_error() {
    let spec = r#"{ "openapi": "3.0.0", "info": { "title": "Empty" } }"#;
    let result = import_openapi(spec);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("paths"));
}

#[test]
fn import_openapi3_with_header_params() {
    let spec = r#"{
        "openapi": "3.0.0",
        "info": { "title": "Test", "version": "1.0" },
        "paths": {
            "/data": {
                "get": {
                    "summary": "Get data",
                    "parameters": [
                        { "name": "X-Api-Key", "in": "header", "required": true }
                    ]
                }
            }
        }
    }"#;
    let collections = import_openapi(spec).unwrap();
    let req = &collections[0].requests[0];
    assert_eq!(req.headers.len(), 1);
    assert_eq!(req.headers[0].key, "X-Api-Key");
    assert!(req.headers[0].is_enabled);
}

#[test]
fn import_invalid_input_returns_error() {
    let result = import_openapi("not valid json or yaml {{{");
    assert!(result.is_err());
}
