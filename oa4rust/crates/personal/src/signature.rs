use axum::{
    extract::{Extension, Multipart, Path},
    http::HeaderMap,
    Json,
};
use base64::Engine;
use deadpool_postgres::Pool;
use serde::Serialize;
use serde_json::{json, Value};
use shared::error::AppError;
use shared::middleware::is_admin;
use shared::response::ActionResult;
use uuid::Uuid;

use auth::SessionManager;

use crate::resolve_current_person_unique;

const MAX_SIGNATURE_SIZE: u64 = 5 * 1024 * 1024;

#[derive(Debug, Serialize)]
pub struct SignatureInfo {
    pub id: String,
    pub name: String,
    pub person: String,
    pub value: String,
    pub created_at: Option<String>,
}

/// POST /jaxrs/person/signature/upload
///
/// 接收 multipart/form-data 图片字节，Base64 编码后存入 x_custom 表。
pub async fn upload(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    mut form: Multipart,
) -> Result<Json<ActionResult<SignatureInfo>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;

    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;

    while let Some(field) = form
        .next_field()
        .await
        .map_err(|_| AppError::BadRequest("表单解析失败".to_string()))?
    {
        if field.file_name().is_none() {
            continue;
        }
        filename = field.file_name().map(|f| f.to_string());
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
        None => return Ok(Json(ActionResult::error("未提供签名文件"))),
    };

    if data.len() as u64 > MAX_SIGNATURE_SIZE {
        return Ok(Json(ActionResult::error("文件大小超过限制（最大 5MB）")));
    }

    let signature_id = Uuid::new_v4().to_string();
    let name = format!("SIGNATURE_{signature_id}");
    let encoded = base64::engine::general_purpose::STANDARD.encode(&data);

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    client
        .execute(
            "INSERT INTO x_custom (id, name, person, value, created_at) \
             VALUES ($1, $2, $3, $4, NOW())",
            &[&signature_id, &name, &person_unique, &encoded],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let info = SignatureInfo {
        id: signature_id,
        name,
        person: person_unique,
        value: encoded,
        created_at: None,
    };

    Ok(Json(ActionResult::success(info)))
}

/// GET /jaxrs/person/signature/list
///
/// 返回当前用户的所有签名列表。
pub async fn list(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, value, created_at \
             FROM x_custom \
             WHERE person = $1 AND deleted_at IS NULL \
             ORDER BY created_at DESC",
            &[&person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let signatures: Vec<Value> = rows
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<_, String>("id"),
                "name": row.get::<_, String>("name"),
                "person": row.get::<_, String>("person"),
                "value": row.get::<_, String>("value"),
                "created_at": row.get::<_, Option<String>>("created_at").map(|s| {
                    chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                        .ok()
                        .map(|dt| dt.to_string())
                        .unwrap_or(s)
                }),
            })
        })
        .collect();

    Ok(Json(ActionResult::success(json!({ "signatures": signatures }))))
}

/// GET /jaxrs/person/signature/delete/{id}
///
/// 软删除指定签名（仅当前用户自己的签名）。
pub async fn delete(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows_affected = client
        .execute(
            "UPDATE x_custom SET deleted_at = NOW() \
             WHERE id = $1 AND person = $2 AND deleted_at IS NULL",
            &[&id, &person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if rows_affected == 0 {
        return Ok(Json(ActionResult::error("签名不存在或无权删除")));
    }

    Ok(Json(ActionResult::success(json!({ "success": true }))))
}

/// GET /jaxrs/person/signature/manager/list
///
/// 管理员查看所有用户签名列表。
pub async fn manager_list(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let person_unique = resolve_current_person_unique(&session_manager, &headers).await?;

    if !is_admin(&pool, &person_unique).await {
        return Err(AppError::Forbidden);
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, person, value, created_at \
             FROM x_custom \
             WHERE deleted_at IS NULL \
             ORDER BY created_at DESC",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let signatures: Vec<Value> = rows
        .iter()
        .map(|row| {
            json!({
                "id": row.get::<_, String>("id"),
                "name": row.get::<_, String>("name"),
                "person": row.get::<_, String>("person"),
                "value": row.get::<_, String>("value"),
                "created_at": row.get::<_, Option<String>>("created_at").map(|s| {
                    chrono::NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                        .ok()
                        .map(|dt| dt.to_string())
                        .unwrap_or(s)
                }),
            })
        })
        .collect();

    Ok(Json(ActionResult::success(json!({ "signatures": signatures }))))
}
