use axum::{extract::Extension, extract::Path, routing::get, Json, Router};
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;
use shared::session::SessionManager;
use std::sync::OnceLock;

// ──────────────────────────────────────────────────────────────────────────────
// welink — 华为云 WeLink SSO 登录
//
// 配置经环境变量注入（不落代码）：
//   WELINK_APP_KEY / WELINK_APP_SECRET
//
// 流程：前端拿到 WeLink 授权 code → 调 login 端点 → 后端用 code 换 userId
// → 查 auth_person WHERE unique_id = 'welink_{userId}' → 未绑定则返回错误
// （WeLink 用户需先在 OA 中预创建）→ 签发会话 token。
// ──────────────────────────────────────────────────────────────────────────────

const WELINK_UNIQUE_PREFIX: &str = "welink_";
const WELINK_API_BASE: &str = "https://open.welink.huaweicloud.com";

#[derive(Debug, Serialize)]
pub struct WelinkLoginResponse {
    pub token: String,
    pub person: WelinkPersonInfo,
    pub role_list: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct WelinkPersonInfo {
    pub unique: String,
    pub name: String,
    pub mobile: Option<String>,
    pub email: Option<String>,
    pub icon: Option<String>,
}

struct WelinkConfig {
    app_key: String,
    app_secret: String,
}

fn welink_config() -> Option<WelinkConfig> {
    Some(WelinkConfig {
        app_key: std::env::var("WELINK_APP_KEY").ok()?,
        app_secret: std::env::var("WELINK_APP_SECRET").ok()?,
    })
}

fn welink_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| reqwest::Client::new())
}

/// 获取 WeLink accessToken
async fn welink_access_token(config: &WelinkConfig) -> Result<String, AppError> {
    let client = welink_client();
    let resp: Value = client
        .get(format!("{WELINK_API_BASE}/api/auth/v2/token"))
        .query(&[("app_key", &config.app_key), ("app_secret", &config.app_secret)])
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    resp.get("access_token")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or(AppError::Internal)
}

/// 用 code 换取 WeLink userId
async fn welink_user_id(config: &WelinkConfig, code: &str) -> Result<String, AppError> {
    let access_token = welink_access_token(config).await?;
    let client = welink_client();
    let resp: Value = client
        .get(format!("{WELINK_API_BASE}/api/auth/v2/userid"))
        .query(&[("code", code)])
        .header("WeLink-Auth-Key", &access_token)
        .send()
        .await
        .map_err(|_| AppError::Internal)?
        .json()
        .await
        .map_err(|_| AppError::Internal)?;

    resp.get("userid")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or(AppError::Internal)
}

/// 查询用户角色列表
async fn fetch_role_list(pool: &Pool, person_id: &str) -> Result<Vec<String>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT r.name FROM auth_role r \
             JOIN auth_person_role pr ON pr.role_id = r.id \
             WHERE pr.person_id = $1 \
               AND r.deleted_at IS NULL \
               AND r.disable = false",
            &[&person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(rows.iter().map(|r| r.get::<_, String>("name")).collect())
}

/// GET /jaxrs/welink/code/{code} — WeLink SSO 登录
pub async fn welink_login(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    Path(code): Path<String>,
) -> Result<Json<ActionResult<WelinkLoginResponse>>, AppError> {
    let config = welink_config().ok_or(AppError::Internal)?;
    let user_id = welink_user_id(&config, &code).await?;
    let unique_id = format!("{WELINK_UNIQUE_PREFIX}{user_id}");

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&unique_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let (person_id, person_unique, person_name, person_mobile, person_email, person_icon) =
        match row {
            Some(r) => (
                r.get::<_, String>("id"),
                r.get::<_, String>("unique_id"),
                r.get::<_, String>("name"),
                r.get::<_, Option<String>>("mobile"),
                r.get::<_, Option<String>>("email"),
                r.get::<_, Option<String>>("icon"),
            ),
            None => return Ok(Json(ActionResult::error("user not bound to WeLink"))),
        };

    let role_list = fetch_role_list(&pool, &person_id).await?;

    let token = uuid::Uuid::new_v4().to_string();
    let session = session_manager
        .create_session(person_unique.clone(), token.clone())
        .await?;

    Ok(Json(ActionResult::success(WelinkLoginResponse {
        token: session.token,
        person: WelinkPersonInfo {
            unique: person_unique,
            name: person_name,
            mobile: person_mobile,
            email: person_email,
            icon: person_icon,
        },
        role_list,
    })))
}

/// WeLink SSO 路由
pub fn router() -> Router {
    Router::new()
        .route("/jaxrs/welink/code/{code}", get(welink_login))
}
