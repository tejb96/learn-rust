// DO NOT EDIT — implement the solution in src/lib.rs

use axum::body::Body;
use axum::http::{Request, StatusCode};
use rest_api::{new_router, AppState, Item};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

fn app_with_item() -> axum::Router {
    let mut items = HashMap::new();
    items.insert(
        "1".into(),
        Item {
            id: "1".into(),
            name: "widget".into(),
        },
    );
    let state = AppState {
        items: Arc::new(Mutex::new(items)),
    };
    new_router().with_state(state)
}

#[tokio::test]
async fn health_no_auth() {
    let app = new_router().with_state(AppState::default());
    let response = app
        .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn items_requires_api_key() {
    let app = app_with_item();
    let response = app
        .oneshot(Request::builder().uri("/items").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn items_with_valid_key() {
    let app = app_with_item();
    let response = app
        .oneshot(
            Request::builder()
                .uri("/items")
                .header("X-API-Key", "dev-key")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
