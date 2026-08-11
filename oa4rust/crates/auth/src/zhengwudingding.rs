use axum::{
    extract::Extension,
    extract::Path,
    routing::get,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;

// ──────────────────────────────────────────────────────────────────────────────
// zhengwudingding — 政务钉钉登录
//
// 配置经环境变量注入（不落代码）：
//   ZWDINGDING_API_BASE         政务钉钉 API 基础地址
//   ZWDINGDING_CORP_ACCESS_TOKEN 企业级 access_token（用于 code→dingUserId）
//   ZWDINGDING_APP_ACCESS_TOKEN   应用级 access_token（用于 dingUserId→userId）
//
// 流程：
//   1. GET {API_BASE}/user/getuserinfo?access_token={corp_token}&code={code}
//      → 返回 dingUserId
//   2. POST {API_BASE}/user/singleGetUserIdByDingId?access_token={app_token}&dingUserId={dingUserId}
//      → 返回 userId
//   3. 按 unique_id = 'zwding_{userId}' 查询 auth_person
//   4. 找到则签发会话；未找到则返回 "user not found"
//
// 端点：
//   GET /jaxrs/zhengwudingding/code/{code}     — 登录
//   GET /jaxrs/zhengwudingding/info            — 配置状态
// ──────────────────────────────────────────────────────────────────────────────

/// GET /jaxrs/zhengwudingding/code/{code} — 政务钉钉登录
pub async fn zwdingding_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
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

    let unique_id = format!("zwding_{user_id}");

    // Step 3: 查询本地用户
    let db = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = db
        .query_opt(
            "SELECT unique_id, name, mobile, email, icon FROM auth_person \
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

    // Step 4: 签发会话
    let token = uuid::Uuid::new_v4().to_string();
    let session = session_manager
        .create_session(person_unique.clone(), token.clone())
        .await;

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

/// GET /jaxrs/zhengwudingding/info — 配置状态
pub async fn zwdingding_info() -> Result<Json<ActionResult<Value>>, AppError> {
    let api_base = match std::env::var("ZWDINGDING_API_BASE") {
        Ok(v) => v,
        Err(_) => return Ok(Json(ActionResult::success(json!({ "enabled": false })))),
    };
    Ok(Json(ActionResult::success(json!({
        "enabled": true,
        "apiBase": api_base,
    }))))
}

pub fn router() -> Router {
    Router::new()
        .route("/jaxrs/zhengwudingding/code/{code}", get(zwdingding_login))
        .route("/jaxrs/zhengwudingding/info", get(zwdingding_info))
}
