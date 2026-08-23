//! plan002 U2 收尾：对齐 Java x_organization_assemble_authentication 残余端点。
//!
//! 路径约定：沿用本仓库前缀 `/jaxrs/organization/assemble/authentication/**`，
//! Java 类路径逐段映射（AuthenticationAction 的 `authentication` 段保留）：
//!   GET  authentication/mode                          登录模式开关（配置投影）
//!   POST authentication                               登录（别名，直连 auth::login）
//!   GET  authentication                               当前用户（别名，直连 auth::whoami）
//!   DELETE authentication                             登出（别名，直连 auth::logout）
//!   GET  authentication/mockdeletetoget               登出 MockDeleteToGet
//!   POST authentication/captcha                       图片验证码登录（真实验证码 + 密码）
//!   GET  authentication/captcha/width/{w}/height/{h}  生成验证码（auth crate 复用）
//!   GET  authentication/captchaRSAPublicKey           RSA 公钥配置投影
//!   POST authentication/two/factory/login             双因素登录（auth crate 复用）
//!   GET  bind/list                                    扫码绑定记录列表（x_org_bind_record）
//!   GET  bind/meta/{meta}                             轮询扫码绑定并签发会话（auth crate 复用）
//!   POST bind/meta/{meta}                             确认绑定（auth crate 复用）
//!   PUT/POST switchuser(/mockputtopost)               管理员切换用户（auth crate 复用）
//!   GET  safe/logout                                  安全注销（注销本人全部会话）
//!   GET  sso/encrypt/client/{c}/key/{k}/credential/{cred} SSO 加密（query 版）
//!   POST dingding/info                                钉钉 jsapi 配置签名（SHA1）

use axum::{
    extract::{Extension, Path},
    http::HeaderMap,
    Json,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::{
    error::AppError,
    response::ActionResult,
    session::SessionManager,
};

// ── mode ────────────────────────────────────────────────────────────────────

fn env_flag(name: &str) -> bool {
    std::env::var(name)
        .map(|v| matches!(v.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

/// GET authentication/mode —— 登录模式开关（对齐 Config.person() 投影，经环境变量注入）
pub async fn mode() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(json!({
        "codeLogin": env_flag("AUTH_CODE_LOGIN"),
        "bindLogin": env_flag("AUTH_BIND_LOGIN"),
        "faceLogin": env_flag("AUTH_FACE_LOGIN"),
        "captchaLogin": env_flag("AUTH_CAPTCHA_LOGIN"),
        "twoFactorLogin": env_flag("AUTH_TWO_FACTOR_LOGIN"),
        "userPwdLogin": env_flag("AUTH_USER_PWD_LOGIN"),
    }))))
}

// ── mockdeletetoget（GET 版登出）────────────────────────────────────────────

/// GET authentication/mockdeletetoget —— Java MockDeleteToGet 语义：GET 触发登出。
/// 独立实现以避免复用带 JSON body 提取器的 logout 处理器。
pub async fn logout_get(
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if let Some(token) = shared::middleware::extract_token_from_headers(&headers) {
        session_manager.remove_session(&token).await;
    }
    Ok(Json(ActionResult::success(json!({
        "message": "logged out",
    }))))
}

/// GET authentication/captcha/width/{width}/height/{height} —— auth crate 复用
pub async fn captcha_with_size_alias(
    Path((width, height)): Path<(u32, u32)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    auth::captcha::captcha_with_size(Path((width, height))).await
}

// ── captchaRSAPublicKey ─────────────────────────────────────────────────────

/// GET authentication/captchaRSAPublicKey —— 对齐 Config.publicKey()/token().getRsaEnable()
pub async fn captcha_rsa_public_key() -> Result<Json<ActionResult<Value>>, AppError> {
    let public_key = std::env::var("AUTH_RSA_PUBLIC_KEY").unwrap_or_default();
    Ok(Json(ActionResult::success(json!({
        "publicKey": public_key,
        "rsaEnable": env_flag("AUTH_RSA_ENABLE"),
    }))))
}

// ── captchaLogin ────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CaptchaLoginRequest {
    #[serde(default)]
    pub credential: String,
    #[serde(default)]
    pub password: String,
    #[serde(rename = "captchaId", default)]
    pub captcha_id: String,
    #[serde(rename = "captchaAnswer", default)]
    pub captcha_answer: String,
}

/// POST authentication/captcha —— 图片验证码登录
///
/// 流程：验证码校验（captcha_store，一次性）→ 凭据定位用户 → 锁定检查 →
/// 密码校验 → 签发会话。任一环节失败均拒绝，不产生会话。
pub async fn captcha_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Json(req): Json<CaptchaLoginRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.credential.trim().is_empty()
        || req.password.is_empty()
        || req.captcha_id.trim().is_empty()
        || req.captcha_answer.trim().is_empty()
    {
        return Ok(Json(ActionResult::error("credential, password, captchaId and captchaAnswer are required")));
    }

    use captcha_store::VerifyResult;
    match captcha_store::captcha_store().verify(&req.captcha_id, &req.captcha_answer) {
        VerifyResult::Ok => {}
        VerifyResult::Expired => return Ok(Json(ActionResult::error("captcha expired"))),
        VerifyResult::TooManyAttempts => {
            return Ok(Json(ActionResult::error("too many captcha attempts")))
        }
        _ => return Ok(Json(ActionResult::error("invalid captcha"))),
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile, email, icon, password_hash, locked \
             FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let Some(row) = row else {
        // 统一错误文案，防止凭据枚举
        return Ok(Json(ActionResult::error("invalid credentials")));
    };
    if row.get::<_, bool>("locked") {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let stored_hash: String = row.get("password_hash");
    if !auth::password::verify_password(&req.password, &stored_hash, "", None) {
        return Ok(Json(ActionResult::error("invalid credentials")));
    }

    let person_unique: String = row.get("unique_id");
    let token = uuid::Uuid::new_v4().to_string();
    let session = session_manager.create_session(person_unique.clone(), token.clone()).await?;

    Ok(Json(ActionResult::success(json!({
        "token": session.token,
        "person": {
            "id": row.get::<_, String>("id"),
            "unique": person_unique,
            "name": row.get::<_, Option<String>>("name").unwrap_or_default(),
            "mobile": row.get::<_, Option<String>>("mobile"),
            "email": row.get::<_, Option<String>>("email"),
            "icon": row.get::<_, Option<String>>("icon"),
        },
    }))))
}

// ── safe/logout ─────────────────────────────────────────────────────────────

/// GET safe/logout —— 安全注销：当前人全部会话过期 + 注销本次令牌
///
/// Java ActionSafeLogout 通过 TokenThreshold 广播实现；此处等价地按人撤销全部会话。
pub async fn safe_logout_get(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;
    let person = session.person_unique.clone();

    // 使本人所有会话失效（含当前），再显式移除当前令牌兜底
    session_manager.remove_sessions_by_person(&person).await;
    session_manager.remove_session(&token).await;
    let _ = pool; // pool 仅用于与其它端点保持一致的扩展形态

    Ok(Json(ActionResult::success(json!({
        "tokenType": "anonymous",
        "name": "anonymous",
    }))))
}

// ── sso encrypt (GET 变体) ──────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct SsoEncryptQuery {
    #[serde(default)]
    pub msg_signature: Option<String>,
    #[serde(default)]
    pub timestamp: Option<String>,
    #[serde(default)]
    pub nonce: Option<String>,
    #[serde(default)]
    pub echostr: Option<String>,
}

/// GET authentication/captcha —— 生成图片验证码（默认尺寸，auth crate 复用）
pub async fn captcha_default_alias() -> Result<Json<ActionResult<Value>>, AppError> {
    auth::captcha::captcha_default().await
}

/// GET sso/encrypt/client/{client}/key/{key}/credential/{credential}
///
/// 与 POST /sso/encrypt 同义（Java 提供两种入口），复用 auth crate 的加密逻辑。
pub async fn sso_encrypt_get(
    Path((client, key, credential)): Path<(String, String, String)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let req = auth::sso::SsoEncryptRequest { client, key, credential };
    auth::sso::sso_encrypt(Json(req)).await
}

// ── bind/list ───────────────────────────────────────────────────────────────

/// GET bind/list —— 扫码绑定记录列表（x_org_bind_record，按创建时间升序）
pub async fn bind_list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, COALESCE(message,'') AS message, create_time \
             FROM x_org_bind_record ORDER BY create_time ASC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let items: Vec<Value> = rows
        .iter()
        .map(|r| {
            json!({
                "id": r.get::<_, String>("id"),
                "name": r.get::<_, String>("name"),
                "message": r.get::<_, String>("message"),
            })
        })
        .collect();
    Ok(Json(ActionResult::success(Value::Array(items))))
}

// ── dingding/info ───────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct DingdingInfoWi {
    #[serde(default)]
    pub url: Option<String>,
}

/// POST dingding/info —— 钉钉 jsapi 免登配置签名
///
/// 对齐 Java DingdingAction.info：signature = SHA1(jsapi_ticket=..&noncestr=..&timestamp=..&url=..)。
/// 未配置 DINGDING_CORP_ID/DINGDING_AGENT_ID/DINGDING_JSAPI_TICKET 时显式报错（与 Java 配置缺失行为一致）。
pub async fn dingding_info(
    Json(wi): Json<DingdingInfoWi>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let corp_id = std::env::var("DINGDING_CORP_ID").ok().filter(|s| !s.is_empty());
    let agent_id = std::env::var("DINGDING_AGENT_ID").ok().filter(|s| !s.is_empty());
    let jsticket = std::env::var("DINGDING_JSAPI_TICKET").ok().filter(|s| !s.is_empty());
    let (Some(corp_id), Some(agent_id), Some(jsticket)) = (corp_id, agent_id, jsticket) else {
        return Ok(Json(ActionResult::error("dingding not configured")));
    };
    let Some(url) = wi.url.as_deref().map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(Json(ActionResult::error("url is required")));
    };

    let nonce_str = "o2oa";
    let timestamp = chrono::Utc::now().timestamp();
    let plain = format!(
        "jsapi_ticket={jsticket}&noncestr={nonce_str}&timestamp={timestamp}&url={url}"
    );
    let signature = {
        use sha1::{Digest, Sha1};
        let mut hasher = Sha1::new();
        hasher.update(plain.as_bytes());
        hex::encode(hasher.finalize())
    };

    Ok(Json(ActionResult::success(json!({
        "jsticket": jsticket,
        "agentid": agent_id,
        "corpId": corp_id,
        "nonceStr": nonce_str,
        "signature": signature,
        "timeStamp": timestamp,
    }))))
}

// ── zhengwudingding/info（POST 契约路径）────────────────────────────────────

/// POST zhengwudingding/info —— 政务钉钉配置状态（Java 为 POST；GET 已由 auth crate 提供）
pub async fn zhengwudingding_info_post() -> Result<Json<ActionResult<Value>>, AppError> {
    match std::env::var("ZWDINGDING_API_BASE") {
        Ok(api_base) if !api_base.is_empty() => Ok(Json(ActionResult::success(json!({
            "enabled": true,
            "apiBase": api_base,
        })))),
        _ => Ok(Json(ActionResult::success(json!({ "enabled": false })))),
    }
}
