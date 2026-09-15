use axum::{extract::Extension, http::HeaderMap, Json};
use deadpool_postgres::Pool;
use serde::Deserialize;
use shared::error::AppError;
use shared::response::ActionResult;

use auth::SessionManager;
use personal::PersonInfo;

// 更新个人信息请求 DTO
#[derive(Debug, Deserialize)]
pub struct UpdatePersonalRequest {
    pub name: Option<String>,
    pub mobile: Option<String>,
    pub email: Option<String>,
}

// 获取当前登录用户信息
//
// 从 Authorization header 中提取 token，验证会话后查询 auth_person 表，
// 返回当前用户的 id、唯一标识、姓名、手机号、邮箱、头像等基本信息。
//
// # 参数
// - `pool`: 数据库连接池
// - `session_manager`: 会话管理器，用于验证当前用户身份
// - `headers`: 请求头，从中提取 Bearer token
pub async fn get_info(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let token =
        shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let info = PersonInfo {
        id: row.get("id"),
        unique: row.get("unique_id"),
        name: row.get("name"),
        mobile: row.get("mobile"),
        email: row.get("email"),
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(info)))
}

// 更新当前登录用户的个人信息
//
// 支持部分更新 name、mobile、email 字段，未提供的字段保留原值。
// 更新后同时刷新 updated_at 时间戳。
//
// # 参数
// - `pool`: 数据库连接池
// - `session_manager`: 会话管理器，用于验证当前用户身份
// - `headers`: 请求头，从中提取 Bearer token
// - `req`: 更新请求体，包含可选的 name、mobile、email 字段
pub async fn update_info(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    axum::extract::Json(req): axum::extract::Json<UpdatePersonalRequest>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    let token =
        shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 先查询当前用户信息，用于保留未更新的字段
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let person_id: String = row.get("id");
    let current_name: String = row.get("name");
    let current_mobile: Option<String> = row.get("mobile");
    let current_email: Option<String> = row.get("email");
    let icon: Option<String> = row.get("icon");

    let name = req.name.unwrap_or(current_name);
    let mobile = req.mobile.or(current_mobile);
    let email = req.email.or(current_email);

    client
        .execute(
            "UPDATE auth_person SET name = $1, mobile = $2, email = $3, updated_at = NOW() WHERE id = $4",
            &[&name, &mobile, &email, &person_id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let updated = PersonInfo {
        id: person_id,
        unique: session.person_unique,
        name,
        mobile,
        email,
        icon,
    };

    Ok(Json(ActionResult::success(updated)))
}

// 获取指定用户的信息（需登录）
//
// 查询 auth_person 表中指定 unique_id 的用户信息。
// 需要当前用户已登录，且目标用户未被锁定。
//
// # 参数
// - `pool`: 数据库连接池
// - `session_manager`: 会话管理器，用于验证当前用户身份
// - `headers`: 请求头，从中提取 Bearer token
// - `id`: 路径参数，目标用户的唯一标识
pub async fn get_detail(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<PersonInfo>>, AppError> {
    // 验证当前请求者已登录
    let token =
        shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let _session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, icon FROM auth_person \
             WHERE unique_id = $1 AND locked = false AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let info = PersonInfo {
        id: row.get("id"),
        unique: row.get("unique_id"),
        name: row.get("name"),
        mobile: row.get("mobile"),
        email: row.get("email"),
        icon: row.get("icon"),
    };

    Ok(Json(ActionResult::success(info)))
}

// ── person/signature/save + personal/face/list（斜杠路径家族补齐，查/写真实表 095）──
/// POST /jaxrs/person/signature/save —— 保存当前登录用户签名（upsert x_person_signature）。
pub async fn save_signature(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    let token =
        shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let signature = payload.get("signature").and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let mime = payload
        .get("mimeType")
        .and_then(|v| v.as_str())
        .unwrap_or("image/png")
        .to_string();

    let existing = client
        .query_opt(
            "SELECT id FROM x_person_signature WHERE person_id = $1 AND deleted_at IS NULL LIMIT 1",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if let Some(row) = existing {
        let eid: String = row.get("id");
        client
            .execute(
                "UPDATE x_person_signature SET signature = $1, mime_type = $2, update_time = NOW() WHERE id = $3",
                &[&signature, &mime, &eid],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        Ok(Json(ActionResult::success(serde_json::Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), serde_json::Value::String(eid)),
                ("saved".to_string(), serde_json::Value::Bool(true)),
            ]),
        ))))
    } else {
        let id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_person_signature (id, person_id, signature, mime_type, creator, create_time, update_time) \
                 VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
                &[&id, &session.person_unique, &signature, &mime, &session.person_unique],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        Ok(Json(ActionResult::success(serde_json::Value::Object(
            serde_json::Map::from_iter([
                ("id".to_string(), serde_json::Value::String(id)),
                ("saved".to_string(), serde_json::Value::Bool(true)),
            ]),
        ))))
    }
}

/// GET /jaxrs/personal/face/list —— 当前登录用户的人脸特征列表（x_person_face）。
pub async fn face_list(
    pool: Extension<Pool>,
    session_manager: Extension<SessionManager>,
    headers: HeaderMap,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    let token =
        shared::middleware::extract_token_from_headers(&headers).ok_or(AppError::Unauthorized)?;
    let session = session_manager
        .validate_session(&token)
        .await
        .ok_or(AppError::Unauthorized)?;
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, face_name, face_type, status, create_time::text AS create_time FROM x_person_face \
             WHERE person_id = $1 AND deleted_at IS NULL ORDER BY create_time DESC",
            &[&session.person_unique],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<serde_json::Value> = rows
        .iter()
        .map(|row| {
            serde_json::Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), serde_json::Value::String(row.get("id"))),
                ("faceName".to_string(), serde_json::Value::String(row.get::<_, Option<String>>("face_name").unwrap_or_default())),
                ("faceType".to_string(), serde_json::Value::String(row.get::<_, Option<String>>("face_type").unwrap_or_default())),
                ("status".to_string(), serde_json::Value::String(row.get::<_, Option<String>>("status").unwrap_or_default())),
                ("createTime".to_string(), serde_json::Value::String(row.get::<_, Option<String>>("create_time").unwrap_or_default())),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::java_success(
        serde_json::Value::Array(data),
        count,
        0,
    )))
}

// ── face 家族 CRUD（x_person_face 095，通用参数化写）──
fn face_spec() -> shared::crud::CrudSpec {
    shared::crud::CrudSpec {
        table: "x_person_face",
        columns: &[
            ("personId", "person_id"),
            ("faceName", "face_name"),
            ("faceType", "face_type"),
            ("feature", "feature"),
            ("status", "status"),
        ],
        soft_delete: true,
    }
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn face_create(
    pool: Extension<Pool>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    let id = shared::crud_create(&pool, &face_spec(), &payload).await?;
    Ok(Json(ActionResult::success(serde_json::Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), serde_json::Value::String(id)),
            ("created".to_string(), serde_json::Value::Bool(true)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn face_save(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    let saved = shared::crud_save(&pool, &face_spec(), &id, &payload).await?;
    Ok(Json(ActionResult::success(serde_json::Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), serde_json::Value::String(id)),
            ("saved".to_string(), serde_json::Value::Bool(saved)),
        ]),
    ))))
}

#[axum::debug_handler]
#[allow(non_snake_case)]
pub async fn face_delete(
    pool: Extension<Pool>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    let deleted = shared::crud_delete(&pool, &face_spec(), &id).await?;
    Ok(Json(ActionResult::success(serde_json::Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), serde_json::Value::String(id)),
            ("deleted".to_string(), serde_json::Value::Bool(deleted)),
        ]),
    ))))
}
