use super::*;

fn minimal_har(entries_json: &str) -> String {
    format!(r#"{{"log":{{"version":"1.2","entries":[{entries_json}]}}}}"#)
}

fn entry(method: &str, url: &str, extra: &str) -> String {
    format!(
        r#"{{"request":{{"method":"{method}","url":"{url}","headers":[],"queryString":[]{extra}}}}}"#
    )
}

#[test]
fn import_simple_get() {
    let har = minimal_har(&entry("GET", "https://api.example.com/users", ""));
    let collection = import_har(&har).unwrap();
    assert_eq!(collection.requests.len(), 1);
    let req = &collection.requests[0];
    assert_eq!(req.method, HttpMethod::Get);
    assert_eq!(req.url, "https://api.example.com/users");
    assert!(req.name.contains("/users"));
}

#[test]
fn import_multiple_entries() {
    let entries = format!(
        "{},{}",
        entry("GET", "https://example.com/a", ""),
        entry("POST", "https://example.com/b", "")
    );
    let har = minimal_har(&entries);
    let collection = import_har(&har).unwrap();
    assert_eq!(collection.requests.len(), 2);
    assert_eq!(collection.requests[0].method, HttpMethod::Get);
    assert_eq!(collection.requests[1].method, HttpMethod::Post);
    assert_eq!(collection.requests[0].sort_order, 0);
    assert_eq!(collection.requests[1].sort_order, 1);
}

#[test]
fn import_with_query_params() {
    let har = minimal_har(&format!(
        r#"{{"request":{{"method":"GET","url":"https://example.com/search?q=hello&page=1","headers":[],"queryString":[{{"name":"q","value":"hello"}},{{"name":"page","value":"1"}}]}}}}"#
    ));
    let collection = import_har(&har).unwrap();
    let req = &collection.requests[0];
    assert_eq!(req.url, "https://example.com/search");
    assert_eq!(req.query_params.len(), 2);
    assert_eq!(req.query_params[0].key, "q");
    assert_eq!(req.query_params[0].value, "hello");
    assert_eq!(req.query_params[1].key, "page");
}

#[test]
fn import_with_headers() {
    let har = minimal_har(&format!(
        r#"{{"request":{{"method":"GET","url":"https://example.com/","headers":[{{"name":"Authorization","value":"Bearer tok123"}},{{"name":"Accept","value":"application/json"}}],"queryString":[]}}}}"#
    ));
    let collection = import_har(&har).unwrap();
    let req = &collection.requests[0];
    assert_eq!(req.headers.len(), 2);
    assert_eq!(req.headers[0].key, "Authorization");
    assert_eq!(req.headers[1].key, "Accept");
}

#[test]
fn import_skips_pseudo_headers_and_cookies() {
    let har = minimal_har(&format!(
        r#"{{"request":{{"method":"GET","url":"https://example.com/","headers":[{{"name":":authority","value":"example.com"}},{{"name":"Cookie","value":"session=abc"}},{{"name":"Accept","value":"*/*"}}],"queryString":[]}}}}"#
    ));
    let collection = import_har(&har).unwrap();
    let req = &collection.requests[0];
    assert_eq!(req.headers.len(), 1);
    assert_eq!(req.headers[0].key, "Accept");
}

#[test]
fn import_json_body() {
    let har = minimal_har(&format!(
        r#"{{"request":{{"method":"POST","url":"https://example.com/api","headers":[],"queryString":[],"postData":{{"mimeType":"application/json","text":"{{\"name\":\"test\"}}"}}}}}}"#
    ));
    let collection = import_har(&har).unwrap();
    let req = &collection.requests[0];
    match &req.body {
        RequestBody::Json(s) => assert!(s.contains("\"name\""), "body: {s}"),
        other => panic!("Expected Json body, got: {other:?}"),
    }
}

#[test]
fn import_form_data_body() {
    let har = minimal_har(&format!(
        r#"{{"request":{{"method":"POST","url":"https://example.com/form","headers":[],"queryString":[],"postData":{{"mimeType":"application/x-www-form-urlencoded","params":[{{"name":"user","value":"alice"}},{{"name":"pass","value":"secret"}}]}}}}}}"#
    ));
    let collection = import_har(&har).unwrap();
    let req = &collection.requests[0];
    match &req.body {
        RequestBody::FormData(pairs) => {
            assert_eq!(pairs.len(), 2);
            assert_eq!(pairs[0].key, "user");
            assert_eq!(pairs[1].value, "secret");
        }
        other => panic!("Expected FormData body, got: {other:?}"),
    }
}

#[test]
fn import_raw_text_body() {
    let har = minimal_har(&format!(
        r#"{{"request":{{"method":"POST","url":"https://example.com/xml","headers":[],"queryString":[],"postData":{{"mimeType":"application/xml","text":"<root/>"}}}}}}"#
    ));
    let collection = import_har(&har).unwrap();
    let req = &collection.requests[0];
    match &req.body {
        RequestBody::RawText(s) => assert_eq!(s, "<root/>"),
        other => panic!("Expected RawText body, got: {other:?}"),
    }
}

#[test]
fn import_uses_page_title_as_collection_name() {
    let har = format!(
        r#"{{"log":{{"version":"1.2","pages":[{{"title":"My API Session"}}],"entries":[{entry}]}}}}"#,
        entry = entry("GET", "https://example.com/", "")
    );
    let collection = import_har(&har).unwrap();
    assert_eq!(collection.name, "My API Session");
}

#[test]
fn import_empty_entries_returns_error() {
    let har = r#"{"log":{"version":"1.2","entries":[]}}"#;
    let result = import_har(har);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("no entries"));
}

#[test]
fn import_invalid_json_returns_error() {
    let result = import_har("not json");
    assert!(result.is_err());
}

#[test]
fn import_head_and_options_methods() {
    let entries = format!(
        "{},{}",
        entry("HEAD", "https://example.com/health", ""),
        entry("OPTIONS", "https://example.com/cors", "")
    );
    let har = minimal_har(&entries);
    let collection = import_har(&har).unwrap();
    assert_eq!(collection.requests[0].method, HttpMethod::Head);
    assert_eq!(collection.requests[1].method, HttpMethod::Options);
}
