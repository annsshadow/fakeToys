use axum::{extract::Extension, extract::Path, routing::post, routing::get, Json as AxumJson, Router};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;
use sha2::Digest;
use std::sync::OnceLock;

const QYWXPOS_UNIQUE_PREFIX: &str = "qywxpos_";

fn qywxpos_config() -> Option<(String, String)> {
    Some((
        std::env::var("QYWX_CORP_ID").ok()?,
        std::env::var("QYWX_APP_SECRET").ok()?,
    ))
}

fn qywx_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| reqwest::Client::new())
}

async fn exchange_code_for_userid(config: &(String, String), code: &str) -> Result<String, AppError> {
    let client = qywx_client();
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

async fn get_user_detail(config: &(String, String), userid: &str) -> Result<Value, AppError> {
    let client = qywx_client();
    let token_resp: Value = client
        .get("https://qyapi.weixin.qq.com/cgi-bin/gettoken")
        .query(&[("corpid", &config.0), ("corpsecret", &config.1)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    let access_token = token_resp
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or(AppError::Internal)?;

    let user_resp: Value = client
        .get("https://qyapi.weixin.qq.com/cgi-bin/user/get")
        .query(&[("access_token", access_token), ("userid", userid)])
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
    Ok(user_resp)
}

async fn create_session(token: &str, person_unique: &str, session_manager: &SessionManager) -> Result<String, AppError> {
    let session = session_manager.create_session(person_unique.to_string(), token.to_string()).await?;
    Ok(session.token)
}

/// GET /jaxrs/qiyeweixin/code/{code}
pub async fn qiyeweixin_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let config = qywxpos_config().ok_or(AppError::Internal)?;
    let userid = exchange_code_for_userid(&config, &code).await?;
    let unique_id = format!("{}{}", QYWXPOS_UNIQUE_PREFIX, userid);

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
            let _person_id: String = r.get("id");
            let person_unique: String = r.get("unique_id");
            let person_name: Option<String> = r.get("name");
            let person_mobile: Option<String> = r.get("mobile");
            let person_email: Option<String> = r.get("email");
            let person_icon: Option<String> = r.get("icon");

            let token = uuid::Uuid::new_v4().to_string();
            let session_token = create_session(&token, &person_unique, &session_manager).await?;

            let person = json!({
                "unique": person_unique,
                "name": person_name.unwrap_or_default(),
                "mobile": person_mobile,
                "email": person_email,
                "icon": person_icon,
            });

            Ok(AxumJson(ActionResult::success(json!({
                "token": session_token,
                "person": person,
                "unbind": false,
            }))))
        }
        None => {
            Ok(AxumJson(ActionResult::success(json!({
                "userid": userid,
                "unbind": true,
            }))))
        }
    }
}

/// GET /jaxrs/qiyeweixin/update/person/detail/{code}
pub async fn qiyeweixin_update_person_detail(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let config = qywxpos_config().ok_or(AppError::Internal)?;
    let userid = exchange_code_for_userid(&config, &code).await?;
    let unique_id = format!("{}{}", QYWXPOS_UNIQUE_PREFIX, userid);

    let detail_resp = get_user_detail(&config, &userid).await?;
    let name: Option<String> = detail_resp.get("name").and_then(|v| v.as_str()).map(|s| s.to_string());
    let mobile: Option<String> = detail_resp.get("mobile").and_then(|v| v.as_str()).map(|s| s.to_string());
    let email: Option<String> = detail_resp.get("email").and_then(|v| v.as_str()).map(|s| s.to_string());

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let (person_id, person_unique, _person_name, _person_mobile, _person_email) = match row {
        Some(r) => (
            r.get::<_, String>("id"),
            r.get::<_, String>("unique_id"),
            r.get::<_, Option<String>>("name"),
            r.get::<_, Option<String>>("mobile"),
            r.get::<_, Option<String>>("email"),
        ),
        None => {
            let id = uuid::Uuid::new_v4().to_string();
            let password_hash = crate::password::hash_password(&uuid::Uuid::new_v4().to_string());
            client
                .execute(
                    "INSERT INTO auth_person (id, unique_id, name, mobile, email, password_hash, locked, created_at) \
                     VALUES ($1, $2, $3, $4, $5, $6, false, NOW())",
                    &[&id, &unique_id, &name.clone().unwrap_or_default(), &mobile.clone().unwrap_or_default(), &email.clone().unwrap_or_default(), &password_hash],
                )
                .await
                .map_err(|_| AppError::Internal)?;
            (id, unique_id.clone(), name.clone(), mobile.clone(), email.clone())
        }
    };

    if let (Some(n), Some(m), Some(e)) = (&name, &mobile, &email) {
        if !n.is_empty() || !m.is_empty() || !e.is_empty() {
            let _ = client
                .execute(
                    "UPDATE auth_person SET name = COALESCE(NULLIF($1, ''), name), \
                     mobile = COALESCE(NULLIF($2, ''), mobile), \
                     email = COALESCE(NULLIF($3, ''), email), \
                     updated_at = NOW() \
                     WHERE unique_id = $4",
                    &[n, m, e, &unique_id],
                )
                .await;
        }
    }

    let token = uuid::Uuid::new_v4().to_string();
    let session_token = create_session(&token, &person_unique, &session_manager).await?;

    let icon: Option<String> = client
        .query_opt(
            "SELECT icon FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .and_then(|r| r.get::<_, Option<String>>("icon"));

    Ok(AxumJson(ActionResult::success(json!({
        "token": session_token,
        "person": {
            "id": person_id,
            "unique": person_unique,
            "name": name.unwrap_or_default(),
            "mobile": mobile,
            "email": email,
            "icon": icon,
        },
        "unbind": false,
    }))))
}

#[derive(Debug, Deserialize)]
pub struct JssdkSignRequest {
    pub url: String,
    pub nonce_str: Option<String>,
    #[serde(rename = "jsticketType")]
    pub jsticket_type: Option<String>,
}

fn get_jsapi_ticket() -> Option<String> {
    std::env::var("QYWX_JSAPI_TICKET").ok()
}

fn get_qywx_agent_id() -> Option<String> {
    std::env::var("QYWX_AGENT_ID").ok()
}

/// POST /jaxrs/qiyeweixin/jssdk/sign/info
pub async fn qiyeweixin_jssdk_sign(
    AxumJson(req): AxumJson<JssdkSignRequest>,
) -> Result<AxumJson<ActionResult<Value>>, AppError> {
    let ticket = get_jsapi_ticket().ok_or(AppError::Internal)?;
    let nonce_str = req.nonce_str.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
    let timestamp = chrono::Utc::now().timestamp() as u64;
    let corpid = std::env::var("QYWX_CORP_ID").map_err(|_| AppError::Internal)?;
    let agentid = get_qywx_agent_id().ok_or(AppError::Internal)?;

    let text = format!("jsapi_ticket={}&noncestr={}&timestamp={}&url={}", ticket, nonce_str, timestamp, req.url);
    let signature = format!("{:x}", sha1::Sha1::digest(text.as_bytes()));

    Ok(AxumJson(ActionResult::success(json!({
        "signature": signature,
        "nonceStr": nonce_str,
        "timestamp": timestamp,
        "corpid": corpid,
        "agentid": agentid,
    }))))
}

pub fn router() -> Router {
    Router::new()
        .route("/jaxrs/qiyeweixin/code/{code}", get(qiyeweixin_login))
        .route("/jaxrs/qiyeweixin/update/person/detail/{code}", get(qiyeweixin_update_person_detail))
        .route("/jaxrs/qiyeweixin/jssdk/sign/info", post(qiyeweixin_jssdk_sign))
}
