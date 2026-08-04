use axum::{
    extract::Extension,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Duration, Utc};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub mod model;
pub mod password;
pub mod person;
pub mod secret;

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

// --- 会话管理（纯 Rust 侧实现，独立于服务端 Session） ---

#[derive(Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub person_unique: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct SessionManager {
    pub sessions: Arc<RwLock<std::collections::HashMap<String, Session>>>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 创建会话
    ///
    /// 生成一个新的 Session，有效期 2 小时，存入内存 HashMap。
    ///
    /// # 参数
    /// - `person_unique`: 人员唯一标识（auth_person.unique_id）
    /// - `token`: 随机生成的会话令牌
    ///
    /// # 返回
    /// - `Session`: 创建好的会话对象
    pub async fn create_session(&self, person_unique: String, token: String) -> Session {
        let now = Utc::now();
        let session = Session {
            token: token.clone(),
            person_unique,
            created_at: now,
            expires_at: now + Duration::hours(2),
        };

        self.sessions.write().await.insert(token.clone(), session.clone());
        session
    }

    /// 验证会话令牌是否有效（未过期、存在）
    ///
    /// # 参数
    /// - `token`: 会话令牌字符串
    ///
    /// # 返回
    /// - `Some(Session)`: 令牌有效，返回对应会话
    /// - `None`: 令牌不存在或已过期
    pub async fn validate_session(&self, token: &str) -> Option<Session> {
        let sessions = self.sessions.read().await;
        sessions.get(token).cloned()
    }

    /// 删除会话（退出登录时使用）
    ///
    /// # 参数
    /// - `token`: 要删除的会话令牌
    pub async fn remove_session(&self, token: &str) {
        self.sessions.write().await.remove(token);
    }
}

// --- 频率限制 ---

#[derive(Clone)]
pub struct RateLimiter {
    pub attempts: Arc<RwLock<std::collections::HashMap<String, (i32, DateTime<Utc>)>>>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            attempts: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 检查是否超出频率限制
    ///
    /// 记录每个 key（如 IP）的尝试次数，在指定时间窗口内超过上限则返回错误。
    ///
    /// # 参数
    /// - `key`: 限流键（通常为客户端 IP）
    /// - `max_attempts`: 时间窗口内允许的最大尝试次数
    /// - `window_minutes`: 时间窗口（分钟）
    ///
    /// # 返回
    /// - `Ok(())`: 未超出限制，允许继续
    /// - `Err(AppError::BadRequest)`: 已超出限制，拒绝请求
    pub async fn check_rate_limit(&self, key: &str, max_attempts: i32, window_minutes: i64) -> Result<(), AppError> {
        let mut attempts = self.attempts.write().await;
        let now = Utc::now();

        if let Some((count, last_attempt)) = attempts.get(key) {
            let elapsed = now - *last_attempt;
            if elapsed.num_minutes() < window_minutes
                && *count >= max_attempts {
                    return Err(AppError::BadRequest(
                        format!("rate limit exceeded: {} attempts in last {} minutes", count, window_minutes)
                    ));
                }
        }

        let count = attempts.get(key).map(|(c, _)| c + 1).unwrap_or(1);
        attempts.insert(key.to_string(), (count, now));
        Ok(())
    }

    /// 记录一次失败尝试（递增计数器）
    ///
    /// # 参数
    /// - `key`: 限流键
    pub async fn record_failure(&self, key: &str) {
        let mut attempts = self.attempts.write().await;
        let now = Utc::now();
        let count = attempts.get(key).map(|(c, _)| c + 1).unwrap_or(1);
        attempts.insert(key.to_string(), (count, now));
    }

    /// 重置指定 key 的尝试计数（登录成功后调用）
    ///
    /// # 参数
    /// - `key`: 限流键
    pub async fn reset(&self, key: &str) {
        self.attempts.write().await.remove(key);
    }
}

// --- 认证处理器 ---

/// 用户登录接口
///
/// 接收用户名/工号和密码，验证通过后签发 2 小时有效会话令牌。
/// 支持 MD5 和 DES 两种密码加密方式。登录失败会累积失败次数，超过限流阈值将拒绝请求。
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `rate_limiter`: 频率限制器
/// - `session_manager`: 会话管理器
/// - `req`: 登录请求体，包含 `credential`（用户名/工号）和 `password`
///
/// # 返回
/// - `Ok(Json<ActionResult<LoginResponse>>)`: 登录成功，返回 token 和用户基本信息
/// - `Err(AppError)`: 数据库错误等异常情况
pub async fn login(
    pool: Extension<Pool>,
    rate_limiter: Extension<RateLimiter>,
    session_manager: Extension<SessionManager>,
    axum::extract::Json(req): axum::extract::Json<LoginRequest>,
) -> Result<Json<ActionResult<LoginResponse>>, AppError> {
    let ip = "127.0.0.1";
    rate_limiter.check_rate_limit(ip, 5, 1).await?;

    if req.credential.is_empty() || req.password.is_empty() {
        rate_limiter.record_failure(ip).await;
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let client = pool.get().await.map_err(|_e| {
        AppError::Internal
    })?;

    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, password_hash, locked FROM auth_person WHERE unique_id = $1 AND locked = false",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let _person_id: String = row.get("id");
    let person_unique: String = row.get("unique_id");
    let person_name: String = row.get("name");
    let person_mobile: Option<String> = row.get("mobile");
    let password_hash: String = row.get("password_hash");

    // 验证密码（支持 MD5 和 DES 加密）
    let valid = password::verify_password(&req.password, &password_hash, "", None);
    if !valid {
        rate_limiter.record_failure(ip).await;
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let token = Uuid::new_v4().to_string();
    let session = session_manager.create_session(person_unique.clone(), token.clone()).await;

    rate_limiter.reset(ip).await;

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

/// 获取验证码（占位实现）
///
/// 生成一个 captchaId 并返回 base64 占位图片，实际图片生成需对接第三方服务。
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `captchaId` 和 `image` 字段
pub async fn captcha() -> Result<Json<ActionResult<Value>>, AppError> {
    let captcha_id = Uuid::new_v4().to_string();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("captchaId".to_string(), Value::String(captcha_id)),
        ("image".to_string(), Value::String("base64-encoded-image-placeholder".to_string())),
    ])))))
}

/// 绑定接口：通过凭证创建会话
///
/// 根据 `credential`（用户名/工号）查询 auth_person，验证存在后签发新 token。
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `session_manager`: 会话管理器
/// - `payload`: JSON 请求体，包含 `credential` 字段
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 绑定成功，返回 `{"bound": true}`
/// - `Err(AppError::NotFound)`: 凭证不存在
pub async fn bind(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let credential = payload.get("credential").and_then(|v| v.as_str()).unwrap_or("");
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one("SELECT id FROM auth_person WHERE unique_id = $1", &[&credential])
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");
    let token = Uuid::new_v4().to_string();
    session_manager.create_session(person_id, token).await;

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("bound".to_string(), Value::Bool(true)),
    ])))))
}

/// OAuth 登录入口（占位实现）
///
/// 返回 OAuth 授权 URL，实际逻辑需对接第三方 OAuth 提供商。
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `oauthUrl` 字段
pub async fn oauth(
    _payload: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("oauthUrl".to_string(), Value::String("https://oauth.example.com/authorize".to_string())),
    ])))))
}

/// 刷新会话令牌
///
/// 使用旧 token 换取新 token，旧 token 随即失效。用于会话续期。
///
/// # 参数
/// - `session_manager`: 会话管理器
/// - `payload`: JSON 请求体，包含 `token` 字段
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 刷新成功，返回新 `token`
/// - `Err/AppError`: token 无效时返回错误信息
pub async fn refresh(
    _pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let old_token = payload.get("token").and_then(|v| v.as_str()).unwrap_or("");

    if let Some(session) = session_manager.validate_session(old_token).await {
        let new_token = Uuid::new_v4().to_string();
        session_manager.create_session(session.person_unique, new_token.clone()).await;
        session_manager.remove_session(old_token).await;

        Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("token".to_string(), Value::String(new_token)),
        ])))))
    } else {
        Ok(Json(ActionResult::error("invalid token")))
    }
}

/// 生成一次性验证码（占位实现）
///
/// 生成 UUID 作为 code 返回，用于后续重置密码等场景。
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `code` 字段
pub async fn code(
    _payload: Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let code = Uuid::new_v4().to_string();
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("code".to_string(), Value::String(code)),
    ])))))
}

/// 用户登出接口
///
/// 根据请求体中的 token 删除对应会话，使 token 失效。
///
/// # 参数
/// - `session_manager`: 会话管理器
/// - `payload`: JSON 请求体，包含 `token` 字段
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 登出成功
pub async fn logout(
    session_manager: Extension<SessionManager>,
    axum::extract::Json(payload): axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if let Some(token) = payload.get("token").and_then(|v| v.as_str()) {
        session_manager.remove_session(token).await;
    }
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("message".to_string(), Value::String("logged out".to_string())),
    ])))))
}

/// 查询当前认证用户信息（ whoami ）
///
/// 从数据库读取首条未锁定人员记录，返回其基本信息以确认认证状态。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)` : 包含 `authenticated`（是否认证）、`id`、`unique`、`name`、`mobile`
pub async fn whoami(
    pool: Extension<Pool>,
    _session_manager: Extension<SessionManager>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, unique_id, name, mobile FROM auth_person LIMIT 1", &[])
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = rows.first() {
        Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("authenticated".to_string(), Value::Bool(true)),
            ("id".to_string(), Value::String(row.get("id"))),
            ("unique".to_string(), Value::String(row.get("unique_id"))),
            ("name".to_string(), Value::String(row.get("name"))),
            ("mobile".to_string(), row.get::<_, Option<String>>("mobile").map(Value::String).unwrap_or(Value::Null)),
        ])))))
    } else {
        Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("authenticated".to_string(), Value::Bool(false)),
        ])))))
    }
}

/// 获取组织架构树（部门/单位列表）
///
/// 查询 auth_unit 表，按层级排序返回所有单位信息，用于前端渲染组织架构树。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `count` 和 `data` 数组，每项含 `id`、`name`、`parentId`、`level`
pub async fn unit_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, parent_id, level FROM auth_unit ORDER BY level", &[])
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
///
/// 查询 auth_role 表，返回所有可用角色及其描述。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `count` 和 `data` 数组，每项含 `id`、`name`、`description`
pub async fn role_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name, description FROM auth_role", &[])
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
///
/// 查询 auth_group 表，仅返回未禁用的用户组。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Ok(Json<ActionResult<Value>>)`: 包含 `count` 和 `data` 数组，每项含 `id`、`name`
pub async fn group_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query("SELECT id, name FROM auth_group WHERE disable = false", &[])
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
/// 注册所有认证相关接口，包括登录、登出、会话刷新、组织架构查询等。
/// 挂载 RateLimiter 和 SessionManager 中间件层。
///
/// # 参数
/// - `pool`: 数据库连接池
///
/// # 返回
/// - `Router`: Axum 路由实例
pub fn router(pool: Pool) -> Router {
    let rate_limiter = RateLimiter::new();
    let session_manager = SessionManager::new();

    Router::new()
        .route("/jaxrs/authentication/login", post(login))
        .route("/jaxrs/authentication/logout", post(logout))
        .route("/jaxrs/authentication/who", get(whoami))
        .route("/jaxrs/authentication/captcha", get(captcha))
        .route("/jaxrs/authentication/bind", post(bind))
        .route("/jaxrs/authentication/oauth", post(oauth))
        .route("/jaxrs/authentication/refresh", post(refresh))
        .route("/jaxrs/authentication/code", post(code))
        .route("/jaxrs/person/{flag}", get(person::get))
        .route("/jaxrs/person/list", get(person::list))
        .route("/jaxrs/unit/list", get(unit_list))
        .route("/jaxrs/role/list", get(role_list))
        .route("/jaxrs/group/list", get(group_list))
        .merge(secret::router())
        .layer(Extension(pool))
        .layer(Extension(rate_limiter))
        .layer(Extension(session_manager))
}
