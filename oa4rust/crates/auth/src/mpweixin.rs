use axum::{
    extract::Extension,
    extract::Path,
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::error::AppError;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;
use shared::session::SessionManager;
use std::sync::OnceLock;

// ──────────────────────────────────────────────────────────────────────────────
// mpweixin — 微信小程序 OAuth 登录
//
// 配置经环境变量注入（小程序 AppId/AppSecret 不落代码）：
//   MPWEXIN_APP_ID / MPWEXIN_APP_SECRET
//
// 流程：小程序端调用 wx.login() 获取临时 code →
// 前端携带 code 请求后端 login 端点 → 后端 code→openid 交换（真实 HTTP 调用）→
// 按 unique_id = 'mpwx_{openid}' 绑定或创建本地用户 → 签发会话 token。
//
// 绑定关系采用 unique_id 约定前缀：mpwx_{openid}。
// ──────────────────────────────────────────────────────────────────────────────

const MPWEXIN_UNIQUE_PREFIX: &str = "mpwx_";

static MPWEXIN_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn mpweixin_client() -> &'static reqwest::Client {
    MPWEXIN_CLIENT.get_or_init(|| reqwest::Client::new())
}

fn mpweixin_config() -> Result<(String, String), AppError> {
    let app_id = std::env::var("MPWEXIN_APP_ID").map_err(|_| AppError::Internal)?;
    let app_secret = std::env::var("MPWEXIN_APP_SECRET").map_err(|_| AppError::Internal)?;
    Ok((app_id, app_secret))
}

/// 微信 code→openid 交换
async fn mpweixin_openid(code: &str) -> Result<String, AppError> {
    let (app_id, app_secret) = mpweixin_config()?;
    let client = mpweixin_client();
    let resp: Value = client
        .get("https://api.weixin.qq.com/sns/oauth2/access_token")
        .query(&[
            ("appid", app_id.as_str()),
            ("secret", app_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
        ])
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
    resp.get("openid")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or(AppError::Internal)
}

/// 获取微信 access_token（带缓存）
static WECHAT_ACCESS_TOKEN: OnceLock<std::sync::Mutex<(String, std::time::Instant)>> =
    std::sync::OnceLock::new();

fn wechat_token_cache() -> &'static std::sync::Mutex<(String, std::time::Instant)> {
    WECHAT_ACCESS_TOKEN.get_or_init(|| {
        std::sync::Mutex::new((String::new(), std::time::Instant::now() - std::time::Duration::from_secs(7000)))
    })
}

async fn mpweixin_access_token() -> Result<String, AppError> {
    let cache = wechat_token_cache();
    {
        let guard = cache.lock().unwrap();
        let (token, issued_at) = &*guard;
        if !token.is_empty() && issued_at.elapsed() < std::time::Duration::from_secs(7000) {
            return Ok(token.clone());
        }
    }

    let (app_id, app_secret) = mpweixin_config()?;
    let client = mpweixin_client();
    let resp: Value = client
        .get("https://api.weixin.qq.com/cgi-bin/token")
        .query(&[
            ("grant_type", "client_credential"),
            ("appid", app_id.as_str()),
            ("secret", app_secret.as_str()),
        ])
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

    let token = resp
        .get("access_token")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or(AppError::Internal)?;

    {
        let mut guard = cache.lock().unwrap();
        *guard = (token.clone(), std::time::Instant::now());
    }

    Ok(token)
}

/// 按 openid 查找或创建本地用户，并签发会话 token
async fn mpweixin_login_or_create(
    pool: &Pool,
    session_manager: &SessionManager,
    openid: &str,
) -> Result<Value, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let unique_id = format!("{MPWEXIN_UNIQUE_PREFIX}{openid}");

    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile, email FROM auth_person \
             WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let person_id: String = r.get("id");
            let person_unique: String = r.get("unique_id");
            let person_name: String = r.get("name");
            let person_mobile: Option<String> = r.get("mobile");
            let person_email: Option<String> = r.get("email");

            let token = uuid::Uuid::new_v4().to_string();
            let session = session_manager.create_session(person_unique.clone(), token.clone()).await?;

            Ok(serde_json::json!({
                "token": session.token,
                "person": {
                    "id": person_id,
                    "unique": person_unique,
                    "name": person_name,
                    "mobile": person_mobile.unwrap_or_default(),
                    "email": person_email.unwrap_or_default(),
                },
            }))
        }
        None => Ok(serde_json::json!({
            "unbind": true,
            "mpwxopenId": openid,
        })),
    }
}

/// GET /jaxrs/mpweixin/login/code/{code} —— 微信小程序 code 登录
pub async fn mpweixin_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if code.is_empty() {
        return Ok(Json(ActionResult::error("code is required")));
    }
    let openid = mpweixin_openid(&code).await?;
    let result = mpweixin_login_or_create(&*pool, &*session_manager, &openid).await?;
    Ok(Json(ActionResult::success(result)))
}

/// GET /jaxrs/mpweixin/bind/code/{code} —— 绑定 openid 到当前登录用户
pub async fn mpweixin_bind_code(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let openid = mpweixin_openid(&code).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE auth_person SET mpwxopenId = $1 WHERE unique_id = $2 AND deleted_at IS NULL",
            &[&openid, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(serde_json::json!({ "value": true }))))
}

/// GET /jaxrs/mpweixin/bind/openid/{openid} —— 直接绑定 openid 到当前用户
pub async fn mpweixin_bind_openid(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(openid): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    if openid.is_empty() {
        return Ok(Json(ActionResult::error("openid is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE auth_person SET mpwxopenId = $1 WHERE unique_id = $2 AND deleted_at IS NULL",
            &[&openid, &session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(serde_json::json!({ "value": true }))))
}

/// POST /jaxrs/mpweixin/menu/test/send/to/{person} —— 管理员测试发送模板消息
pub async fn mpweixin_test_send(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(person): Path<String>,
    Json(req): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    if !shared::middleware::is_admin(&pool, &session.person_unique).await {
        return Ok(Json(ActionResult::error("forbidden")));
    }

    if person.is_empty() {
        return Ok(Json(ActionResult::error("person is required")));
    }

    let template_id = req
        .get("template_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("template_id is required".to_string()))?;

    let content = req
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::BadRequest("content is required".to_string()))?;

    let access_token = mpweixin_access_token().await?;
    let client = mpweixin_client();
    let send_resp: Value = client
        .post(format!(
            "https://api.weixin.qq.com/cgi-bin/message/wxopen/template/send?access_token={}",
            access_token
        ))
        .json(&serde_json::json!({
            "touser": person,
            "template_id": template_id,
            "content": content,
        }))
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let errcode = send_resp.get("errcode").and_then(|v| v.as_i64()).unwrap_or(-1);
    if errcode != 0 {
        let errmsg = send_resp
            .get("errmsg")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown error");
        return Ok(Json(ActionResult::error(format!(
            "WeChat template send failed: {}",
            errmsg
        ))));
    }

    Ok(Json(ActionResult::success(serde_json::json!({
        "sent": true,
        "msgid": send_resp.get("msgid").and_then(|v| v.as_str()).unwrap_or(""),
    }))))
}

pub fn router() -> Router {
    Router::new()
        .route("/jaxrs/mpweixin/login/code/{code}", get(mpweixin_login))
        .route("/jaxrs/mpweixin/bind/code/{code}", get(mpweixin_bind_code))
        .route("/jaxrs/mpweixin/bind/openid/{openid}", get(mpweixin_bind_openid))
        .route("/jaxrs/mpweixin/menu/test/send/to/{person}", post(mpweixin_test_send))
}
