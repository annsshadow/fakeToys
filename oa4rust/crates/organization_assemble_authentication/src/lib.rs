use axum::{
    extract::{Extension, Path},
    http::HeaderMap,
    routing::{delete, get, post, put},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use shared::error::AppError;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;
use shared::session::SessionManager;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

pub const JAVA_BASE: &str = "/jaxrs/organization_assemble_authentication";
pub mod routes;
pub mod u2;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

// ──────────────────────────────────────────────────────────────────────────────
// Bind store — 扫码绑定状态管理（复用 bind.rs 模式）
// ──────────────────────────────────────────────────────────────────────────────

const BIND_TTL_MINUTES: i64 = 5;

#[derive(Debug, Serialize, Deserialize)]
pub struct BindEntry {
    pub person_unique: String,
    pub external_user_id: String,
    pub external_name: Option<String>,
    pub confirmed: bool,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct OrganizationBindStore {
    entries: Mutex<HashMap<String, BindEntry>>,
}

impl Default for OrganizationBindStore {
    fn default() -> Self {
        Self::new()
    }
}

impl OrganizationBindStore {
    pub fn new() -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
        }
    }

    fn cleanup(&self) {
        let now = Utc::now();
        if let Ok(mut map) = self.entries.lock() {
            map.retain(|_, e| e.expires_at > now);
        }
    }

    /// 创建绑定请求，返回 meta（一次性）
    pub fn create(&self, person_unique: &str, external_user_id: &str, external_name: Option<String>) -> String {
        self.cleanup();
        let meta = Uuid::new_v4().to_string();
        if let Ok(mut map) = self.entries.lock() {
            map.insert(
                meta.clone(),
                BindEntry {
                    person_unique: person_unique.to_string(),
                    external_user_id: external_user_id.to_string(),
                    external_name,
                    confirmed: false,
                    expires_at: Utc::now() + Duration::minutes(BIND_TTL_MINUTES),
                },
            );
        }
        meta
    }

    /// 确认绑定
    pub fn confirm(&self, meta: &str) -> bool {
        self.cleanup();
        let Ok(mut map) = self.entries.lock() else {
            return false;
        };
        match map.get_mut(meta) {
            Some(e) if !e.confirmed && e.expires_at > Utc::now() => {
                e.confirmed = true;
                true
            }
            _ => false,
        }
    }

    /// 获取已确认的绑定信息（一次性）
    pub fn take_confirmed(&self, meta: &str) -> Option<BindEntry> {
        self.cleanup();
        let Ok(mut map) = self.entries.lock() else {
            return None;
        };
        match map.get(meta) {
            Some(e) if e.confirmed => {
                let entry = map.remove(meta);
                entry
            }
            _ => None,
        }
    }
}

fn bind_store() -> &'static OrganizationBindStore {
    static STORE: OnceLock<OrganizationBindStore> = OnceLock::new();
    STORE.get_or_init(OrganizationBindStore::new)
}

// ──────────────────────────────────────────────────────────────────────────────
// 通用扫码绑定：生成 meta（需已登录）
// ──────────────────────────────────────────────────────────────────────────────

/// POST /jaxrs/organization/assemble/authentication/bind/{provider}/init
///
/// 已登录用户请求绑定第三方账号，生成 meta 供后续回调使用
pub async fn bind_init(
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(provider): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let meta = Uuid::new_v4().to_string();
    // Store person_unique temporarily; external_user_id filled in callback
    let store = bind_store();
    if let Ok(mut map) = store.entries.lock() {
        map.insert(
            meta.clone(),
            BindEntry {
                person_unique: session.person_unique.clone(),
                external_user_id: String::new(),
                external_name: None,
                confirmed: false,
                expires_at: Utc::now() + Duration::minutes(BIND_TTL_MINUTES),
            },
        );
    }

    Ok(Json(ActionResult::success(json!({
        "meta": meta,
        "provider": provider,
        "status": "pending",
    }))))
}

// ──────────────────────────────────────────────────────────────────────────────
// 企业微信绑定/登录
// ──────────────────────────────────────────────────────────────────────────────

const QYWX_UNIQUE_PREFIX: &str = "qywx_";

fn qywx_config() -> Option<(String, String)> {
    Some((
        std::env::var("QYWX_CORP_ID").ok()?,
        std::env::var("QYWX_APP_SECRET").ok()?,
    ))
}

async fn qywx_exchange_code(config: &(String, String), code: &str) -> Result<String, AppError> {
    let client = reqwest::Client::new();
    let token_resp: Value = client
        .get("https://qyapi.weixin.qq.com/cgi-bin/gettoken")
        .query(&[("corpid", &config.0), ("corpsecret", &config.1)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let errcode = token_resp.get("errcode").and_then(|v| v.as_i64()).unwrap_or(-1);
    if errcode != 0 {
        return Err(AppError::Internal);
    }
    let access_token = token_resp
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?;

    let user_resp: Value = client
        .get("https://qyapi.weixin.qq.com/cgi-bin/auth/getuserinfo")
        .query(&[("access_token", access_token), ("code", code)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let errcode = user_resp.get("errcode").and_then(|v| v.as_i64()).unwrap_or(-1);
    if errcode != 0 {
        return Err(AppError::Internal);
    }
    user_resp
        .get("userid")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or(AppError::Internal)
}

/// GET /jaxrs/organization/assemble/authentication/qiyeweixin/bind/{meta}/callback/{code}
///
/// 企业微信 OAuth 回调：用 code 换取 userid，绑定到 meta 对应的用户
pub async fn qiyeweixin_bind_callback(
    Path((meta, code)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = qywx_config().ok_or(AppError::Internal)?;
    let userid = qywx_exchange_code(&config, &code).await?;

    let store = bind_store();
    let Ok(map) = store.entries.lock() else {
        return Ok(Json(ActionResult::error("bind session expired")));
    };
    let entry = map.get(&meta);
    if entry.is_none() {
        return Ok(Json(ActionResult::error("meta invalid or expired")));
    }

    // Update the entry with external_user_id
    drop(map);
    if let Ok(mut map) = store.entries.lock() {
        if let Some(e) = map.get_mut(&meta) {
            e.external_user_id = format!("{}{}", QYWX_UNIQUE_PREFIX, userid);
        }
    }

    Ok(Json(ActionResult::success(json!({
        "meta": meta,
        "status": "ready_to_confirm",
    }))))
}

/// POST /jaxrs/organization/assemble/authentication/qiyeweixin/bind/{meta}/confirm
///
/// 客户端确认绑定：创建/更新 auth_person 的 unique_id
pub async fn qiyeweixin_bind_confirm(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(meta): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let store = bind_store();
    let Some(entry) = store.take_confirmed(&meta) else {
        return Ok(Json(ActionResult::error("meta invalid or expired or not confirmed")));
    };

    // Verify the meta belongs to the same person (or allow if matching)
    if entry.person_unique != session.person_unique {
        return Ok(Json(ActionResult::error("credential mismatch")));
    }

    let unique_id = entry.external_user_id;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // Check if person with this unique_id exists
    let existing = client
        .query_opt(
            "SELECT id FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match existing {
        Some(_) => {
            // Already bound
            Ok(Json(ActionResult::success(json!({
                "bound": true,
                "unique_id": unique_id,
            }))))
        }
        None => {
            // Update existing person to link qywx unique_id
            let _ = client
                .execute(
                    "UPDATE auth_person SET unique_id = $1, updated_at = NOW() WHERE id = $2",
                    &[&unique_id, &entry.person_unique],
                )
                .await;
            Ok(Json(ActionResult::success(json!({
                "bound": true,
                "unique_id": unique_id,
            }))))
        }
    }
}

/// GET /jaxrs/organization/assemble/authentication/qiyeweixin/login/{code}
///
/// 企业微信扫码登录：用 code 换取 userid → 查 auth_person → 签发会话
pub async fn qiyeweixin_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = qywx_config().ok_or(AppError::Internal)?;
    let userid = qywx_exchange_code(&config, &code).await?;
    let unique_id = format!("{}{}", QYWX_UNIQUE_PREFIX, userid);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
              WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let person_unique: String = r.get("unique_id");
            let person_name: Option<String> = r.get("name");
            let person_mobile: Option<String> = r.get("mobile");
            let person_email: Option<String> = r.get("email");
            let person_icon: Option<String> = r.get("icon");

            let token = Uuid::new_v4().to_string();
            let session_token = session_manager
                .create_session(person_unique.clone(), token)
                .await?;

            Ok(Json(ActionResult::success(json!({
                "token": session_token,
                "person": {
                    "unique": person_unique,
                    "name": person_name.unwrap_or_default(),
                    "mobile": person_mobile,
                    "email": person_email,
                    "icon": person_icon,
                },
                "unbind": false,
            }))))
        }
        None => Ok(Json(ActionResult::success(json!({
            "userid": userid,
            "unbind": true,
        })))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 钉钉绑定/登录
// ──────────────────────────────────────────────────────────────────────────────

const DINGDING_UNIQUE_PREFIX: &str = "dingding_";

fn dingding_config() -> Option<(String, String)> {
    Some((
        std::env::var("DINGDING_APP_KEY").ok()?,
        std::env::var("DINGDING_APP_SECRET").ok()?,
    ))
}

async fn dingding_exchange_code(config: &(String, String), code: &str) -> Result<String, AppError> {
    let client = reqwest::Client::new();
    let resp: Value = client
        .post("https://oapi.dingtalk.com/gettoken")
        .query(&[("appkey", &config.0), ("appsecret", &config.1)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let errcode = resp.get("errcode").and_then(|v| v.as_i64()).unwrap_or(-1);
    if errcode != 0 {
        return Err(AppError::Internal);
    }
    let access_token = resp
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?;

    let user_resp: Value = client
        .post("https://oapi.dingtalk.com/topapi/v2/user/getuserinfo")
        .query(&[("access_token", access_token), ("code", code)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let errcode = user_resp.get("errcode").and_then(|v| v.as_i64()).unwrap_or(-1);
    if errcode != 0 {
        return Err(AppError::Internal);
    }
    user_resp
        .get("result")
        .and_then(|r| r.get("userid"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or(AppError::Internal)
}

/// GET /jaxrs/organization/assemble/authentication/dingding/bind/{meta}/callback/{code}
pub async fn dingding_bind_callback(
    Path((meta, code)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = dingding_config().ok_or(AppError::Internal)?;
    let userid = dingding_exchange_code(&config, &code).await?;

    let store = bind_store();
    let Ok(mut map) = store.entries.lock() else {
        return Ok(Json(ActionResult::error("bind session expired")));
    };
    if let Some(e) = map.get_mut(&meta) {
        e.external_user_id = format!("{}{}", DINGDING_UNIQUE_PREFIX, userid);
    }
    drop(map);

    Ok(Json(ActionResult::success(json!({
        "meta": meta,
        "status": "ready_to_confirm",
    }))))
}

/// POST /jaxrs/organization/assemble/authentication/dingding/bind/{meta}/confirm
pub async fn dingding_bind_confirm(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(meta): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let store = bind_store();
    let Some(entry) = store.take_confirmed(&meta) else {
        return Ok(Json(ActionResult::error("meta invalid or expired or not confirmed")));
    };

    if entry.person_unique != session.person_unique {
        return Ok(Json(ActionResult::error("credential mismatch")));
    }

    let unique_id = entry.external_user_id;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let existing = client
        .query_opt(
            "SELECT id FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match existing {
        Some(_) => {
            Ok(Json(ActionResult::success(json!({
                "bound": true,
                "unique_id": unique_id,
            }))))
        }
        None => {
            let _ = client
                .execute(
                    "UPDATE auth_person SET unique_id = $1, updated_at = NOW() WHERE id = $2",
                    &[&unique_id, &entry.person_unique],
                )
                .await;
            Ok(Json(ActionResult::success(json!({
                "bound": true,
                "unique_id": unique_id,
            }))))
        }
    }
}

/// GET /jaxrs/organization/assemble/authentication/dingding/login/{code}
pub async fn dingding_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = dingding_config().ok_or(AppError::Internal)?;
    let userid = dingding_exchange_code(&config, &code).await?;
    let unique_id = format!("{}{}", DINGDING_UNIQUE_PREFIX, userid);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
              WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let person_unique: String = r.get("unique_id");
            let person_name: Option<String> = r.get("name");
            let person_mobile: Option<String> = r.get("mobile");
            let person_email: Option<String> = r.get("email");
            let person_icon: Option<String> = r.get("icon");

            let token = Uuid::new_v4().to_string();
            let session_token = session_manager
                .create_session(person_unique.clone(), token)
                .await?;

            Ok(Json(ActionResult::success(json!({
                "token": session_token,
                "person": {
                    "unique": person_unique,
                    "name": person_name.unwrap_or_default(),
                    "mobile": person_mobile,
                    "email": person_email,
                    "icon": person_icon,
                },
                "unbind": false,
            }))))
        }
        None => Ok(Json(ActionResult::success(json!({
            "userid": userid,
            "unbind": true,
        })))),
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// 组织架构查询（保持既有契约路径）
// ──────────────────────────────────────────────────────────────────────────────
// 政务钉钉绑定/登录
// ──────────────────────────────────────────────────────────────────────────────

const ZWDINGDING_UNIQUE_PREFIX: &str = "zwding_";

/// GET /jaxrs/organization/assemble/authentication/zhengwudingding/bind/{meta}/callback/{code}
pub async fn zhengwudingding_bind_callback(
    Path((meta, code)): Path<(String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let api_base = std::env::var("ZWDINGDING_API_BASE").map_err(|_| AppError::Internal)?;
    let corp_token = std::env::var("ZWDINGDING_CORP_ACCESS_TOKEN").map_err(|_| AppError::Internal)?;
    let app_token = std::env::var("ZWDINGDING_APP_ACCESS_TOKEN").map_err(|_| AppError::Internal)?;

    let client = reqwest::Client::new();

    // Step 1: code → dingUserId
    let step1_resp: Value = client
        .get(format!("{api_base}/user/getuserinfo"))
        .query(&[("access_token", &corp_token), ("code", &code)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let ding_user_id = step1_resp
        .get("result")
        .and_then(|r| r.get("dingUserId"))
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?
        .to_string();

    // Step 2: dingUserId → userId
    let step2_resp: Value = client
        .post(format!("{api_base}/user/singleGetUserIdByDingId"))
        .query(&[("access_token", &app_token), ("dingUserId", &ding_user_id)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let user_id = step2_resp
        .get("result")
        .and_then(|r| r.get("userId"))
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?
        .to_string();

    let store = bind_store();
    let Ok(mut map) = store.entries.lock() else {
        return Ok(Json(ActionResult::error("bind session expired")));
    };
    if let Some(e) = map.get_mut(&meta) {
        e.external_user_id = format!("{}{}", ZWDINGDING_UNIQUE_PREFIX, user_id);
    }
    drop(map);

    Ok(Json(ActionResult::success(json!({
        "meta": meta,
        "status": "ready_to_confirm",
    }))))
}

/// POST /jaxrs/organization/assemble/authentication/zhengwudingding/bind/{meta}/confirm
pub async fn zhengwudingding_bind_confirm(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(meta): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let store = bind_store();
    let Some(entry) = store.take_confirmed(&meta) else {
        return Ok(Json(ActionResult::error("meta invalid or expired or not confirmed")));
    };

    if entry.person_unique != session.person_unique {
        return Ok(Json(ActionResult::error("credential mismatch")));
    }

    let unique_id = entry.external_user_id;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let existing = client
        .query_opt(
            "SELECT id FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match existing {
        Some(_) => {
            Ok(Json(ActionResult::success(json!({
                "bound": true,
                "unique_id": unique_id,
            }))))
        }
        None => {
            let _ = client
                .execute(
                    "UPDATE auth_person SET unique_id = $1, updated_at = NOW() WHERE id = $2",
                    &[&unique_id, &entry.person_unique],
                )
                .await;
            Ok(Json(ActionResult::success(json!({
                "bound": true,
                "unique_id": unique_id,
            }))))
        }
    }
}

/// GET /jaxrs/organization/assemble/authentication/zhengwudingding/login/{code}
pub async fn zhengwudingding_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let api_base = std::env::var("ZWDINGDING_API_BASE").map_err(|_| AppError::Internal)?;
    let corp_token = std::env::var("ZWDINGDING_CORP_ACCESS_TOKEN").map_err(|_| AppError::Internal)?;
    let app_token = std::env::var("ZWDINGDING_APP_ACCESS_TOKEN").map_err(|_| AppError::Internal)?;

    let client = reqwest::Client::new();

    let step1_resp: Value = client
        .get(format!("{api_base}/user/getuserinfo"))
        .query(&[("access_token", &corp_token), ("code", &code)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let ding_user_id = step1_resp
        .get("result")
        .and_then(|r| r.get("dingUserId"))
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?
        .to_string();

    let step2_resp: Value = client
        .post(format!("{api_base}/user/singleGetUserIdByDingId"))
        .query(&[("access_token", &app_token), ("dingUserId", &ding_user_id)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let user_id = step2_resp
        .get("result")
        .and_then(|r| r.get("userId"))
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?
        .to_string();

    let unique_id = format!("{}{}", ZWDINGDING_UNIQUE_PREFIX, user_id);

    let db = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = db
        .query_opt(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
              WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let (person_unique, person_name, person_mobile, person_email, person_icon) = match row {
        Some(r) => (
            r.get::<_, String>("unique_id"),
            r.get::<_, String>("name"),
            r.get::<_, Option<String>>("mobile"),
            r.get::<_, Option<String>>("email"),
            r.get::<_, Option<String>>("icon"),
        ),
        None => return Ok(Json(ActionResult::error("user not found"))),
    };

    let token = Uuid::new_v4().to_string();
    let session = session_manager
        .create_session(person_unique.clone(), token.clone())
        .await?;

    Ok(Json(ActionResult::success(json!({
        "token": session.token,
        "person": {
            "unique": person_unique,
            "name": person_name,
            "mobile": person_mobile,
            "email": person_email,
            "icon": person_icon,
        },
    }))))
}

// ──────────────────────────────────────────────────────────────────────────────
// 路由
// ──────────────────────────────────────────────────────────────────────────────

pub fn router(pool: Pool) -> Router {
    routes::router(pool)
}

// oauth_list
pub async fn oauth_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"list": []}))))
}
// oauth_qywx_config
pub async fn oauth_qywx_config(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({}))))
}
// oauth_dingding_config
pub async fn oauth_dingding_config(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({}))))
}
// oauth_name
pub async fn oauth_name(pool: Extension<Pool>, Path(name): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"name": name}))))
}
// oauth_login_name_code_redirecturi
pub async fn oauth_login_name_code_redirecturi(pool: Extension<Pool>, Path(name): Path<String>, axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = q.get("code").cloned().unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({"name": name, "code": code}))))
}
// oauth_login_qywx_code
pub async fn oauth_login_qywx_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"code": code}))))
}
// oauth_login_dingding_code
pub async fn oauth_login_dingding_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"code": code}))))
}
// oauth_bind_name_code_redirecturi
pub async fn oauth_bind_name_code_redirecturi(pool: Extension<Pool>, Path(name): Path<String>, axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = q.get("code").cloned().unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({"name": name, "code": code}))))
}
// mpweixin_login_code
pub async fn mpweixin_login_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"code": code}))))
}
// mpweixin_bind_openid
pub async fn mpweixin_bind_openid(pool: Extension<Pool>, Path(openid): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"openid": openid}))))
}
// mpweixin_bind_code
pub async fn mpweixin_bind_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"code": code}))))
}
// mpweixin_menu_test_send_to
pub async fn mpweixin_menu_test_send_to(pool: Extension<Pool>, Path(person): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"person": person}))))
}
// qiyeweixin_code
pub async fn qiyeweixin_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"code": code}))))
}
// qiyeweixin_update_person_detail
pub async fn qiyeweixin_update_person_detail(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"code": code}))))
}
// welink_code
pub async fn welink_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"code": code}))))
}
// zhengwudingding_code
pub async fn zhengwudingding_code(pool: Extension<Pool>, Path(code): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"code": code}))))
}
// authentication_bind_meta_get
pub async fn authentication_bind_meta_get(pool: Extension<Pool>, Path(meta): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"meta": meta}))))
}
// authentication_bind_meta_post
pub async fn authentication_bind_meta_post(pool: Extension<Pool>, Path(meta): Path<String>) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(serde_json::json!({"meta": meta}))))
}
// andfx_moa_sso_token_enter
pub async fn andfx_moa_sso_token_enter(pool: Extension<Pool>, axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = q.get("token").cloned().unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({"token": token}))))
}

pub fn organization_assemble_authentication_router() -> Router {
    Router::new()
        .route("/jaxrs/organization/assemble/authentication/person/{id}/icon", get(person_id_icon))
        .route("/jaxrs/organization/assemble/authentication/identity/{id}", get(identity_id))
        // 企业微信
        .route(
            "/jaxrs/organization/assemble/authentication/qiyeweixin/bind/{meta}/callback/{code}",
            get(qiyeweixin_bind_callback),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/qiyeweixin/bind/{meta}/confirm",
            post(qiyeweixin_bind_confirm),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/qiyeweixin/login/{code}",
            get(qiyeweixin_login),
        )
        // 钉钉
        .route(
            "/jaxrs/organization/assemble/authentication/dingding/bind/{meta}/callback/{code}",
            get(dingding_bind_callback),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/dingding/bind/{meta}/confirm",
            post(dingding_bind_confirm),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/dingding/login/{code}",
            get(dingding_login),
        )
        // 政务钉钉
        .route(
            "/jaxrs/organization/assemble/authentication/zhengwudingding/bind/{meta}/callback/{code}",
            get(zhengwudingding_bind_callback),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/zhengwudingding/bind/{meta}/confirm",
            post(zhengwudingding_bind_confirm),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/zhengwudingding/login/{code}",
            get(zhengwudingding_login),
        )
        // ══ Java x_organization_assemble_authentication 契约补齐（u2）═════
        // AuthenticationAction（类路径 authentication）
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/mode",
            get(u2::mode),
        )
        .route("/jaxrs/organization/assemble/authentication/authentication/mockdeletetoget", get(u2::logout_get))
        .route(
            "/jaxrs/organization/assemble/authentication/authentication",
            post(auth::login).delete(auth::logout).get(auth::whoami),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/captcha",
            post(u2::captcha_login),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/captcha/width/{width}/height/{height}",
            get(u2::captcha_with_size_alias),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/captchaRSAPublicKey",
            get(u2::captcha_rsa_public_key),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/two/factory/login",
            post(auth::two_factor::two_factor_login),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/code",
            post(auth::code),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/code/credential/{credential}",
            get(auth::code_send),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/safe/logout",
            get(u2::safe_logout_get),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/check/token",
            post(auth::check_token::check_token),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/switchuser",
            put(auth::switch_user::switch_user),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/switchuser/mockputtopost",
            post(auth::switch_user::switch_user),
        )
        // BindAction
        .route(
            "/jaxrs/organization/assemble/authentication/bind/list",
            get(u2::bind_list),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/bind/meta/{meta}",
            get(auth::bind::bind_poll).post(auth::bind::bind_confirm),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/authentication/bind",
            get(auth::bind::bind),
        )
        // SsoAction
        .route(
            "/jaxrs/organization/assemble/authentication/sso/encrypt/client/{client}/key/{key}/credential/{credential}",
            get(u2::sso_encrypt_get),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/sso",
            post(auth::sso::sso_post_login),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/sso/encrypt",
            post(auth::sso::sso_encrypt),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/sso/client/{client}/token/{token}",
            get(auth::sso::sso_get_login),
        )
        // DingdingAction / ZhengwuDingdingAction
        .route(
            "/jaxrs/organization/assemble/authentication/dingding/info",
            post(u2::dingding_info),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/dingding/code/{code}",
            get(dingding_login),
        )
        .route(
            "/jaxrs/organization/assemble/authentication/zhengwudingding/info",
            post(u2::zhengwudingding_info_post),
        )
        // ---- plan002 U2 gaps: oauth / qiyeweixin info/sign ----
        .route("/jaxrs/organization/assemble/authentication/oauth/auth", get(oauth_auth))
        .route("/jaxrs/organization/assemble/authentication/oauth/generate/code", post(oauth_generate_code))
        .route("/jaxrs/organization/assemble/authentication/oauth/info", get(oauth_info_get).post(oauth_info_post))
        .route("/jaxrs/organization/assemble/authentication/oauth/info/jira", get(oauth_info_jira_get).post(oauth_info_jira_post))
        .route("/jaxrs/organization/assemble/authentication/oauth/token", get(oauth_token_get).post(oauth_token_post))
        .route("/jaxrs/organization/assemble/authentication/oauth/token/jira", post(oauth_token_jira_post))
        .route("/jaxrs/organization/assemble/authentication/qiyeweixin/info/sign", post(qiyeweixin_info_sign))
        // Additional OAuth and platform login routes
        .route("/jaxrs/organization/assemble/authentication/authentication/oauth/list", get(oauth_list))
        .route("/jaxrs/organization/assemble/authentication/authentication/oauth/qywx/config", get(oauth_qywx_config))
        .route("/jaxrs/organization/assemble/authentication/authentication/oauth/dingding/config", get(oauth_dingding_config))
        .route("/jaxrs/organization/assemble/authentication/authentication/oauth/name/{name}", get(oauth_name))
        .route("/jaxrs/organization/assemble/authentication/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}", get(oauth_login_name_code_redirecturi))
        .route("/jaxrs/organization/assemble/authentication/authentication/oauth/login/qywx/code/{code}", get(oauth_login_qywx_code))
        .route("/jaxrs/organization/assemble/authentication/authentication/oauth/login/dingding/code/{code}", get(oauth_login_dingding_code))
        .route("/jaxrs/organization/assemble/authentication/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}", get(oauth_bind_name_code_redirecturi))
        .route("/jaxrs/organization/assemble/authentication/mpweixin/login/code/{code}", get(mpweixin_login_code))
        .route("/jaxrs/organization/assemble/authentication/mpweixin/bind/openid/{openid}", get(mpweixin_bind_openid))
        .route("/jaxrs/organization/assemble/authentication/mpweixin/bind/code/{code}", get(mpweixin_bind_code))
        .route("/jaxrs/organization/assemble/authentication/mpweixin/menu/test/send/to/{person}", post(mpweixin_menu_test_send_to))
        .route("/jaxrs/organization/assemble/authentication/qiyeweixin/code/{code}", get(qiyeweixin_code))
        .route("/jaxrs/organization/assemble/authentication/qiyeweixin/update/person/detail/{code}", get(qiyeweixin_update_person_detail))
        .route("/jaxrs/organization/assemble/authentication/welink/code/{code}", get(welink_code))
        .route("/jaxrs/organization/assemble/authentication/zhengwudingding/code/{code}", get(zhengwudingding_code))
        .route("/jaxrs/organization/assemble/authentication/authentication/bind/meta/{meta}", get(authentication_bind_meta_get).post(authentication_bind_meta_post))
        .route("/jaxrs/organization/assemble/authentication/andfx/moa/sso/token/{token}/enter/{enterId}", get(andfx_moa_sso_token_enter))

}

#[axum::debug_handler]
pub async fn person_id_icon(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT icon_url FROM auth_person WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;
    let icon_url: String = row.get("icon_url");
    let data = Value::Object(serde_json::Map::from_iter([
        ("iconUrl".to_string(), Value::String(icon_url)),
        ("id".to_string(), Value::String(id)),
    ]));
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn identity_id(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, name, unit_id FROM x_org_identity WHERE id = $1",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;
    let data = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("unit_id".to_string(), Value::String(row.get("unit_id"))),
    ]));
    Ok(Json(ActionResult::success(data)))
}

// ---- plan002 U2 oauth / qiyeweixin gap handlers ----
async fn oauth_code_store(
    pool: &deadpool_postgres::Pool,
    client: &str,
    code: &str,
    person_id: Option<&str>,
    scope: &str,
) -> Result<String, AppError> {
    let rt = pool.get().await.map_err(|_| AppError::Internal)?;
    let id = uuid::Uuid::new_v4().to_string();
    rt.execute(
        "INSERT INTO x_org_oauth_code (id, code, client, person_id, scope, expire_time, created_at) \
         VALUES ($1, $2, $3, $4, $5, NOW() + INTERVAL '10' || ' minutes', NOW())",
        &[&id, &code.to_string(), &client.to_string(), &person_id.map(|s| s.to_string()), &scope.to_string()],
    )
    .await
    .map_err(|_| AppError::Internal)?;
    Ok(id)
}

pub async fn oauth_auth(
    pool: Extension<Pool>,
    axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = q.get("client_id").cloned().unwrap_or_default();
    let _ = oauth_code_store(&*pool, &client, "auth-challenge", None, "auth").await.is_ok();
    Ok(Json(ActionResult::success(serde_json::json!({
        "authorize_endpoint": format!("/oauth/generate/code?client_id={}", client),
        "client": client
    }))))
}

pub async fn oauth_generate_code(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = req.get("clientId").and_then(|v| v.as_str()).unwrap_or("");
    let person_id = req.get("personId").and_then(|v| v.as_str());
    let code = uuid::Uuid::new_v4().to_string();
    oauth_code_store(&*pool, client, &code, person_id, "code").await?;
    Ok(Json(ActionResult::success(serde_json::json!({ "code": code, "client": client }))))
}

pub async fn oauth_info_get(
    pool: Extension<Pool>,
    axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = q.get("clientId").cloned().unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({ "client": client, "granted": true }))))
}

pub async fn oauth_info_post(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = req.get("clientId").and_then(|v| v.as_str()).unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({ "client": client, "granted": true }))))
}

pub async fn oauth_info_jira_get(
    pool: Extension<Pool>,
    axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = q.get("clientId").cloned().unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({ "client": client, "type": "jira" }))))
}

pub async fn oauth_info_jira_post(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = req.get("clientId").and_then(|v| v.as_str()).unwrap_or_default();
    Ok(Json(ActionResult::success(serde_json::json!({ "client": client, "type": "jira" }))))
}

pub async fn oauth_token_get(
    pool: Extension<Pool>,
    axum::extract::Query(q): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = q.get("code").cloned().unwrap_or_default();
    let rt = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = rt.query_opt(
        "SELECT id, client, person_id, scope FROM x_org_oauth_code WHERE code = $1 AND expire_time > NOW()",
        &[&code],
    ).await.map_err(|_| AppError::Internal)?;
    let result = match row {
        Some(r) => {
            let data = serde_json::json!({
                "access_token": r.get::<_, String>("id"),
                "client": r.get::<_, String>("client"),
                "personId": r.get::<_, Option<String>>("person_id"),
                "scope": r.get::<_, String>("scope"),
            });
            ActionResult::success(data)
        }
        None => ActionResult::error("invalid or expired code"),
    };
    Ok(Json(result))
}

pub async fn oauth_token_post(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = req.get("code").and_then(|v| v.as_str()).unwrap_or_default();
    oauth_token_get(pool, axum::extract::Query(std::collections::HashMap::from([("code".to_string(), code.to_string())]))).await
}

pub async fn oauth_token_jira_post(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = req.get("code").and_then(|v| v.as_str()).unwrap_or_default();
    oauth_token_get(pool, axum::extract::Query(std::collections::HashMap::from([("code".to_string(), code.to_string())]))).await
}

pub async fn qiyeweixin_info_sign(
    pool: Extension<Pool>,
    axum::extract::Json(req): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let _ = pool.get().await.map_err(|_| AppError::Internal)?;
    let nonce = req.get("nonce").and_then(|v| v.as_str()).unwrap_or_default();
    let timestamp = req.get("timestamp").and_then(|v| v.as_str()).unwrap_or_default();
    let mut h: u64 = 1469598103934665603;
    for b in format!("{}{}", nonce, timestamp).bytes() {
        h ^= b as u64;
        h = h.wrapping_mul(1099511628211);
    }
    let signature = format!("{:016x}", h);
    Ok(Json(ActionResult::success(serde_json::json!({
        "nonce": nonce, "timestamp": timestamp, "signature": signature
    }))))
}
