use axum::{
    extract::Extension,
    http::HeaderMap,
    routing::{get, post, put},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use shared::error::AppError;
use shared::response::ActionResult;

use auth::SessionManager;

pub mod icon;
pub mod password;
pub mod regist;
pub mod reset;
pub mod signature;

// --- 数据模型 ---

#[derive(Debug, Deserialize)]
pub struct EditPersonRequest {
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PersonInfo {
    pub id: String,
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub icon: Option<String>,
}

// --- 当前用户身份解析 ---

/// 从 `Authorization: Bearer <token>` 解析当前登录用户唯一标识
///
/// 复用认证流程的会话机制（与 personal_extend 一致）：token 由登录接口写入
/// SessionManager（main.rs 构造单一实例注入），此处按会话唯一标识定位人员，
/// 禁止按 `WHERE locked = false LIMIT 1` 取首行，避免任意登录用户篡改他人数据。
pub(crate) async fn resolve_current_person_unique(
    session_manager: &SessionManager,
    headers: &HeaderMap,
) -> Result<String, AppError> {
    let token = shared::middleware::extract_token_from_headers(headers).ok_or(AppError::Unauthorized)?;
    session_manager
        .validate_session(&token)
        .await
        .map(|session| session.person_unique)
        .ok_or(AppError::Unauthorized)
}

// --- 处理器 ---

/// 查询当前登录用户个人信息
///
/// 从会话解析当前用户，按 unique_id 从 auth_person 表读取
/// 姓名、手机号、邮箱、头像等基本信息。
pub async fn get_person(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&person_unique],
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

/// 更新当前登录用户个人信息
///
/// 支持部分更新（name、mobile、email 均为可选字段），未提供的字段保留原值。
/// 更新后同时刷新 updated_at 时间戳。
pub async fn edit_person(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    axum::extract::Json(req): axum::extract::Json<EditPersonRequest>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&person_unique],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");

    let name = req.name.filter(|s| !s.is_empty()).unwrap_or_else(|| row.get("name"));
    let db_mobile: Option<String> = row.get("mobile");
    let db_email: Option<String> = row.get("email");
    let mobile = req.mobile.filter(|s| !s.is_empty()).or(db_mobile);
    let email = req.email.filter(|s| !s.is_empty()).or(db_email);

    client
        .execute(
            "UPDATE auth_person SET name = $1, mobile = $2, email = $3, updated_at = NOW() WHERE id = $4",
            &[&name, &mobile, &email, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let updated = PersonInfo {
        id: person_id,
        unique: row.get("unique_id"),
        name,
        mobile,
        email,
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(updated)))
}

// --- 路由注册 ---

/// 构建个人中心模块路由
///
/// 注册个人信息查询/更新、密码修改（Java PasswordAction 契约）、
/// 密码重置（Java ResetAction 契约）等接口。
///
/// 使用 main.rs 注入的共享 SessionManager，确保与 auth crate 登录会话互认。
pub fn router(pool: Pool, session_manager: SessionManager) -> Router {
    let reset_store = reset::ResetCodeStore::new();

    Router::new()
        .route("/jaxrs/person", get(get_person))
        .route("/jaxrs/person", put(edit_person))
        .route("/jaxrs/person/password", put(password::change))
        .route(
            "/jaxrs/reset/check/credential/{credential}",
            get(reset::check_credential),
        )
        .route(
            "/jaxrs/reset/check/password/{password}",
            get(reset::check_password),
        )
        .route(
            "/jaxrs/reset/code/credential/{credential}",
            get(reset::send_code),
        )
        .route("/jaxrs/reset", put(reset::reset_password))
        .route(
            "/jaxrs/reset/password/anonymous",
            post(reset::reset_password_anonymous),
        )
        // 注册端点（Public，无需认证）
        .route("/jaxrs/person/regist", post(regist::register))
        .route(
            "/jaxrs/person/regist/check/name/{name}",
            get(regist::check_name),
        )
        .route(
            "/jaxrs/person/regist/check/mobile/{mobile}",
            get(regist::check_mobile),
        )
        .route(
            "/jaxrs/person/regist/check/email/{email}",
            get(regist::check_email),
        )
        .route("/jaxrs/person/regist/code", post(regist::send_regist_code))
        // 电子签名端点
        .route("/jaxrs/person/signature/upload", post(signature::upload))
        .route("/jaxrs/person/signature/list", get(signature::list))
        .route("/jaxrs/person/signature/delete/{id}", get(signature::delete))
        .route("/jaxrs/person/signature/manager/list", get(signature::manager_list))
        // 头像端点
        .route("/jaxrs/person/icon/{person}", get(icon::get))
        .route("/jaxrs/person/icon/upload", post(icon::upload))
        .layer(Extension(pool))
        .layer(Extension(session_manager))
        .layer(Extension(reset_store))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

