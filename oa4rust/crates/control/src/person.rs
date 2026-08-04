use axum::extract::{Extension, Path, Query};
use axum::Json;
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;
use std::collections::HashMap;

/// 创建人员请求体
#[derive(Debug, Deserialize)]
pub struct PersonCreateRequest {
    /// 唯一标识（如工号）
    pub unique_id: String,
    /// 姓名
    pub name: String,
    /// 手机号（可选）
    pub mobile: Option<String>,
    /// 邮箱（可选）
    pub email: Option<String>,
    /// 密码（创建时必填）
    pub password: String,
}

/// 更新人员请求体
#[derive(Debug, Deserialize)]
pub struct PersonUpdateRequest {
    /// 姓名
    pub name: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 是否锁定（true=锁定，false=解锁）
    pub locked: Option<bool>,
}

/// 获取人员详情
pub async fn get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, unique_id, name, mobile, email, locked \
             FROM auth_person WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("uniqueId".to_string(), Value::String(row.get("unique_id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("mobile".to_string(), Value::String(row.get::<_, Option<String>>("mobile").unwrap_or_default())),
        ("email".to_string(), Value::String(row.get::<_, Option<String>>("email").unwrap_or_default())),
        ("locked".to_string(), Value::Bool(row.get("locked"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 获取人员列表（支持分页）
pub async fn list(
    pool: Extension<Pool>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let page = 1i64;
    let size = 20i64;
    let offset = 0i64;

    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) as count FROM auth_person WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let rows = client
        .query(
            "SELECT id, unique_id, name, mobile, email, locked \
             FROM auth_person \
             WHERE deleted_at IS NULL \
             ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            &[&size, &offset],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("uniqueId".to_string(), Value::String(row.get("unique_id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("mobile".to_string(), Value::String(row.get::<_, Option<String>>("mobile").unwrap_or_default())),
                ("email".to_string(), Value::String(row.get::<_, Option<String>>("email").unwrap_or_default())),
                ("locked".to_string(), Value::Bool(row.get("locked"))),
            ]))
        })
        .collect();

    let result = Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(total))),
        ("size".to_string(), Value::Number(serde_json::Number::from(size))),
        ("page".to_string(), Value::Number(serde_json::Number::from(page))),
        ("data".to_string(), Value::Array(data)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 创建人员
pub async fn create(
    pool: Extension<Pool>,
    Json(req): Json<PersonCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.unique_id.is_empty() || req.name.is_empty() || req.password.is_empty() {
        return Ok(Json(ActionResult::error("unique_id, name and password are required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let existing = client
        .query_one(
            "SELECT 1 FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&req.unique_id],
        )
        .await;

    if existing.is_ok() {
        return Ok(Json(ActionResult::error("unique_id already exists")));
    }

    // 密码哈希（使用 MD5，兼容旧系统）
    let password_hash = format!("{:x}", md5::compute(req.password.as_bytes()));

    let id = uuid::Uuid::new_v4().to_string();
    let mobile = req.mobile.clone().unwrap_or_default();
    let email = req.email.clone().unwrap_or_default();

    client
        .execute(
            "INSERT INTO auth_person (id, unique_id, name, mobile, email, password_hash, locked, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6, false, NOW())",
            &[&id, &req.unique_id, &req.name, &mobile, &email, &password_hash],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("uniqueId".to_string(), Value::String(req.unique_id)),
        ("name".to_string(), Value::String(req.name)),
        ("mobile".to_string(), Value::String(mobile)),
        ("email".to_string(), Value::String(email)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 更新人员信息
#[axum::debug_handler]
pub async fn update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<PersonUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let exists = client
        .query_one(
            "SELECT 1 FROM auth_person WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await;

    if exists.is_err() {
        return Ok(Json(ActionResult::error("person not found")));
    }

    let name = req.name.clone().unwrap_or_default();
    let mobile = req.mobile.clone().unwrap_or_default();
    let email = req.email.clone().unwrap_or_default();
    let locked = req.locked.unwrap_or(false);

    client
        .execute(
            "UPDATE auth_person SET name = $1, mobile = $2, email = $3, locked = $4, updated_at = NOW() \
             WHERE id = $5 AND deleted_at IS NULL",
            &[&name, &mobile, &email, &locked, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Null)))
}

/// 软删除人员
pub async fn delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE auth_person SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("person not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Null)))
}
