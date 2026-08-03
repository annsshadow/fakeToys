use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SendCodeRequest {
    pub credential: String,
}

#[derive(Debug, Deserialize)]
pub struct CheckCodeRequest {
    pub credential: String,
    pub code: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub credential: String,
    pub code: String,
    pub password: String,
}

#[derive(Clone)]
pub struct ResetCodeStore {
    codes: Arc<RwLock<std::collections::HashMap<String, (String, chrono::DateTime<chrono::Utc>)>>>,
}

impl Default for ResetCodeStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ResetCodeStore {
    pub fn new() -> Self {
        Self {
            codes: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn store(&self, key: String, code: String) {
        let mut codes = self.codes.write().await;
        codes.insert(key, (code, chrono::Utc::now() + chrono::Duration::minutes(10)));
    }

    pub async fn verify(&self, key: &str, code: &str) -> bool {
        let codes = self.codes.read().await;
        if let Some((stored_code, _)) = codes.get(key) {
            return stored_code == code;
        }
        false
    }

    pub async fn remove(&self, key: &str) {
        self.codes.write().await.remove(key);
    }
}

pub async fn send_code(
    pool: Extension<Pool>,
    store: Extension<ResetCodeStore>,
    axum::extract::Json(req): axum::extract::Json<SendCodeRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id FROM auth_person WHERE unique_id = $1 AND locked = false",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");
    let code = Uuid::new_v4().to_string();
    store.store(person_id, code).await;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("sent".to_string(), Value::Bool(true)),
    ])))))
}

pub async fn check_code(
    store: Extension<ResetCodeStore>,
    axum::extract::Json(req): axum::extract::Json<CheckCodeRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let valid = store.verify(&req.credential, &req.code).await;
    if valid {
        Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("valid".to_string(), Value::Bool(true)),
        ])))))
    } else {
        Ok(Json(ActionResult::error("invalid code")))
    }
}

pub async fn reset_password(
    pool: Extension<Pool>,
    store: Extension<ResetCodeStore>,
    axum::extract::Json(req): axum::extract::Json<ResetPasswordRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let valid = store.verify(&req.credential, &req.code).await;
    if !valid {
        return Ok(Json(ActionResult::error("invalid or expired code")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id FROM auth_person WHERE unique_id = $1 AND locked = false",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");
    let new_hash = format!("{:x}", md5::compute(req.password.as_bytes()));

    client
        .execute(
            "UPDATE auth_person SET password_hash = $1, updated_at = NOW() WHERE id = $2",
            &[&new_hash, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    store.remove(&req.credential).await;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("success".to_string(), Value::Bool(true)),
    ])))))
}
