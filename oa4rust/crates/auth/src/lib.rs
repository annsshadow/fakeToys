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
use shared::{db::dialect, error::AppError, response::ActionResult};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

mod ldap_auth;
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
pub mod sms;
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
#[cfg(test)]
mod tests_generated;


// --- 请求/响应 DTO ---

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub credential: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub token_type: String,
    pub role_list: Vec<String>,
    pub password_expired: bool,
    pub identity_list: Vec<String>,
    pub person: PersonInfo,
}

#[derive(Debug, Serialize, Clone)]
pub struct PersonInfo {
    pub id: String,
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub icon: Option<String>,
    pub job: Option<String>,
    pub department: Option<String>,
    pub unit: Option<String>,
    pub position: Option<String>,
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

    let d = dialect();
    let sql = format!(
        "SELECT id, unique_id, name, mobile, email, icon, job, department, unit, position, \
         password_hash, locked, {}, {} FROM auth_person \
         WHERE unique_id = {} AND deleted_at IS NULL",
        d.cast_text("change_password_time"),
        d.cast_text("password_expired_time"),
        d.param(1),
    );
    let row = client
        .query_one(&sql, &[&req.credential])
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let person_id: String = row.get("id");
    let person_unique: String = row.get("unique_id");
    let person_name: String = row.get("name");
    let person_mobile: Option<String> = row.get("mobile");
    let person_email: Option<String> = row.get("email");
    let person_icon: Option<String> = row.get("icon");
    let person_job: Option<String> = row.get("job");
    let person_department: Option<String> = row.get("department");
    let person_unit: Option<String> = row.get("unit");
    let person_position: Option<String> = row.get("position");
    let password_hash: String = row.get("password_hash");
    let locked: bool = row.get("locked");
    let change_password_time: Option<String> = row.get("change_password_time");
    let password_expired_time: Option<String> = row.get("password_expired_time");

    // 检查账户是否被锁定
    if locked {
        // 返回通用错误消息，防止账户锁定状态枚举
        return Ok(Json(ActionResult::error("invalid credentials")));
    }


    // LDAP 认证优先（若启用）：成功后跳过数据库密码校验
    let ldap_result = match ldap_auth::try_ldap_authenticate(&req.credential, &req.password).await {
        Ok(r) => r,
        Err(e) => return Err(e), // LDAP 连接错误：不静默回退
    };
    let valid = match ldap_result {
        Some(ldap_auth::LdapAuthOutcome::Success) => true, // LDAP 认证成功
        Some(ldap_auth::LdapAuthOutcome::Failed) => {
            // LDAP 凭据无效，回退到数据库密码校验
            password::verify_password(&req.password, &password_hash, "", None)
        }
        None => {
            // LDAP 未启用（Disabled 或 from_env 返回 None），走数据库密码校验
            password::verify_password(&req.password, &password_hash, "", None)
        }
        // Error 和 Disabled 已通过 Err/None 路径处理，此处为防御性分支
        Some(_) => password::verify_password(&req.password, &password_hash, "", None),
    };
    if !valid {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    // 密码哈希 rehash：检测旧算法（MD5/DES），自动升级为 bcrypt
    if password::needs_rehash(&password_hash) {
        let new_hash = password::rehash_password(&req.password);
        let _ = client
            .execute(
                "UPDATE auth_person SET password_hash = $1, updated_at = NOW() WHERE id = $2",
                &[&new_hash, &person_id],
            )
            .await;
    }

    // 检查密码是否过期（简化实现：如果 change_password_time 为 NULL 则密码过期）
    let password_expired = match change_password_time {
        None => true,
        Some(_) => false,
    };

    // 查询用户角色列表
    let role_list: Vec<String> = {
        let role_rows = client
            .query(
                "SELECT r.name FROM auth_role r \
                 JOIN auth_person_role pr ON r.id = pr.role_id \
                 WHERE pr.person_id = $1 AND r.deleted_at IS NULL",
                &[&person_id],
            )
            .await
            .unwrap_or_default();
        role_rows.iter().map(|r| r.get::<_, String>("name")).collect()
    };

    // 查询用户身份列表
    let identity_list: Vec<String> = {
        let identity_rows = client
            .query(
                "SELECT i.name FROM auth_identity i \
                 JOIN auth_person_identity pi ON i.id = pi.identity_id \
                 WHERE pi.person_id = $1 AND i.deleted_at IS NULL",
                &[&person_id],
            )
            .await
            .unwrap_or_default();
        identity_rows.iter().map(|r| r.get::<_, String>("name")).collect()
    };

    let token = Uuid::new_v4().to_string();
    let session = session_manager.create_session(person_unique.clone(), token.clone()).await;

    let response = ActionResult::success(LoginResponse {
        token: session.token,
        token_type: "Bearer".to_string(),
        role_list,
        password_expired,
        identity_list,
        person: PersonInfo {
            id: person_id,
            unique: person_unique,
            name: person_name,
            mobile: person_mobile,
            email: person_email,
            icon: person_icon,
            job: person_job,
            department: person_department,
            unit: person_unit,
            position: person_position,
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
            &dialect().format_sql(
                "SELECT id, unique_id, name, mobile FROM auth_person \
                 WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            ),
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

// --- 临时 Token 存储（双因素登录阶段绑定）---

#[derive(Clone)]
struct TempTokenEntry {
    credential: String,
    expires_at: DateTime<Utc>,
}

const TEMP_TOKEN_TTL_MINUTES: i64 = 5;

pub struct TempTokenStore {
    entries: Mutex<HashMap<String, TempTokenEntry>>,
}

impl Default for TempTokenStore {
    fn default() -> Self {
        Self::new()
    }
}

impl TempTokenStore {
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

    pub fn issue(&self, credential: &str) -> String {
        self.cleanup();
        let token = Uuid::new_v4().to_string();
        if let Ok(mut map) = self.entries.lock() {
            map.insert(
                token.clone(),
                TempTokenEntry {
                    credential: credential.to_string(),
                    expires_at: Utc::now() + Duration::minutes(TEMP_TOKEN_TTL_MINUTES),
                },
            );
        }
        token
    }

    pub fn verify(&self, token: &str) -> Option<String> {
        self.cleanup();
        let Ok(mut map) = self.entries.lock() else {
            return None;
        };
        let entry = map.remove(token)?;
        if entry.expires_at <= Utc::now() {
            return None;
        }
        Some(entry.credential)
    }
}

pub(crate) fn temp_token_store() -> &'static TempTokenStore {
    static STORE: OnceLock<TempTokenStore> = OnceLock::new();
    STORE.get_or_init(TempTokenStore::new)
}

/// GET /jaxrs/authentication/code/credential/{credential} —— 向凭据发送登录验证码
pub async fn code_send(
    pool: Extension<Pool>,
    Path(credential): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if credential.is_empty() {
        return Err(AppError::BadRequest("credential cannot be empty".to_string()));
    }
    // 检查凭证是否存在（防止对任意凭据发送验证码）
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let exists = client
        .query_opt(
            "SELECT 1 FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&credential],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    if exists.is_none() {
        // 返回通用成功消息，防止凭据枚举
        return Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
            ("message".to_string(), Value::String("code sent".to_string())),
        ])))))
    }
    let _plain = code_store().issue(&credential);
    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("message".to_string(), Value::String("code sent".to_string())),
    ])))))
}

/// POST /jaxrs/authentication/code —— 双因素登录第二阶段
///
/// 验证 credential + codeAnswer + temp_token，签发完整会话
pub async fn code(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Json(payload): Json<Value>,
) -> Result<Json<ActionResult<TwoFactorLoginResponse>>, AppError> {
    let credential = payload
        .get("credential")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let code_answer = payload
        .get("codeAnswer")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let temp_token = payload
        .get("tempToken")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if credential.is_empty() || code_answer.is_empty() || temp_token.is_empty() {
        return Ok(Json(ActionResult::error("invalid request")));
    }

    let bound_credential = temp_token_store().verify(&temp_token);
    let bound_credential = match bound_credential {
        Some(cred) => cred,
        None => return Ok(Json(ActionResult::error("invalid or expired session"))),
    };

    if bound_credential != credential {
        return Ok(Json(ActionResult::error("credential mismatch")));
    }

    let code_valid = code_store().verify(&credential, &code_answer);
    if !code_valid {
        return Ok(Json(ActionResult::error("invalid code")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon, job, department, unit, position \
             FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&credential],
        )
        .await
        .map_err(|_| AppError::Unauthorized)?;

    let person_id: String = row.get("id");
    let person_unique: String = row.get("unique_id");
    let person_name: String = row.get("name");
    let person_mobile: Option<String> = row.get("mobile");
    let person_email: Option<String> = row.get("email");
    let person_icon: Option<String> = row.get("icon");
    let person_job: Option<String> = row.get("job");
    let person_department: Option<String> = row.get("department");
    let person_unit: Option<String> = row.get("unit");
    let person_position: Option<String> = row.get("position");

    let token = Uuid::new_v4().to_string();
    let session = session_manager.create_session(person_unique.clone(), token.clone()).await;

    Ok(Json(ActionResult::success(TwoFactorLoginResponse {
        token: session.token,
        person: PersonInfo {
            id: person_id,
            unique: person_unique,
            name: person_name,
            mobile: person_mobile,
            email: person_email,
            icon: person_icon,
            job: person_job,
            department: person_department,
            unit: person_unit,
            position: person_position,
        },
    })))
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

// --- 验证码 + 短信集成函数 ---

/// 生成验证码（使用 auth::captcha 模块），返回 (captcha_id, data_uri)
pub async fn captcha_generate() -> Result<(String, String), AppError> {
    let Json(result) = crate::captcha::captcha_default().await?;
    let data = result.data.ok_or(AppError::Internal)?;
    let id = data
        .get("captchaId")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?
        .to_string();
    let image = data
        .get("image")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?
        .to_string();
    Ok((id, image))
}

/// 校验验证码（使用 auth::captcha 模块）
pub async fn captcha_verify(captcha_id: &str, answer: &str) -> Result<bool, AppError> {
    use crate::captcha::VerifyResult;
    match crate::captcha::captcha_store().verify(captcha_id, answer) {
        VerifyResult::Ok => Ok(true),
        VerifyResult::TooManyAttempts => {
            Err(AppError::BadRequest("too many attempts".to_string()))
        }
        VerifyResult::Expired => Err(AppError::BadRequest("captcha expired".to_string())),
        _ => Err(AppError::BadRequest("invalid captcha".to_string())),
    }
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
        .merge(sms::sms_router())
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
        .route("/jaxrs/authentication/unit/list", get(unit_list))
        .route("/jaxrs/authentication/role/list", get(role_list))
        .route("/jaxrs/authentication/group/list", get(group_list))
        .route(
            "/jaxrs/andfx/moa/sso/token/{token}/enter/{enterId}",
            get(andfx::andfx_moa_sso),
        )
        .layer(Extension(pool))
        .layer(Extension(rate_limiter))
        .layer(Extension(session_manager))
}