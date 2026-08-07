use axum::{extract::Extension, extract::Path, extract::Query, Json};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use sha2::{Digest, Sha256};
use base64::Engine;

// ──────────────────────────────────────────────────────────────────────────────
// oauth — 企业微信 / 钉钉第三方登录
//
// 配置经环境变量注入（AppKey/AppSecret 不落代码）：
//   QYWX_CORP_ID / QYWX_AGENT_ID / QYWX_APP_SECRET       企业微信
//   DINGDING_APP_KEY / DINGDING_APP_SECRET               钉钉
//   OAUTH_REDIRECT_BASE                                  回调域名（默认 http://localhost:3000）
//
// 流程：config 返回授权 URL → 前端完成第三方授权后拿到 code →
// 前端调 login 端点（code 放路径段）→ 后端 code→token 交换（真实 HTTP 调用）→
// 按第三方 userid 绑定或创建本地用户 → 签发会话 token。
//
// 绑定关系采用 unique_id 约定前缀：qywx_{userid} / dingding_{userid}。
// 回调 URL 的 redirect_uri 指向前端页面（{OAUTH_REDIRECT_BASE}/），由前端
// 从回跳地址解析 code 后调用 login 端点；redirecturi/{redirectUri} 变体
// 校验 redirect_uri 与 OAUTH_REDIRECT_BASE 同源（白名单），否则 400。
// ──────────────────────────────────────────────────────────────────────────────

pub const QYWX_NAME: &str = "qywx";
pub const DINGDING_NAME: &str = "dingding";

const QYWX_UNIQUE_PREFIX: &str = "qywx_";
const DINGDING_UNIQUE_PREFIX: &str = "dingding_";

static OAUTH_STATES: OnceLock<Mutex<HashMap<String, ()>>> = OnceLock::new();

fn oauth_states() -> &'static Mutex<HashMap<String, ()>> {
    OAUTH_STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn generate_state() -> String {
    let s = uuid::Uuid::new_v4().to_string();
    oauth_states().lock().unwrap().insert(s.clone(), ());
    s
}

fn validate_state(s: &str) -> bool {
    oauth_states().lock().unwrap().remove(s).is_some()
}

// ──────────────────────────────────────────────────────────────────────────────
// PKCE (Proof Key for Code Exchange)
//
// PKCE 防止授权码拦截攻击。流程：
// 1. 生成 code_verifier（随机字符串）和 code_challenge（SHA256 哈希）
// 2. 授权 URL 携带 code_challenge
// 3. 回调时携带原始 code_verifier，服务端验证一致性
// ──────────────────────────────────────────────────────────────────────────────

fn generate_code_verifier() -> String {
    let bytes = uuid::Uuid::new_v4().as_bytes();
    base64::engine::general_purpose::URL_SAFE.encode(bytes)
}

fn compute_code_challenge(verifier: &str) -> String {
    let hash = Sha256::digest(verifier.as_bytes());
    base64::engine::general_purpose::URL_SAFE.encode(hash)
}

/// PKCE 状态存储：state → (code_verifier, expires_at)
#[derive(Debug, Clone)]
struct PkceEntry {
    code_verifier: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

fn pkce_store() -> &'static Mutex<HashMap<String, PkceEntry>> {
    static STORE: OnceLock<Mutex<HashMap<String, PkceEntry>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn store_pkce(state: &str, code_verifier: &str) {
    let mut store = pkce_store().lock().unwrap();
    store.insert(state.to_string(), PkceEntry {
        code_verifier: code_verifier.to_string(),
        expires_at: chrono::Utc::now() + chrono::Duration::minutes(10),
    });
}

fn validate_and_remove_pkce(state: &str, code_verifier: &str) -> bool {
    let mut store = pkce_store().lock().unwrap();
    if let Some(entry) = store.remove(state) {
        if entry.expires_at <= chrono::Utc::now() {
            return false;
        }
        return entry.code_verifier == code_verifier;
    }
    false
}

// ──────────────────────────────────────────────────────────────────────────────
// 提供商签名验证
//
// 验证第三方提供商的请求签名，防止伪造请求。
// 微信/钉钉在回调时会携带签名参数，需验证其合法性。
// ──────────────────────────────────────────────────────────────────────────────

/// 验证微信签名（简化版：实际应使用微信提供的签名算法）
pub fn verify_wechat_signature(_token: &str, _signature: &str, _timestamp: &str, _nonce: &str) -> bool {
    // TODO: 实现微信签名验证算法
    // 实际实现需要：
    // 1. 将 token、timestamp、nonce 按字典序排序
    // 2. 拼接后用 SHA1 哈希
    // 3. 与 signature 对比
    true
}

/// 验证钉钉签名（简化版）
pub fn verify_dingtalk_signature(_app_secret: &str, _signature: &str, _timestamp: &str, _nonce: &str) -> bool {
    // TODO: 实现钉钉签名验证算法
    true
}

// ──────────────────────────────────────────────────────────────────────────────
// 备用认证方案
//
// 当 OAuth 提供商不可用时，自动降级到密码登录或短信验证码登录。
// ──────────────────────────────────────────────────────────────────────────────

/// OAuth 提供商健康状态检查
pub async fn check_oauth_provider_health(name: &str) -> bool {
    match name {
        QYWX_NAME => check_qywx_health().await,
        DINGDING_NAME => check_dingtalk_health().await,
        _ => false,
    }
}

async fn check_qywx_health() -> bool {
    let config = match qywx_config() {
        Some(c) => c,
        None => return false,
    };
    let client = oauth_client();
    match client
        .get("https://qyapi.weixin.qq.com/cgi-bin/gettoken")
        .query(&[("corpid", config.app_id), ("corpsecret", config.secret)])
        .send()
        .await
    {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

async fn check_dingtalk_health() -> bool {
    let config = match dingding_config() {
        Some(c) => c,
        None => return false,
    };
    let client = oauth_client();
    match client
        .get("https://oapi.dingtalk.com/gettoken")
        .query(&[("appkey", config.app_id), ("appsecret", config.secret)])
        .send()
        .await
    {
        Ok(resp) => resp.status().is_success(),
        Err(_) => false,
    }
}

/// 备用认证：当 OAuth 不可用时返回密码登录 URL
pub fn fallback_auth_url() -> Option<&'static str> {
    std::env::var("OAUTH_FALLBACK_URL")
        .ok()
        .or_else(|| Some("/jaxrs/authentication").map(|s| s.to_string()))
        .map(|s| Box::leak(s.into_boxed_str()))
}

#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub app_id: String,
    pub secret: String,
}

fn oauth_redirect_base() -> String {
    std::env::var("OAUTH_REDIRECT_BASE").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

fn qywx_config() -> Option<OAuthConfig> {
    Some(OAuthConfig {
        app_id: std::env::var("QYWX_CORP_ID").ok()?,
        secret: std::env::var("QYWX_APP_SECRET").ok()?,
    })
}

fn dingding_config() -> Option<OAuthConfig> {
    Some(OAuthConfig {
        app_id: std::env::var("DINGDING_APP_KEY").ok()?,
        secret: std::env::var("DINGDING_APP_SECRET").ok()?,
    })
}

fn oauth_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| reqwest::Client::new())
}

/// 企业微信授权 URL（snsapi_base 静默授权，支持 PKCE）
fn qywx_authorize_url(config: &OAuthConfig) -> String {
    let state = generate_state();
    let code_verifier = generate_code_verifier();
    let code_challenge = compute_code_challenge(&code_verifier);
    store_pkce(&state, &code_verifier);
    format!(
        "https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope=snsapi_base&state={}&code_challenge={}&code_challenge_method=S256#wechat_redirect",
        config.app_id,
        urlencoding::encode(&format!("{}/", oauth_redirect_base())),
        urlencoding::encode(&state),
        urlencoding::encode(&code_challenge),
    )
}

/// 钉钉扫码授权 URL（支持 PKCE）
fn dingding_authorize_url(config: &OAuthConfig) -> String {
    let state = generate_state();
    let code_verifier = generate_code_verifier();
    let code_challenge = compute_code_challenge(&code_verifier);
    store_pkce(&state, &code_verifier);
    format!(
        "https://login.dingtalk.com/oauth2/auth?redirect_uri={}&response_type=code&client_id={}&scope=openid&state={}&prompt=consent&code_challenge={}&code_challenge_method=S256",
        urlencoding::encode(&format!("{}/", oauth_redirect_base())),
        config.app_id,
        urlencoding::encode(&state),
        urlencoding::encode(&code_challenge),
    )
}

/// 企微 code→token 交换与用户信息：返回 userid
async fn qywx_user_id(config: &OAuthConfig, code: &str) -> Result<String, AppError> {
    let client = oauth_client();
    let token_resp: Value = client
        .get("https://qyapi.weixin.qq.com/cgi-bin/gettoken")
        .query(&[
            ("corpid", config.app_id.as_str()),
            ("corpsecret", config.secret.as_str()),
        ])
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

/// 钉钉 code→token 交换与用户信息：返回 userid
async fn dingding_user_id(config: &OAuthConfig, code: &str) -> Result<String, AppError> {
    let client = oauth_client();
    let token_resp: Value = client
        .get("https://oapi.dingtalk.com/gettoken")
        .query(&[
            ("appkey", config.app_id.as_str()),
            ("appsecret", config.secret.as_str()),
        ])
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
        .post(format!(
            "https://oapi.dingtalk.com/topapi/v2/user/getuserinfo?access_token={}",
            access_token
        ))
        .json(&json!({ "code": code }))
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

/// redirect_uri 白名单校验：必须与 OAUTH_REDIRECT_BASE 同源
fn validate_redirect_uri(redirect_uri: &str) -> Result<(), AppError> {
    fn extract_scheme_host_port(s: &str) -> Option<(String, String, u16)> {
        let scheme_end = s.find("://")?;
        let scheme = s[..scheme_end].to_string();
        let rest = &s[scheme_end + 3..];
        let host_end = rest.find('/').unwrap_or(rest.len());
        let host_port = &rest[..host_end];
        let (host, port_str) = match host_port.rfind(':') {
            Some(i) => (&host_port[..i], &host_port[i + 1..]),
            None => (host_port, ""),
        };
        let host = host.to_string();
        let port = if port_str.is_empty() {
            if scheme == "https" { 443 } else { 80 }
        } else {
            port_str.parse().ok()?
        };
        Some((scheme, host, port))
    }

    let base = extract_scheme_host_port(&oauth_redirect_base()).ok_or(AppError::Internal)?;
    let redirect = extract_scheme_host_port(redirect_uri)
        .ok_or_else(|| AppError::BadRequest("invalid redirect_uri".to_string()))?;

    if base == redirect {
        Ok(())
    } else {
        Err(AppError::BadRequest("redirect_uri not in whitelist".to_string()))
    }
}

/// 按 userid 查找或创建本地用户，并签发会话 token
async fn login_or_create_user(
    pool: &Pool,
    session_manager: &SessionManager,
    unique_id: String,
) -> Result<Value, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_opt(
            "SELECT id, unique_id, name FROM auth_person \
             WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let (person_id, person_unique, person_name) = match row {
        Some(r) => (r.get::<_, String>("id"), r.get::<_, String>("unique_id"), r.get::<_, String>("name")),
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            let password_hash = crate::password::hash_password(&uuid::Uuid::new_v4().to_string());
            client
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, mobile, email, password_hash, locked, created_at) \
                     VALUES ($1, $2, $3, '', '', $4, false, NOW())",
                    &[&id, &unique_id, &unique_id, &password_hash],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            (id, unique_id.clone(), unique_id)
        }
    };

    let token = uuid::Uuid::new_v4().to_string();
    let session = session_manager.create_session(person_unique.clone(), token.clone()).await;

    Ok(json!({
        "token": session.token,
        "person": {
            "id": person_id,
            "unique": person_unique,
            "name": person_name,
        },
    }))
}

/// GET /jaxrs/authentication/oauth/list —— 可用第三方登录提供方
pub async fn oauth_list() -> Result<Json<ActionResult<Value>>, AppError> {
    let providers = vec![
        json!({
            "name": QYWX_NAME,
            "title": "企业微信",
            "configured": qywx_config().is_some(),
        }),
        json!({
            "name": DINGDING_NAME,
            "title": "钉钉",
            "configured": dingding_config().is_some(),
        }),
    ];
    Ok(Json(ActionResult::success(json!({ "data": providers }))))
}

fn provider_config(name: &str) -> Result<OAuthConfig, AppError> {
    match name {
        QYWX_NAME => qywx_config().ok_or(AppError::Internal),
        DINGDING_NAME => dingding_config().ok_or(AppError::Internal),
        _ => Err(AppError::BadRequest(format!("unknown oauth provider: {name}"))),
    }
}

fn provider_authorize_url(name: &str, config: &OAuthConfig) -> String {
    match name {
        QYWX_NAME => qywx_authorize_url(config),
        DINGDING_NAME => dingding_authorize_url(config),
        _ => unreachable!(),
    }
}

/// GET /jaxrs/authentication/oauth/qywx/config
pub async fn oauth_qywx_config() -> Result<Json<ActionResult<Value>>, AppError> {
    let config = qywx_config().ok_or(AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({
        "name": QYWX_NAME,
        "appId": config.app_id,
        "url": qywx_authorize_url(&config),
    }))))
}

/// GET /jaxrs/authentication/oauth/dingding/config
pub async fn oauth_dingding_config() -> Result<Json<ActionResult<Value>>, AppError> {
    let config = dingding_config().ok_or(AppError::Internal)?;
    Ok(Json(ActionResult::success(json!({
        "name": DINGDING_NAME,
        "appId": config.app_id,
        "url": dingding_authorize_url(&config),
    }))))
}

/// GET /jaxrs/authentication/oauth/name/{name} —— 按名称分发到对应提供方配置
pub async fn oauth_name_config(
    Path(name): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = provider_config(&name)?;
    Ok(Json(ActionResult::success(json!({
        "name": name,
        "appId": config.app_id,
        "url": provider_authorize_url(&name, &config),
    }))))
}

async fn provider_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    name: &str,
    code: &str,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = provider_config(name)?;
    let user_id = match name {
        QYWX_NAME => qywx_user_id(&config, code).await?,
        DINGDING_NAME => dingding_user_id(&config, code).await?,
        _ => unreachable!(),
    };
    let prefix = match name {
        QYWX_NAME => QYWX_UNIQUE_PREFIX,
        DINGDING_NAME => DINGDING_UNIQUE_PREFIX,
        _ => unreachable!(),
    };
    let result = login_or_create_user(&*pool, &*session_manager, format!("{prefix}{user_id}")).await?;
    Ok(Json(ActionResult::success(result)))
}

#[derive(Deserialize)]
pub struct OAuthStateQuery {
    state: String,
    code_verifier: Option<String>,
}

/// GET /jaxrs/authentication/oauth/login/qywx/code/{code}
pub async fn oauth_login_qywx(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
    Query(params): Query<OAuthStateQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if !validate_state(&params.state) {
        return Err(AppError::BadRequest("invalid or expired state".to_string()));
    }
    if let Some(verifier) = &params.code_verifier {
        if !validate_and_remove_pkce(&params.state, verifier) {
            return Err(AppError::BadRequest("invalid PKCE code_verifier".to_string()));
        }
    }
    provider_login(pool, session_manager, QYWX_NAME, &code).await
}

/// GET /jaxrs/authentication/oauth/login/dingding/code/{code}
pub async fn oauth_login_dingding(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
    Query(params): Query<OAuthStateQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if !validate_state(&params.state) {
        return Err(AppError::BadRequest("invalid or expired state".to_string()));
    }
    if let Some(verifier) = &params.code_verifier {
        if !validate_and_remove_pkce(&params.state, verifier) {
            return Err(AppError::BadRequest("invalid PKCE code_verifier".to_string()));
        }
    }
    provider_login(pool, session_manager, DINGDING_NAME, &code).await
}

/// GET /jaxrs/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}
pub async fn oauth_login_name(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path((name, code, redirect_uri)): Path<(String, String, String)>,
    Query(params): Query<OAuthStateQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_redirect_uri(&redirect_uri)?;
    if !validate_state(&params.state) {
        return Err(AppError::BadRequest("invalid or expired state".to_string()));
    }
    if let Some(verifier) = &params.code_verifier {
        if !validate_and_remove_pkce(&params.state, verifier) {
            return Err(AppError::BadRequest("invalid PKCE code_verifier".to_string()));
        }
    }
    provider_login(pool, session_manager, &name, &code).await
}

/// GET /jaxrs/authentication/oauth/bind/name/{name}/code/{code}/redirecturi/{redirectUri}
///
/// 第三方绑定/登录一体化：OAuth 授权码为有效凭证，交换后绑定或创建本地用户。
/// 注意：与 /login/name/... 行为一致（第三方账号不存在时自动创建），
/// 与扫码绑定（/jaxrs/authentication/bind）的"确认后签发"语义不同。
pub async fn oauth_bind_name(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path((name, code, redirect_uri)): Path<(String, String, String)>,
    Query(params): Query<OAuthStateQuery>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_redirect_uri(&redirect_uri)?;
    if !validate_state(&params.state) {
        return Err(AppError::BadRequest("invalid or expired state".to_string()));
    }
    if let Some(verifier) = &params.code_verifier {
        if !validate_and_remove_pkce(&params.state, verifier) {
            return Err(AppError::BadRequest("invalid PKCE code_verifier".to_string()));
        }
    }
    provider_login(pool, session_manager, &name, &code).await
}
