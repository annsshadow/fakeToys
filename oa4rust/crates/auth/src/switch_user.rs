use axum::{
    extract::{Extension, Json},
    http::HeaderMap,
};
use chrono::Utc;
use serde::Deserialize;
use shared::error::AppError;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;
use shared::session::SessionManager;
use tracing;

/// POST /jaxrs/authentication/switchuser — 用户切换（管理员）
///
/// 管理员临时切换为其他用户身份操作。
/// 原管理员 session 保持有效，返回新 token 用作目标用户身份。
/// 权限：仅 admin 可调用。
#[derive(Debug, Deserialize)]
pub struct SwitchUserRequest {
    pub credential: String,
}

pub async fn switch_user(
    pool: Extension<deadpool_postgres::Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Json(req): Json<SwitchUserRequest>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    // 验证当前用户是 admin
    let token = extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager.validate_session(&token).await.ok_or(AppError::Unauthorized)?;

    if !shared::middleware::is_admin(&pool, &session.person_unique).await {
        return Ok(Json(ActionResult::error("forbidden")));
    }

    // 查找目标用户
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon, job, department, unit, position, \
             change_password_time, password_expired_time FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&req.credential],
        )
        .await
        .map_err(|_| AppError::BadRequest("user not found".to_string()))?;

    let target_id: String = row.get("id");
    let target_unique: String = row.get("unique_id");
    let target_name: String = row.get("name");
    let target_mobile: Option<String> = row.get("mobile");
    let target_email: Option<String> = row.get("email");
    let target_icon: Option<String> = row.get("icon");
    let target_job: Option<String> = row.get("job");
    let target_department: Option<String> = row.get("department");
    let target_unit: Option<String> = row.get("unit");
    let target_position: Option<String> = row.get("position");
    let change_password_time: Option<String> = row.get("change_password_time");
    let password_expired_time: Option<String> = row.get("password_expired_time");

    // 检查密码是否过期（简化实现：如果 change_password_time 为 NULL 则密码过期）
    let password_expired = match change_password_time {
        None => true,
        Some(_) => false,
    };

    // 查询目标用户角色列表
    let role_list: Vec<String> = {
        let role_rows = client
            .query(
                "SELECT r.name FROM auth_role r \
                 JOIN auth_person_role pr ON r.id = pr.role_id \
                 WHERE pr.person_id = $1 AND r.deleted_at IS NULL",
                &[&target_id],
            )
            .await
            .unwrap_or_default();
        role_rows.iter().map(|r| r.get::<_, String>("name")).collect()
    };

    // 为目标用户签发新 session
    let new_token = uuid::Uuid::new_v4().to_string();
    let new_session = session_manager.create_session(target_unique.clone(), new_token.clone()).await;

    tracing::info!(
        switcher = %session.person_unique,
        target = %target_unique,
        "admin user switch"
    );

    Ok(Json(ActionResult::success(serde_json::json!({
        "token": new_session.token,
        "tokenType": "Bearer",
        "roleList": role_list,
        "passwordExpired": password_expired,
        "person": {
            "unique": target_unique,
            "name": target_name,
            "mobile": target_mobile,
            "email": target_email,
            "icon": target_icon,
            "job": target_job,
            "department": target_department,
            "unit": target_unit,
            "position": target_position,
        },
    }))))
}