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
pub mod u2;

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
        // ══ Java x_organization_assemble_personal 契约补齐（u2）════════════
        // PersonAction
        // 注：GET/PUT /jaxrs/person/icon 已由 personal_extend 提供同路径实现，
        // 此处仅补齐 octet-stream 上传与 mock 别名，避免跨 crate 路由冲突。
        .route("/jaxrs/person/mockputtopost", post(edit_person))
        .route(
            "/jaxrs/person/icon",
            post(u2::set_icon_octet_stream),
        )
        .route(
            "/jaxrs/person/icon/mockputtopost",
            post(u2::upload_multipart_alias),
        )
        // PasswordAction
        .route("/jaxrs/person/password/mockputtopost", post(password::change))
        // RegistAction
        .route("/jaxrs/person/regist/mode", get(u2::regist_mode))
        .route(
            "/jaxrs/person/regist/captcha/width/{width}/height/{height}",
            get(auth::captcha::captcha_with_size),
        )
        .route(
            "/jaxrs/person/regist/code/mobile/{mobile}",
            get(u2::regist_code_mobile),
        )
        .route(
            "/jaxrs/person/regist/check/password/{password}",
            get(u2::regist_check_password),
        )
        // ResetAction
        .route("/jaxrs/reset/mockputtopost", post(reset::reset_password))
        // SignatureAction
        .route(
            "/jaxrs/person/signature/list/person/{flag}",
            get(u2::signature_list_person),
        )
        // CustomAction
        .route(
            "/jaxrs/person/custom/{name}",
            get(u2::custom_get)
                .put(u2::custom_edit)
                .post(u2::custom_edit)
                .delete(u2::custom_delete),
        )
        .route("/jaxrs/person/custom/{name}/mockdeletetoget", get(u2::custom_delete))
        .route(
            "/jaxrs/person/custom/manager/person/{person}/name/{name}",
            get(u2::custom_manager_get).put(u2::custom_manager_edit),
        )
        .route(
            "/jaxrs/person/custom/manager/person/{person}/name/{name}/mockputtopost",
            post(u2::custom_manager_edit),
        )
        // DefinitionAction
        .route(
            "/jaxrs/person/definition/{name}",
            get(u2::definition_get)
                .put(u2::definition_edit)
                .post(u2::definition_edit)
                .delete(u2::definition_delete),
        )
        .route("/jaxrs/person/definition/{name}/mockdeletetoget", get(u2::definition_delete))
        .route("/jaxrs/person/definition/{name}/mockputtopost", post(u2::definition_edit))
        // EmpowerAction 残余
        .route(
            "/jaxrs/person/empower/list/{id}/next/{count}",
            get(u2::empower_list_next),
        )
        .route(
            "/jaxrs/person/empower/list/{id}/prev/{count}",
            get(u2::empower_list_prev),
        )
        .route(
            "/jaxrs/person/empower/list/person/{flag}",
            get(u2::empower_list_with_person),
        )
        .route(
            "/jaxrs/person/empower/{id}/mockputtopost",
            post(empower::update),
        )
        .route(
            "/jaxrs/person/empower/manager/{id}/mockputtopost",
            post(empower::manager_update),
        )
        .route(
            "/jaxrs/person/empower/{id}/mockdeletetoget",
            get(empower::delete),
        )
        .route(
            "/jaxrs/person/empower/manager/{id}/mockdeletetoget",
            get(empower::manager_delete),
        )
        // EmpowerLogAction
        .route(
            "/jaxrs/person/empowerlog/list/{id}/next/{count}",
            get(u2::log_list_next),
        )
        .route(
            "/jaxrs/person/empowerlog/list/{id}/prev/{count}",
            get(u2::log_list_prev),
        )
        .route(
            "/jaxrs/person/empowerlog/list/currentperson/paging/{page}/size/{size}",
            post(u2::log_currentperson_paging),
        )
        .route(
            "/jaxrs/person/empowerlog/list/to/currentperson/paging/{page}/size/{size}",
            post(u2::log_to_currentperson_paging),
        )
        .route(
            "/jaxrs/person/empowerlog/manager/list/paging/{page}/size/{size}",
            post(u2::log_manager_paging),
        )
        .route(
            "/jaxrs/person/empowerlog/{id}",
            axum::routing::delete(u2::log_delete),
        )
        .route(
            "/jaxrs/person/empowerlog/{id}/mockdeletetoget",
            get(u2::log_delete),
        )
        // ExmailAction
        .route("/jaxrs/person/exmail/new/count", get(u2::exmail_new_count))
        .route(
            "/jaxrs/person/exmail/new/count/passive",
            get(u2::exmail_new_count_passive),
        )
        .route(
            "/jaxrs/person/exmail/list/title/passive",
            get(u2::exmail_list_title_passive),
        )
        .route("/jaxrs/person/exmail/sso", get(u2::exmail_sso))
        .route(
            "/jaxrs/person/exmail",
            get(u2::exmail_callback_get).post(u2::exmail_callback_post),
        )
        .layer(Extension(pool))
        .layer(Extension(session_manager))
        .layer(Extension(reset_store))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

