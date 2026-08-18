use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

use crate::reset::{is_password_acceptable, ResetCodeStore};

// ──────────────────────────────────────────────────────────────────────────────
// regist — 用户注册
//
// 端点：
//   POST /jaxrs/person/regist              — 注册新用户
//   GET  /jaxrs/person/regist/check/name/{name}       — 检查用户名唯一性
//   GET  /jaxrs/person/regist/check/mobile/{mobile}   — 检查手机号唯一性
//   GET  /jaxrs/person/regist/check/email/{email}     — 检查邮箱唯一性
//   POST /jaxrs/person/regist/code            — 发送注册验证码
//
// 权限：Public（无需认证）
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub credential: String,
    pub password: String,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub code: String,
}

/// POST /jaxrs/person/regist — 注册新用户
pub async fn register(
    pool: Extension<Pool>,
    reset_store: Extension<ResetCodeStore>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    // 参数校验
    if req.credential.is_empty() || req.password.is_empty() || req.name.is_empty() {
        return Ok(Json(ActionResult::error("credential, password, and name are required")));
    }

    // 密码强度校验（复用 reset.rs 中的规则：6-64 位，含字母和数字）
    if !is_password_acceptable(&req.password) {
        return Ok(Json(ActionResult::error(
            "password must be 6-64 characters and contain at least one letter and one digit",
        )));
    }

    // 验证码校验
    if let Err(e) = reset_store.verify_and_consume(&req.credential, &req.code).await {
        return Ok(Json(ActionResult::error(e.to_string())));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 应用层预检查：用户名唯一性
    let name_exists = client
        .query_opt(
            "SELECT 1 FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if name_exists.is_some() {
        return Ok(Json(ActionResult::error("username already exists")));
    }

    // 应用层预检查：手机号唯一性
    if let Some(ref mobile) = req.mobile {
        let mobile_exists = client
            .query_opt(
                "SELECT 1 FROM auth_person WHERE mobile = $1 AND deleted_at IS NULL",
                &[mobile],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if mobile_exists.is_some() {
            return Ok(Json(ActionResult::error("mobile already exists")));
        }
    }

    // 应用层预检查：邮箱唯一性
    if let Some(ref email) = req.email {
        let email_exists = client
            .query_opt(
                "SELECT 1 FROM auth_person WHERE email = $1 AND deleted_at IS NULL",
                &[email],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        if email_exists.is_some() {
            return Ok(Json(ActionResult::error("email already exists")));
        }
    }

    // 创建用户
    let id = uuid::Uuid::new_v4().to_string();
    let password_hash = auth::password::hash_password(&req.password);

    client
        .execute(
            "INSERT INTO auth_person (id, unique_id, name, mobile, email, password_hash, locked, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6, false, NOW())",
            &[&id, &req.credential, &req.name, &req.mobile, &req.email, &password_hash],
        )
        .await
        .map_err(|e| {
            let msg = e.to_string();
            if msg.contains("unique") || msg.contains("duplicate") {
                if msg.contains("unique_id") {
                    AppError::BadRequest("username already exists".to_string())
                } else if msg.contains("mobile") {
                    AppError::BadRequest("mobile already exists".to_string())
                } else if msg.contains("email") {
                    AppError::BadRequest("email already exists".to_string())
                } else {
                    AppError::BadRequest("registration failed: duplicate entry".to_string())
                }
            } else {
                AppError::Internal
            }
        })?;

    Ok(Json(ActionResult::success(serde_json::json!({
        "id": id,
        "unique": req.credential,
        "name": req.name,
        "mobile": req.mobile,
        "email": req.email,
    }))))
}

/// GET /jaxrs/person/regist/check/name/{name}
pub async fn check_name(
    pool: Extension<Pool>,
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let exists = client
        .query_opt(
            "SELECT id FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&name],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "available": exists.is_none(),
    }))))
}

/// GET /jaxrs/person/regist/check/mobile/{mobile}
pub async fn check_mobile(
    pool: Extension<Pool>,
    Path(mobile): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let exists = client
        .query_opt(
            "SELECT id FROM auth_person WHERE mobile = $1 AND deleted_at IS NULL",
            &[&mobile],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "available": exists.is_none(),
    }))))
}

/// GET /jaxrs/person/regist/check/email/{email}
pub async fn check_email(
    pool: Extension<Pool>,
    Path(email): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let exists = client
        .query_opt(
            "SELECT id FROM auth_person WHERE email = $1 AND deleted_at IS NULL",
            &[&email],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    Ok(Json(ActionResult::success(serde_json::json!({
        "available": exists.is_none(),
    }))))
}

/// POST /jaxrs/person/regist/code — 发送注册验证码
pub async fn send_regist_code(
    reset_store: Extension<ResetCodeStore>,
    Json(req): Json<serde_json::Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let credential = req.get("credential")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("credential is required".to_string()))?;

    reset_store.issue(credential).await;
    // TODO: 发送实际短信（当前仅存储验证码）
    Ok(Json(ActionResult::success(serde_json::json!({
        "message": "code sent",
    }))))
}
