use axum::{Json, Router, routing::get, routing::post};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;

use shared::{error::AppError, response::ActionResult};

#[derive(Clone)]
pub struct SecretState {
    pub secret: Arc<RwLock<Option<String>>>,
}

impl Default for SecretState {
    fn default() -> Self {
        Self::new()
    }
}

impl SecretState {
    pub fn new() -> Self {
        Self {
            secret: Arc::new(RwLock::new(None)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SetSecretRequest {
    pub secret: String,
}

pub async fn check() -> Result<Json<ActionResult<Value>>, AppError> {
    let state = SecretState::new();
    let secret = state.secret.read().await;
    let initialized = secret.is_some();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("initialized".to_string(), Value::Bool(initialized)),
    ])))))
}

pub async fn set(
    axum::extract::Json(req): Json<SetSecretRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.secret.is_empty() {
        return Ok(Json(ActionResult::error("secret cannot be empty")));
    }

    let state = SecretState::new();
    let mut secret = state.secret.write().await;
    *secret = Some(req.secret.clone());

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("set".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn set_cancel() -> Result<Json<ActionResult<Value>>, AppError> {
    let state = SecretState::new();
    let mut secret = state.secret.write().await;
    *secret = None;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("canceled".to_string(), Value::Bool(true)),
    ])))))
}

pub fn program_init_router() -> Router {
    Router::new()
        .route("/jaxrs/secret/check", get(check))
        .route("/jaxrs/secret/set", post(set))
        .route("/jaxrs/secret/set/cancel", get(set_cancel))
}
