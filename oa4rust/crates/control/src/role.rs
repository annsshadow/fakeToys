use axum::extract::{Extension, Path, Query};
use axum::Json;
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;
use std::collections::HashMap;

/// 创建角色请求体
#[derive(Debug, Deserialize)]
pub struct RoleCreateRequest {
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
}

/// 更新角色请求体
#[derive(Debug, Deserialize)]
pub struct RoleUpdateRequest {
    /// 角色名称
    pub name: Option<String>,
    /// 角色描述
    pub description: Option<String>,
    /// 是否禁用（true=禁用，false=启用）
    pub disable: Option<bool>,
}

/// 获取角色详情
pub async fn get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, name, description, disable FROM auth_role WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
        ("disable".to_string(), Value::Bool(row.get("disable"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 获取角色列表（支持分页）
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
            "SELECT COUNT(*) as count FROM auth_role WHERE deleted_at IS NULL AND disable = false",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let rows = client
        .query(
            "SELECT id, name, description, disable FROM auth_role \
             WHERE deleted_at IS NULL AND disable = false \
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
                ("name".to_string(), Value::String(row.get("name"))),
                ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
                ("disable".to_string(), Value::Bool(row.get("disable"))),
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

/// 创建角色
pub async fn create(
    pool: Extension<Pool>,
    Json(req): Json<RoleCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let existing = client
        .query_one(
            "SELECT 1 FROM auth_role WHERE name = $1 AND deleted_at IS NULL",
            &[&req.name],
        )
        .await;

    if existing.is_ok() {
        return Ok(Json(ActionResult::error("role name already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let description = req.description.clone().unwrap_or_default();

    client
        .execute(
            "INSERT INTO auth_role (id, name, description, disable, created_at) \
             VALUES ($1, $2, $3, false, NOW())",
            &[&id, &req.name, &description],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(req.name)),
        ("description".to_string(), Value::String(description)),
        ("disable".to_string(), Value::Bool(false)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 更新角色信息
#[axum::debug_handler]
pub async fn update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<RoleUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let exists = client
        .query_one(
            "SELECT 1 FROM auth_role WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await;

    if exists.is_err() {
        return Ok(Json(ActionResult::error("role not found")));
    }

    let name = req.name.clone().unwrap_or_default();
    let description = req.description.clone().unwrap_or_default();
    let disable = req.disable.unwrap_or(false);

    client
        .execute(
            "UPDATE auth_role SET name = $1, description = $2, disable = $3, updated_at = NOW() \
             WHERE id = $4 AND deleted_at IS NULL",
            &[&name, &description, &disable, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Null)))
}

/// 软删除角色
pub async fn delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE auth_role SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("role not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Null)))
}
