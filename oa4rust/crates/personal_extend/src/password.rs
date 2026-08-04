use axum::{
    extract::Extension,
    http::HeaderMap,
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

use auth::{password::verify_password, SessionManager};

// 修改密码请求 DTO
#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// 重置密码请求 DTO
#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub credential: String,
    pub code: String,
    pub password: String,
}

// 验证码验证请求 DTO
#[derive(Debug, Deserialize)]
pub struct VerifyPasswordRequest {
    pub credential: String,
    pub password: String,
}

// 验证码存储（用于重置密码流程）
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

    /// 存储验证码，有效期10分钟
    pub async fn store(&self, key: String, code: String) {
        let mut codes = self.codes.write().await;
        codes.insert(key, (code, chrono::Utc::now() + chrono::Duration::minutes(10)));
    }

    /// 验证验证码是否正确
    pub async fn verify(&self, key: &str, code: &str) -> bool {
        let codes = self.codes.read().await;
        if let Some((stored_code, _)) = codes.get(key) {
            return stored_code == code;
        }
        false
    }

    /// 删除已使用的验证码
    pub async fn remove(&self, key: &str) {
        self.codes.write().await.remove(key);
    }
}

// 修改密码
//
// 验证旧密码后更新为新密码（MD5 哈希存储）。
// 需要当前用户已登录并传入 Authorization header。
//
// # 参数
// - `pool`: 数据库连接池
// - `session_manager`: 会话管理器
// - `req`: 包含 old_password 和 new_password 的请求体
pub async fn change(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: Extension<HeaderMap>,
    axum::extract::Json(req): axum::extract::Json<ChangePasswordRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_bearer_token(&headers)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, password_hash FROM auth_person WHERE unique_id = $1 AND locked = false",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");
    let stored_hash: String = row.get("password_hash");

    if !verify_password(&req.old_password, &stored_hash, "", None) {
        return Ok(Json(ActionResult::error("旧密码不正确")));
    }

    let new_hash = format!("{:x}", md5::compute(req.new_password.as_bytes()));
    client
        .execute(
            "UPDATE auth_person SET password_hash = $1, updated_at = NOW() WHERE id = $2",
            &[&new_hash, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("success".to_string(), Value::Bool(true)),
    ])))))
}

// 重置密码（发送验证码）
//
// 根据用户 unique_id 查询 auth_person，验证用户存在后生成随机验证码并存储。
//
// # 参数
// - `pool`: 数据库连接池
// - `store`: 验证码存储
// - `req`: 包含 credential（用户唯一标识）的请求体
pub async fn reset(
    pool: Extension<Pool>,
    store: Extension<ResetCodeStore>,
    axum::extract::Json(req): axum::extract::Json<ResetPasswordRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 验证用户存在且未锁定
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

// 验证密码
//
// 用于验证指定用户是否拥有正确的旧密码（不修改）。
// 通常用于敏感操作前的二次确认。
//
// # 参数
// - `pool`: 数据库连接池
// - `req`: 包含 credential 和 password 的请求体
pub async fn verify(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<VerifyPasswordRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, password_hash FROM auth_person WHERE unique_id = $1 AND locked = false",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let stored_hash: String = row.get("password_hash");

    if verify_password(&req.password, &stored_hash, "", None) {
        Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("valid".to_string(), Value::Bool(true)),
        ])))))
    } else {
        Ok(Json(ActionResult::error("密码不正确")))
    }
}

/// 从 Authorization header 中提取 Bearer token
pub(crate) fn extract_bearer_token(headers: &HeaderMap) -> Result<String, AppError> {
    let auth = headers
        .get(axum::http::header::AUTHORIZATION)
        .ok_or(AppError::Unauthorized)?
        .to_str()
        .map_err(|_| AppError::Unauthorized)?;

    let prefix = "Bearer ";
    if !auth.starts_with(prefix) {
        return Err(AppError::Unauthorized);
    }

    Ok(auth[prefix.len()..].to_string())
}
