use axum::{
    extract::{Extension, Multipart, Path},
    http::HeaderMap,
    Json,
};
use base64::Engine;
use deadpool_postgres::Pool;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::response::ActionResult;

use auth::SessionManager;

use crate::resolve_current_person_unique;

const MAX_ICON_SIZE: u64 = 5 * 1024 * 1024;

/// GET /jaxrs/person/icon/{person}
///
/// 公开接口，无需认证。支持按 unique_id、name 或 id 查询用户头像。
/// 返回 auth_person.icon 字段（base64 字符串），未设置时 icon 为空字符串、exists 为 false。
pub async fn get(
    pool: Extension<Pool>,
    Path(person): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT icon FROM auth_person \
             WHERE (unique_id = $1 OR name = $1 OR id = $1) \
             AND locked = false AND deleted_at IS NULL",
            &[&person],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(r) => {
            let icon: Option<String> = r.get("icon");
            let icon_str = icon.unwrap_or_default();
            let exists = !icon_str.is_empty();
            Ok(Json(ActionResult::success(json!({
                "icon": icon_str,
                "exists": exists,
            }))))
        }
        None => Ok(Json(ActionResult::error("用户不存在"))),
    }
}

/// POST /jaxrs/person/icon/upload
///
/// 接收 multipart/form-data 图片文件，base64 编码后写入 auth_person.icon 字段。
/// 需要认证，仅当前登录用户可更新自己的头像。
pub async fn upload(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    mut form: Multipart,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;

    let mut file_data: Option<Vec<u8>> = None;

    while let Some(field) = form
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("表单解析失败".to_string()))?
    {
        if field.file_name().is_none() {
            continue;
        }
        file_data = Some(
            field
                .bytes()
                .await
                .map_err(|_| AppError::BadRequest("文件读取失败".to_string()))?
                .to_vec(),
        );
        break;
    }

    let data = match file_data {
        Some(d) => d,
        None => return Ok(Json(ActionResult::error("未提供头像文件"))),
    };

    if data.len() as u64 > MAX_ICON_SIZE {
        return Ok(Json(ActionResult::error("文件大小超过限制（最大 5MB）")));
    }

    let encoded = base64::engine::general_purpose::STANDARD.encode(&data);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "UPDATE auth_person SET icon = $1, updated_at = NOW() \
             WHERE unique_id = $2 AND locked = false AND deleted_at IS NULL",
            &[&encoded, &person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(json!({
        "icon": encoded,
        "exists": true,
    }))))
}
