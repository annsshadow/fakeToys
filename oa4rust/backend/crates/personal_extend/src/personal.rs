// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
/// POST /api/person/signature/save —— 保存当前登录用户签名（upsert x_person_signature）。
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

    let signature = payload
        .get("signature")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string();
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

/// GET /api/personal/face/list —— 当前登录用户的人脸特征列表（x_person_face）。
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
                (
                    "faceName".to_string(),
                    serde_json::Value::String(
                        row.get::<_, Option<String>>("face_name")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "faceType".to_string(),
                    serde_json::Value::String(
                        row.get::<_, Option<String>>("face_type")
                            .unwrap_or_default(),
                    ),
                ),
                (
                    "status".to_string(),
                    serde_json::Value::String(
                        row.get::<_, Option<String>>("status").unwrap_or_default(),
                    ),
                ),
                (
                    "createTime".to_string(),
                    serde_json::Value::String(
                        row.get::<_, Option<String>>("create_time")
                            .unwrap_or_default(),
                    ),
                ),
            ]))
        })
        .collect();

    let count = data.len() as i64;
    Ok(Json(ActionResult::legacy_success(
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

/// save 用**窄**白名单（同 bbs 的 `section_update_spec` 惯例）：只列可写列，**不含 `person_id`**。
/// `crud_save` 会把 spec 里每一列都按 payload 取值写回（缺键取 `""`），全列 spec 下
/// 「属主改个 faceName」这一步就会把归属列抹成空串——之后本人再 delete/save 全部 403（实测复现）。
fn face_update_spec() -> shared::crud::CrudSpec {
    shared::crud::CrudSpec {
        table: "x_person_face",
        columns: &[
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
    session: Option<Extension<shared::session::Session>>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    // IDOR 门禁（A3-N9）：人脸特征属生物特征数据，建档只允许为**本人**（`person_id` 强制
    // 取会话登录人，忽略请求体里的他人 id）；无会话 401。admin 可为任意人建档（保留其 personId）。
    let Some(Extension(sess)) = session else {
        return Err(AppError::Unauthorized);
    };
    let mut payload = payload;
    if !shared::middleware::is_admin(&pool, &sess.person_unique).await {
        match payload.as_object_mut() {
            Some(obj) => {
                obj.insert(
                    "personId".to_string(),
                    serde_json::Value::String(sess.person_unique.clone()),
                );
            }
            None => {
                return Err(AppError::BadRequest(
                    "payload must be an object".to_string(),
                ))
            }
        }
    }
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
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    // IDOR 门禁（A3-N9）：人脸属 person_id 本人，收归属门（admin 放行 / 本人放行）。
    // save 走窄白名单（不含 person_id）：归属列只由 face_create 定，save 不改写它。
    let Some(Extension(sess)) = session else {
        return Err(AppError::Unauthorized);
    };
    let saved = shared::crud_save_owned(
        &pool,
        &face_update_spec(),
        &id,
        &payload,
        "person_id",
        &sess,
    )
    .await?;
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
    session: Option<Extension<shared::session::Session>>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<serde_json::Value>>, AppError> {
    // IDOR 门禁（A3-N9）：人脸属 person_id 本人，收归属门。
    let Some(Extension(sess)) = session else {
        return Err(AppError::Unauthorized);
    };
    let deleted = shared::crud_delete_owned(&pool, &face_spec(), &id, "person_id", &sess).await?;
    Ok(Json(ActionResult::success(serde_json::Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), serde_json::Value::String(id)),
            ("deleted".to_string(), serde_json::Value::Bool(deleted)),
        ]),
    ))))
}
