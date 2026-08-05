use axum::{extract::Extension, extract::Path, Json};
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;

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

/// 企业微信授权 URL（snsapi_base 静默授权）
fn qywx_authorize_url(config: &OAuthConfig) -> String {
    format!(
        "https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope=snsapi_base&state={}#wechat_redirect",
        config.app_id,
        urlencoding::encode(&format!("{}/", oauth_redirect_base())),
        urlencoding::encode(&format!("qywx-login")),
    )
}

/// 钉钉扫码授权 URL
fn dingding_authorize_url(config: &OAuthConfig) -> String {
    format!(
        "https://login.dingtalk.com/oauth2/auth?redirect_uri={}&response_type=code&client_id={}&scope=openid&state={}&prompt=consent",
        urlencoding::encode(&format!("{}/", oauth_redirect_base())),
        config.app_id,
        urlencoding::encode("dingding-login"),
    )
}

/// 企微 code→token 交换与用户信息：返回 userid
async fn qywx_user_id(config: &OAuthConfig, code: &str) -> Result<String, AppError> {
    let client = reqwest::Client::new();
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
    let client = reqwest::Client::new();
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
    if redirect_uri.starts_with(&oauth_redirect_base()) {
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
            let password_hash = crate::password::hash_password(&unique_id);
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

/// GET /jaxrs/authentication/oauth/login/qywx/code/{code}
pub async fn oauth_login_qywx(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    provider_login(pool, session_manager, QYWX_NAME, &code).await
}

/// GET /jaxrs/authentication/oauth/login/dingding/code/{code}
pub async fn oauth_login_dingding(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    provider_login(pool, session_manager, DINGDING_NAME, &code).await
}

/// GET /jaxrs/authentication/oauth/login/name/{name}/code/{code}/redirecturi/{redirectUri}
pub async fn oauth_login_name(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path((name, code, redirect_uri)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_redirect_uri(&redirect_uri)?;
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
) -> Result<Json<ActionResult<Value>>, AppError> {
    validate_redirect_uri(&redirect_uri)?;
    provider_login(pool, session_manager, &name, &code).await
}
