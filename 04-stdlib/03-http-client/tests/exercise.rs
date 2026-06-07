// DO NOT EDIT — implement the solution in src/lib.rs

use http_client::{is_success, parse_response, parse_simple_json, HttpResponse};

#[test]
fn parse_response_ok() {
    let raw = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nhello";
    let resp = parse_response(raw).unwrap();
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, "hello");
}

#[test]
fn parse_response_invalid() {
    assert!(parse_response("not http").is_err());
}

#[test]
fn is_success_range() {
    let ok = HttpResponse {
        status: 201,
        body: String::new(),
    };
    assert!(is_success(&ok));
    let fail = HttpResponse {
        status: 404,
        body: String::new(),
    };
    assert!(!is_success(&fail));
}

#[test]
fn parse_simple_json_pairs() {
    let body = r#"{"name":"Ada","lang":"Rust"}"#;
    let pairs = parse_simple_json(body);
    assert!(pairs.contains(&("name".into(), "Ada".into())));
    assert!(pairs.contains(&("lang".into(), "Rust".into())));
}
