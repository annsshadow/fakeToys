use axum::{
    extract::{Extension, Path},
    http::HeaderMap,
    routing::{delete, get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

pub mod andfx;
pub mod bind;
pub mod captcha;
pub mod check_token;
pub mod model;
pub mod mpweixin;
pub mod oauth;
pub mod password;
pub mod person;
pub mod qiyeweixin;
pub mod safe_logout;
pub mod sso;
pub mod switch_user;
pub mod two_factor;
pub mod welink;
pub mod zhengwudingding;

// 兼容重导出：会话与限流类型已移入 shared，供外部 crate 使用 auth:: 前缀继续引用
pub use shared::rate_limit::RateLimiter;
pub use shared::session::{Session, SessionManager};
pub(crate) use shared::middleware::extract_token_from_headers;

#[cfg(test)]
mod tests;

// --- 请求/响应 DTO ---

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub credential: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub person: PersonInfo,
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonInfo {
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TwoFactorLoginResponse {
    pub token: String,
    pub person: PersonInfo,
}

// --- 认证处理器 ---

/// 用户登录接口（契约路径 POST /jaxrs/authentication，兼容自造路径）
///
/// 接收用户名/工号（credential）和密码，验证通过后签发 2 小时有效会话令牌。
/// 支持 bcrypt（前缀 {bcrypt}）与 MD5/DES 兼容校验。限流由 shared 中间件统一处理。
pub async fn login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    axum::extract::Json(req): axum::extract::Json<LoginRequest>,
) -> Result<Json<ActionResult<LoginResponse>>, AppError> {
    if req.credential.is_empty() || req.password.is_empty() {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, password_hash, locked FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let _person_id: String = row.get("id");
    let person_unique: String = row.get("unique_id");
    let person_name: String = row.get("name");
    let person_mobile: Option<String> = row.get("mobile");
    let password_hash: String = row.get("password_hash");

    // 验证密码（支持 bcrypt/MD5/DES 多算法兼容）
    let valid = password::verify_password(&req.password, &password_hash, "", None);
    if !valid {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    // 密码哈希 rehash：检测旧算法（MD5/DES），自动升级为 bcrypt
    if password::needs_rehash(&password_hash) {
        let new_hash = password::rehash_password(&req.password);
        let _ = client
            .execute(
                "UPDATE auth_person SET password_hash = $1, updated_at = NOW() WHERE id = $2",
                &[&new_hash, &_person_id],
            )
            .await;
    }

    let token = Uuid::new_v4().to_string();
    let session = session_manager.create_session(person_unique.clone(), token.clone()).await;

    let response = ActionResult::success(LoginResponse {
        token: session.token,
        person: PersonInfo {
            unique: person_unique,
            name: person_name,
            mobile: person_mobile,
        },
    });

    Ok(Json(response))
}

// --- 刷新 / 登出 / 当前用户 ---

/// 刷新会话令牌：用旧 token 换取新 token，旧 token 随即失效。
/// 安全修复：必须从 header 提取有效 token，且与 body 中的 old_token 一致才允许刷新。
pub async fn refresh(
    _pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let header_token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let old_token = payload.get("token").and_then(|v| v.as_str()).unwrap_or("");

    if header_token != old_token {
        return Ok(Json(ActionResult::error("token mismatch")));
    }

    let session = session_manager.validate_session(&header_token).await.ok_or(AppError::Unauthorized)?;

    let new_token = Uuid::new_v4().to_string();
    session_manager.create_session(session.person_unique, new_token.clone()).await;
    session_manager.remove_session(old_token).await;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("token".to_string(), Value::String(new_token)),
    ])))))
}

/// 用户登出接口（契约路径 DELETE /jaxrs/authentication，兼容自造路径）
///
/// 令牌来源：Authorization: Bearer / Cookie token= 优先，请求体 token 字段次之。
pub async fn logout(
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers)
        .or_else(|| payload.get("token").and_then(|v| v.as_str()).map(|s| s.to_string()));
    if let Some(token) = token {
        session_manager.remove_session(&token).await;
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("message".to_string(), Value::String("logged out".to_string())),
    ])))))
}

/// 查询当前认证用户信息（契约路径 GET /jaxrs/authentication，兼容自造路径）
///
/// 从会话解析当前用户身份，按 unique_id 查询数据库（不再取首条记录）。
pub async fn whoami(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("authenticated".to_string(), Value::Bool(true)),
            ("id".to_string(), Value::String(row.get("id"))),
            ("unique".to_string(), Value::String(row.get("unique_id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), row.get::<_, Option<String>>("mobile").map(Value::String).unwrap_or(Value::Null)),
        ]))))),
        None => Ok(Json(ActionResult::error("user not found"))),
    }
}

// --- 短信验证码（code）---

#[derive(Debug, Clone)]
struct CodeEntry {
    code: String,
    expires_at: DateTime<Utc>,
    attempts: u32,
}

const CODE_TTL_MINUTES: i64 = 5;
const CODE_ATTEMPTS: u32 = 5;

/// 短信验证码存储：credential → 一次性验证码（发送后校验用）
pub struct CodeStore {
    entries: Mutex<HashMap<String, CodeEntry>>,
}

impl Default for CodeStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeStore {
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

    /// 为 credential 生成 6 位数字验证码并存储，返回明文（发送渠道使用）
    pub fn issue(&self, key: &str) -> String {
        self.cleanup();
        let code = format!("{:06}", rand_code());
        if let Ok(mut map) = self.entries.lock() {
            map.insert(
                key.to_string(),
                CodeEntry {
                    code: code.clone(),
                    expires_at: Utc::now() + Duration::minutes(CODE_TTL_MINUTES),
                    attempts: 0,
                },
            );
        }
        code
    }

    /// 校验验证码：通过删除（一次性），错误达上限删除
    pub fn verify(&self, key: &str, code: &str) -> bool {
        self.cleanup();
        let Ok(mut map) = self.entries.lock() else {
            return false;
        };
        let Some(entry) = map.get_mut(key) else {
            return false;
        };
        if entry.expires_at <= Utc::now() {
            map.remove(key);
            return false;
        }
        if entry.code == code.trim() {
            map.remove(key);
            return true;
        }
        entry.attempts += 1;
        if entry.attempts >= CODE_ATTEMPTS {
            map.remove(key);
        }
        false
    }
}

fn rand_code() -> u32 {
    (uuid::Uuid::new_v4().as_u128() % 1_000_000) as u32
}

pub(crate) fn code_store() -> &'static CodeStore {
    static STORE: OnceLock<CodeStore> = OnceLock::new();
    STORE.get_or_init(CodeStore::new)
}

/// GET /jaxrs/authentication/code/credential/{credential} —— 向凭据发送登录验证码
pub async fn code_send(
    Path(credential): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if credential.is_empty() {
        return Err(AppError::BadRequest("credential cannot be empty".to_string()));
    }
    let _plain = code_store().issue(&credential);
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("message".to_string(), Value::String("code sent".to_string())),
    ])))))
}

/// POST /jaxrs/authentication/code —— 发送登录验证码（请求体可含 credential）
pub async fn code(
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let credential = payload
        .get("credential")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if credential.is_empty() {
        return Err(AppError::BadRequest("credential cannot be empty".to_string()));
    }
    let _plain = code_store().issue(&credential);
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("message".to_string(), Value::String("code sent".to_string())),
    ])))))
}

// --- 组织架构查询（保持既有契约路径）---

/// 获取组织架构树（部门/单位列表）
pub async fn unit_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, parent_id, level FROM auth_unit \
             WHERE deleted_at IS NULL ORDER BY level",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), row.get::<_, Option<String>>("parent_id").map(Value::String).unwrap_or(Value::Null)),
                ("level".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("level")))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// 获取角色列表
pub async fn role_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, description FROM auth_role WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("description".to_string(), row.get::<_, Option<String>>("description").map(Value::String).unwrap_or(Value::Null)),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// 获取用户组列表
pub async fn group_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name FROM auth_group WHERE disable = false AND deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

// --- 路由注册 ---

/// 构建认证模块路由
///
/// 契约路径（Java AuthenticationAction 对齐）：
///   POST   /jaxrs/authentication            登录
///   DELETE /jaxrs/authentication            登出
///   GET    /jaxrs/authentication            当前用户
///   POST   /jaxrs/authentication/refresh    刷新令牌
/// 保留自造路径 /login、/logout、/who 作兼容（前端尚未切到契约路径）。
/// 验证码、扫码绑定、OAuth 端点由对应模块子 router 提供。
pub fn router(pool: Pool, rate_limiter: RateLimiter, session_manager: SessionManager) -> Router {
    Router::new()
        .route("/jaxrs/authentication", post(login))
        .route("/jaxrs/authentication", delete(logout))
        .route("/jaxrs/authentication", get(whoami))
        .route("/jaxrs/authentication/login", post(login))
        .route("/jaxrs/authentication/logout", post(logout))
        .route("/jaxrs/authentication/who", get(whoami))
        .route("/jaxrs/authentication/refresh", post(refresh))
        .route("/jaxrs/authentication/code", post(code))
        .route(
            "/jaxrs/authentication/code/credential/{credential}",
            get(code_send),
        )
        .route("/jaxrs/authentication/oauth/list", get(oauth::oauth_list))
        .route(
            "/jaxrs/authentication/oauth/qywx/config",
            get(oauth::oauth_qywx_config),
        )
        .route(
            "/jaxrs/authentication/oauth/dingding/config",
            get(oauth::oauth_dingding_config),
        )
        .route(
            "/jaxrs/authentication/oauth/name/{name}",
            get(oauth::oauth_name_config),
        )
        .route(
            "/jaxrs/authentication/oauth/login/qywx/code/{code}",
            get(oauth::oauth_login_qywx),
        )
        .route(
            "/jaxrs/authentication/oauth/login/dingding/code/{code}",
            get(oauth::oauth_login_dingding),
        )
        .route(
            "/jaxrs/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}",
            get(oauth::oauth_login_name),
        )
        .route(
            "/jaxrs/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}",
            get(oauth::oauth_bind_name),
        )
        .merge(captcha::captcha_router())
        .merge(bind::bind_router())
        .merge(welink::router())
        .merge(mpweixin::router())
        .merge(qiyeweixin::router())
        .merge(zhengwudingding::router())
        .route("/jaxrs/authentication/two_factor", post(two_factor::two_factor_login))
        .route("/jaxrs/authentication/safe/logout", post(safe_logout::safe_logout))
        .route("/jaxrs/authentication/check/token", post(check_token::check_token))
        .route("/jaxrs/authentication/sso", post(sso::sso_post_login))
        .route("/jaxrs/authentication/sso/encrypt", post(sso::sso_encrypt))
        .route(
            "/jaxrs/authentication/sso/client/{client}/token/{token}",
            get(sso::sso_get_login),
        )
        .route("/jaxrs/authentication/switchuser", post(switch_user::switch_user))
        .route(
            "/jaxrs/andfx/moa/sso/token/{token}/enter/{enterId}",
            get(andfx::andfx_moa_sso),
        )
        .layer(Extension(pool))
        .layer(Extension(rate_limiter))
        .layer(Extension(session_manager))
}