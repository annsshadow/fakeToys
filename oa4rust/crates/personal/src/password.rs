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

use auth::password::{hash_password, verify_password};
use auth::SessionManager;

use crate::resolve_current_person_unique;

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

/// 修改当前登录用户密码（PUT /jaxrs/person/password，Java PasswordAction 契约）
///
/// 按会话唯一标识解析当前用户，校验旧密码（verify_password 支持 bcrypt 前缀
/// 及既有 MD5/DES 旧哈希），新密码使用 hash_password 写入（统一 bcrypt 前缀）。
pub async fn change(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    axum::extract::Json(req): axum::extract::Json<ChangePasswordRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, password_hash FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&person_unique],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");
    let stored_hash: String = row.get("password_hash");

    let valid = verify_password(&req.old_password, &stored_hash, "", None);
    if !valid {
        return Ok(Json(ActionResult::error("old password mismatch")));
    }

    let new_hash = hash_password(&req.new_password);
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
