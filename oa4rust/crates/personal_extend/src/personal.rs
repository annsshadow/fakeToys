use axum::{
    extract::Extension,
    http::HeaderMap,
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use shared::error::AppError;
use shared::response::ActionResult;

use auth::SessionManager;
use personal::PersonInfo;

// 更新个人信息请求 DTO
#[derive(Debug, Deserialize)]
pub struct UpdatePersonalRequest {
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

// 获取当前登录用户信息
//
// 从 Authorization header 中提取 token，验证会话后查询 auth_person 表，
// 返回当前用户的 id、唯一标识、姓名、手机号、邮箱、头像等基本信息。
//
// # 参数
// - `pool`: 数据库连接池
// - `session_manager`: 会话管理器，用于验证当前用户身份
// - `headers`: 请求头，从中提取 Bearer token
pub async fn get_info(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let token = shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let info = PersonInfo {
        id: row.get("id"),
        unique: row.get("unique_id"),
        name: row.get("name"),
        mobile: row.get("mobile"),
        email: row.get("email"),
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(info)))
}

// 更新当前登录用户的个人信息
//
// 支持部分更新 name、mobile、email 字段，未提供的字段保留原值。
// 更新后同时刷新 updated_at 时间戳。
//
// # 参数
// - `pool`: 数据库连接池
// - `session_manager`: 会话管理器，用于验证当前用户身份
// - `headers`: 请求头，从中提取 Bearer token
// - `req`: 更新请求体，包含可选的 name、mobile、email 字段
pub async fn update_info(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    axum::extract::Json(req): axum::extract::Json<UpdatePersonalRequest>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let token = shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 先查询当前用户信息，用于保留未更新的字段
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");
    let current_name: String = row.get("name");
    let current_mobile: Option<String> = row.get("mobile");
    let current_email: Option<String> = row.get("email");
    let icon: Option<String> = row.get("icon");

    let name = req.name.unwrap_or(current_name);
    let mobile = req.mobile.or(current_mobile);
    let email = req.email.or(current_email);

    client
        .execute(
            "UPDATE auth_person SET name = $1, mobile = $2, email = $3, updated_at = NOW() WHERE id = $4",
            &[&name, &mobile, &email, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let updated = PersonInfo {
        id: person_id,
        unique: session.person_unique,
        name,
        mobile,
        email,
        icon,
    };

    Ok(Json(ActionResult::success(updated)))
}

// 获取指定用户的信息（需登录）
//
// 查询 auth_person 表中指定 unique_id 的用户信息。
// 需要当前用户已登录，且目标用户未被锁定。
//
// # 参数
// - `pool`: 数据库连接池
// - `session_manager`: 会话管理器，用于验证当前用户身份
// - `headers`: 请求头，从中提取 Bearer token
// - `id`: 路径参数，目标用户的唯一标识
pub async fn get_detail(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    // 验证当前请求者已登录
    let token = shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let _session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let info = PersonInfo {
        id: row.get("id"),
        unique: row.get("unique_id"),
        name: row.get("name"),
        mobile: row.get("mobile"),
        email: row.get("email"),
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(info)))
}