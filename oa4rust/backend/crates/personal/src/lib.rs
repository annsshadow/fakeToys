// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

use axum::{
    extract::Extension,
    http::HeaderMap,
    routing::{delete, get, post, put},
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
    let token =
        shared::middleware::extract_token_from_headers(headers).ok_or(AppError::Unauthorized)?;
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

    let name = req
        .name
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| row.get("name"));
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
/// 注册个人信息查询/更新、密码修改（o2server PasswordAction 契约）、
/// 密码重置（o2server ResetAction 契约）等接口。
///
/// 使用 main.rs 注入的共享 SessionManager，确保与 auth crate 登录会话互认。
pub fn router(pool: Pool, session_manager: SessionManager) -> Router {
    let reset_store = reset::ResetCodeStore::new();

    Router::new()
        .route("/api/person", get(get_person))
        .route("/api/person", put(edit_person))
        .route("/api/person/password", put(password::change))
        .route(
            "/api/reset/check/credential/{credential}",
            get(reset::check_credential),
        )
        .route(
            // 十类功能1 安全重设计：口令改 POST body（原 GET /{password} 口令入 URL）
            "/api/reset/check/password",
            post(reset::check_password),
        )
        .route(
            "/api/reset/code/credential/{credential}",
            get(reset::send_code),
        )
        .route("/api/reset", put(reset::reset_password))
        .route(
            "/api/reset/password/anonymous",
            post(reset::reset_password_anonymous),
        )
        // 注册端点（Public，无需认证）
        .route("/api/person/regist", post(regist::register))
        .route(
            "/api/person/regist/check/name/{name}",
            get(regist::check_name),
        )
        .route(
            "/api/person/regist/check/mobile/{mobile}",
            get(regist::check_mobile),
        )
        .route(
            "/api/person/regist/check/email/{email}",
            get(regist::check_email),
        )
        .route("/api/person/regist/code", post(regist::send_regist_code))
        // 电子签名端点
        .route("/api/person/signature/upload", post(signature::upload))
        .route("/api/person/signature/list", get(signature::list))
        .route("/api/person/signature/delete/{id}", get(signature::delete))
        .route(
            "/api/person/signature/manager/list",
            get(signature::manager_list),
        )
        // 头像端点
        .route("/api/person/icon/{person}", get(icon::get))
        .route("/api/person/icon/upload", post(icon::upload))
        // ══ o2server x_organization_assemble_personal 契约补齐（u2）════════════
        // PersonAction
        // 注：GET/PUT /api/person/icon 已由 personal_extend 提供同路径实现，
        // 此处仅补齐 octet-stream 上传与 mock 别名，避免跨 crate 路由冲突。
        .route("/api/person/mockputtopost", post(edit_person))
        .route("/api/person/icon", post(u2::set_icon_octet_stream))
        .route(
            "/api/person/icon/mockputtopost",
            post(u2::upload_multipart_alias),
        )
        // PasswordAction
        .route("/api/person/password/mockputtopost", post(password::change))
        // RegistAction
        .route("/api/person/regist/mode", get(u2::regist_mode))
        .route(
            "/api/person/regist/captcha/width/{width}/height/{height}",
            get(auth::captcha::captcha_with_size),
        )
        .route(
            "/api/person/regist/code/mobile/{mobile}",
            get(u2::regist_code_mobile),
        )
        .route(
            // 十类功能1 安全重设计：口令改 POST body（原 GET /{password} 口令入 URL）
            "/api/person/regist/check/password",
            post(u2::regist_check_password),
        )
        // ResetAction
        // round4-b76：mock 别名收会话门（wrapper），PUT /api/reset 匿名流不受影响
        .route(
            "/api/reset/mockputtopost",
            post(reset::reset_password_mockputtopost),
        )
        // SignatureAction
        .route(
            "/api/person/signature/list/person/{flag}",
            get(u2::signature_list_person),
        )
        // CustomAction
        .route("/api/person/custom/{name}", get(u2::custom_get))
        .route("/api/person/custom/{name}", put(u2::custom_edit))
        .route("/api/person/custom/{name}", post(u2::custom_edit))
        .route("/api/person/custom/{name}", delete(u2::custom_delete))
        .route(
            "/api/person/custom/{name}/mockdeletetoget",
            get(u2::custom_delete),
        )
        .route(
            "/api/person/custom/manager/person/{person}/name/{name}",
            get(u2::custom_manager_get),
        )
        .route(
            "/api/person/custom/manager/person/{person}/name/{name}",
            put(u2::custom_manager_edit),
        )
        .route(
            "/api/person/custom/manager/person/{person}/name/{name}/mockputtopost",
            post(u2::custom_manager_edit),
        )
        // DefinitionAction
        .route("/api/person/definition/{name}", get(u2::definition_get))
        .route("/api/person/definition/{name}", put(u2::definition_edit))
        .route("/api/person/definition/{name}", post(u2::definition_edit))
        .route(
            "/api/person/definition/{name}",
            delete(u2::definition_delete),
        )
        .route(
            "/api/person/definition/{name}/mockdeletetoget",
            get(u2::definition_delete),
        )
        .route(
            "/api/person/definition/{name}/mockputtopost",
            post(u2::definition_edit),
        )
        // EmpowerAction 残余
        .route(
            "/api/person/empower/list/{id}/next/{count}",
            get(u2::empower_list_next),
        )
        .route(
            "/api/person/empower/list/{id}/prev/{count}",
            get(u2::empower_list_prev),
        )
        .route(
            "/api/person/empower/list/person/{flag}",
            get(u2::empower_list_with_person),
        )
        .route(
            "/api/person/empower/{id}/mockputtopost",
            post(empower::update),
        )
        .route(
            "/api/person/empower/manager/{id}/mockputtopost",
            post(empower::manager_update),
        )
        .route(
            "/api/person/empower/{id}/mockdeletetoget",
            get(empower::delete),
        )
        .route(
            "/api/person/empower/manager/{id}/mockdeletetoget",
            get(empower::manager_delete),
        )
        // EmpowerLogAction
        .route(
            "/api/person/empowerlog/list/{id}/next/{count}",
            get(u2::log_list_next),
        )
        .route(
            "/api/person/empowerlog/list/{id}/prev/{count}",
            get(u2::log_list_prev),
        )
        .route(
            "/api/person/empowerlog/list/currentperson/paging/{page}/size/{size}",
            post(u2::log_currentperson_paging),
        )
        .route(
            "/api/person/empowerlog/list/to/currentperson/paging/{page}/size/{size}",
            post(u2::log_to_currentperson_paging),
        )
        .route(
            "/api/person/empowerlog/manager/list/paging/{page}/size/{size}",
            post(u2::log_manager_paging),
        )
        .route(
            "/api/person/empowerlog/{id}",
            axum::routing::delete(u2::log_delete),
        )
        .route(
            "/api/person/empowerlog/{id}/mockdeletetoget",
            get(u2::log_delete),
        )
        // ExmailAction
        .route("/api/person/exmail/new/count", get(u2::exmail_new_count))
        .route(
            "/api/person/exmail/new/count/passive",
            get(u2::exmail_new_count_passive),
        )
        .route(
            "/api/person/exmail/list/title/passive",
            get(u2::exmail_list_title_passive),
        )
        .route("/api/person/exmail/sso", get(u2::exmail_sso))
        .route("/api/person/exmail", get(u2::exmail_callback_get))
        .route("/api/person/exmail", post(u2::exmail_callback_post))
        // ── o2server 契约路径别名（2026-10-24 R1，OA-P0-1 收口）────────────
        // 契约（scripts/all_java_handlers.json 的 x_organization_assemble_personal，
        // 76 条）以 /api/organization/assemble/personal/ 为基路径；Rust 前端一直调
        // /api/person/* 与 /api/reset/*。此处把契约路径**原样委托同一 handler**，
        // 旧路径行为零变化（前提是 shared 中间件四张表——豁免/限流/权限/admin——
        // 已按别名形态补齐且 parity 为 0，见 tests 的契约别名守卫）。
        // 每条注释 <- 后是该别名委托的旧路由；handler 相同则行为逐位相同。
        .route("/api/organization/assemble/personal/custom/manager/person/{person}/name/{name}", get(u2::custom_manager_get))  // <- GET /api/person/custom/manager/person/{person}/name/{name} [u2::custom_manager_get]
        .route("/api/organization/assemble/personal/custom/manager/person/{person}/name/{name}", put(u2::custom_manager_edit))  // <- PUT /api/person/custom/manager/person/{person}/name/{name} [u2::custom_manager_edit]
        .route("/api/organization/assemble/personal/custom/manager/person/{person}/name/{name}/mockputtopost", post(u2::custom_manager_edit))  // <- POST /api/person/custom/manager/person/{person}/name/{name}/mockputtopost [u2::custom_manager_edit]
        .route("/api/organization/assemble/personal/custom/{name}", delete(u2::custom_delete))  // <- DELETE /api/person/custom/{name} [u2::custom_delete]
        .route("/api/organization/assemble/personal/custom/{name}", get(u2::custom_get))  // <- GET /api/person/custom/{name} [u2::custom_get]
        .route("/api/organization/assemble/personal/custom/{name}", put(u2::custom_edit))  // <- PUT /api/person/custom/{name} [u2::custom_edit]
        .route("/api/organization/assemble/personal/custom/{name}/mockdeletetoget", get(u2::custom_delete))  // <- GET /api/person/custom/{name}/mockdeletetoget [u2::custom_delete]
        .route("/api/organization/assemble/personal/definition/{name}", delete(u2::definition_delete))  // <- DELETE /api/person/definition/{name} [u2::definition_delete]
        .route("/api/organization/assemble/personal/definition/{name}", get(u2::definition_get))  // <- GET /api/person/definition/{name} [u2::definition_get]
        .route("/api/organization/assemble/personal/definition/{name}", put(u2::definition_edit))  // <- PUT /api/person/definition/{name} [u2::definition_edit]
        .route("/api/organization/assemble/personal/definition/{name}/mockdeletetoget", get(u2::definition_delete))  // <- GET /api/person/definition/{name}/mockdeletetoget [u2::definition_delete]
        .route("/api/organization/assemble/personal/definition/{name}/mockputtopost", post(u2::definition_edit))  // <- POST /api/person/definition/{name}/mockputtopost [u2::definition_edit]
        .route("/api/organization/assemble/personal/empower/list/person/{flag}", get(u2::empower_list_with_person))  // <- GET /api/person/empower/list/person/{flag} [u2::empower_list_with_person]
        .route("/api/organization/assemble/personal/empower/list/{id}/next/{count}", get(u2::empower_list_next))  // <- GET /api/person/empower/list/{id}/next/{count} [u2::empower_list_next]
        .route("/api/organization/assemble/personal/empower/list/{id}/prev/{count}", get(u2::empower_list_prev))  // <- GET /api/person/empower/list/{id}/prev/{count} [u2::empower_list_prev]
        .route("/api/organization/assemble/personal/empower/manager/{id}/mockdeletetoget", get(empower::manager_delete))  // <- GET /api/person/empower/manager/{id}/mockdeletetoget [empower::manager_delete]
        .route("/api/organization/assemble/personal/empower/manager/{id}/mockputtopost", post(empower::manager_update))  // <- POST /api/person/empower/manager/{id}/mockputtopost [empower::manager_update]
        .route("/api/organization/assemble/personal/empower/{id}/mockdeletetoget", get(empower::delete))  // <- GET /api/person/empower/{id}/mockdeletetoget [empower::delete]
        .route("/api/organization/assemble/personal/empower/{id}/mockputtopost", post(empower::update))  // <- POST /api/person/empower/{id}/mockputtopost [empower::update]
        .route("/api/organization/assemble/personal/empowerlog/list/currentperson/paging/{page}/size/{size}", post(u2::log_currentperson_paging))  // <- POST /api/person/empowerlog/list/currentperson/paging/{page}/size/{size} [u2::log_currentperson_paging]
        .route("/api/organization/assemble/personal/empowerlog/list/to/currentperson/paging/{page}/size/{size}", post(u2::log_to_currentperson_paging))  // <- POST /api/person/empowerlog/list/to/currentperson/paging/{page}/size/{size} [u2::log_to_currentperson_paging]
        .route("/api/organization/assemble/personal/empowerlog/list/{id}/next/{count}", get(u2::log_list_next))  // <- GET /api/person/empowerlog/list/{id}/next/{count} [u2::log_list_next]
        .route("/api/organization/assemble/personal/empowerlog/list/{id}/prev/{count}", get(u2::log_list_prev))  // <- GET /api/person/empowerlog/list/{id}/prev/{count} [u2::log_list_prev]
        .route("/api/organization/assemble/personal/empowerlog/manager/list/paging/{page}/size/{size}", post(u2::log_manager_paging))  // <- POST /api/person/empowerlog/manager/list/paging/{page}/size/{size} [u2::log_manager_paging]
        .route("/api/organization/assemble/personal/empowerlog/{id}", delete(u2::log_delete))  // <- DELETE /api/person/empowerlog/{id} [u2::log_delete]
        .route("/api/organization/assemble/personal/empowerlog/{id}/mockdeletetoget", get(u2::log_delete))  // <- GET /api/person/empowerlog/{id}/mockdeletetoget [u2::log_delete]
        .route("/api/organization/assemble/personal/exmail", get(u2::exmail_callback_get))  // <- GET /api/person/exmail [u2::exmail_callback_get]
        .route("/api/organization/assemble/personal/exmail", post(u2::exmail_callback_post))  // <- POST /api/person/exmail [u2::exmail_callback_post]
        .route("/api/organization/assemble/personal/exmail/list/title/passive", get(u2::exmail_list_title_passive))  // <- GET /api/person/exmail/list/title/passive [u2::exmail_list_title_passive]
        .route("/api/organization/assemble/personal/exmail/new/count", get(u2::exmail_new_count))  // <- GET /api/person/exmail/new/count [u2::exmail_new_count]
        .route("/api/organization/assemble/personal/exmail/new/count/passive", get(u2::exmail_new_count_passive))  // <- GET /api/person/exmail/new/count/passive [u2::exmail_new_count_passive]
        .route("/api/organization/assemble/personal/exmail/sso", get(u2::exmail_sso))  // <- GET /api/person/exmail/sso [u2::exmail_sso]
        .route("/api/organization/assemble/personal/icon/{person}", get(icon::get))  // <- GET /api/person/icon/{person} [icon::get]
        .route("/api/organization/assemble/personal/password", put(password::change))  // <- PUT /api/person/password [password::change]
        .route("/api/organization/assemble/personal/password/mockputtopost", post(password::change))  // <- POST /api/person/password/mockputtopost [password::change]
        .route("/api/organization/assemble/personal/person", get(get_person))  // <- GET /api/person [get_person]
        .route("/api/organization/assemble/personal/person", put(edit_person))  // <- PUT /api/person [edit_person]
        .route("/api/organization/assemble/personal/person/icon", post(u2::set_icon_octet_stream))  // <- POST /api/person/icon [u2::set_icon_octet_stream]
        .route("/api/organization/assemble/personal/person/icon/mockputtopost", post(u2::upload_multipart_alias))  // <- POST /api/person/icon/mockputtopost [u2::upload_multipart_alias]
        .route("/api/organization/assemble/personal/person/mockputtopost", post(edit_person))  // <- POST /api/person/mockputtopost [edit_person]
        .route("/api/organization/assemble/personal/person/password", put(password::change))  // <- PUT /api/person/password [password::change]
        .route("/api/organization/assemble/personal/person/password/mockputtopost", post(password::change))  // <- POST /api/person/password/mockputtopost [password::change]
        .route("/api/organization/assemble/personal/regist", post(regist::register))  // <- POST /api/person/regist [regist::register]
        .route("/api/organization/assemble/personal/regist/captcha/width/{width}/height/{height}", get(auth::captcha::captcha_with_size))  // <- GET /api/person/regist/captcha/width/{width}/height/{height} [auth::captcha::captcha_with_size]
        .route("/api/organization/assemble/personal/regist/check/mobile/{mobile}", get(regist::check_mobile))  // <- GET /api/person/regist/check/mobile/{mobile} [regist::check_mobile]
        .route("/api/organization/assemble/personal/regist/check/name/{name}", get(regist::check_name))  // <- GET /api/person/regist/check/name/{name} [regist::check_name]
        .route("/api/organization/assemble/personal/regist/code/mobile/{mobile}", get(u2::regist_code_mobile))  // <- GET /api/person/regist/code/mobile/{mobile} [u2::regist_code_mobile]
        .route("/api/organization/assemble/personal/regist/mode", get(u2::regist_mode))  // <- GET /api/person/regist/mode [u2::regist_mode]
        .route("/api/organization/assemble/personal/reset", put(reset::reset_password))  // <- PUT /api/reset [reset::reset_password]
        .route("/api/organization/assemble/personal/reset/check/credential/{credential}", get(reset::check_credential))  // <- GET /api/reset/check/credential/{credential} [reset::check_credential]
        .route("/api/organization/assemble/personal/reset/code/credential/{credential}", get(reset::send_code))  // <- GET /api/reset/code/credential/{credential} [reset::send_code]
        .route("/api/organization/assemble/personal/reset/mockputtopost", post(reset::reset_password_mockputtopost))  // <- POST /api/reset/mockputtopost [reset::reset_password_mockputtopost]
        .route("/api/organization/assemble/personal/reset/password/anonymous", post(reset::reset_password_anonymous))  // <- POST /api/reset/password/anonymous [reset::reset_password_anonymous]
        .route("/api/organization/assemble/personal/signature/delete/{id}", get(signature::delete))  // <- GET /api/person/signature/delete/{id} [signature::delete]
        .route("/api/organization/assemble/personal/signature/list", get(signature::list))  // <- GET /api/person/signature/list [signature::list]
        .route("/api/organization/assemble/personal/signature/list/person/{flag}", get(u2::signature_list_person))  // <- GET /api/person/signature/list/person/{flag} [u2::signature_list_person]
        .route("/api/organization/assemble/personal/signature/upload", post(signature::upload))  // <- POST /api/person/signature/upload [signature::upload]
        .layer(Extension(pool))
        .layer(Extension(session_manager))
        .layer(Extension(reset_store))
}

#[cfg(test)]
mod security_gate_tests_b76;
#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests;
#[cfg(test)]
mod tests_generated;
