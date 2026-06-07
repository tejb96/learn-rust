use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Default)]
pub struct AppState {
    pub items: Arc<Mutex<HashMap<String, Item>>>,
}

const API_KEY: &str = "dev-key";

/// Builds the router with GET /health (no auth) and GET /items (requires X-API-Key header).
pub fn new_router() -> Router {
    Router::new()
}

async fn health() -> &'static str {
    "ok"
}

async fn list_items(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Item>>, StatusCode> {
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
