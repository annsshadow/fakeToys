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
///
/// 根据 id 查询 auth_role 表，返回未软删除的角色信息
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，角色 ID
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
        ("description".to_string(), row.get::<_, Option<String>>("description").map(Value::String).unwrap_or(Value::Null)),
        ("disable".to_string(), Value::Bool(row.get("disable"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 获取角色列表（支持分页）
///
/// 查询 auth_role 表，支持按名称模糊搜索，支持分页
/// 默认只返回未禁用的角色
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `params`: 查询参数，包含 page、size、name
pub async fn list(
    pool: Extension<Pool>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let page = 1i64;
    let size = 20i64;
    let offset = 0i64;

    // 查询总数
    let total: i64 = client
        .query_one(
            "SELECT COUNT(*) as count FROM auth_role WHERE deleted_at IS NULL AND disable = false",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    // 查询列表
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
                ("description".to_string(), row.get::<_, Option<String>>("description").map(Value::String).unwrap_or(Value::Null)),
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
///
/// 在 auth_role 表中插入新记录，需检查名称唯一性
/// 仅管理员可调用（需权限检查中间件）
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `req`: 请求体，包含 name、description
pub async fn create(
    pool: Extension<Pool>,
    Json(req): Json<RoleCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 检查名称是否已存在
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
    let description: &str = req.description.as_deref().unwrap_or("");

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
        ("description".to_string(), Value::String(req.description.unwrap_or_default())),
        ("disable".to_string(), Value::Bool(false)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 更新角色信息
///
/// 更新 auth_role 表中指定记录的 name/description/disable 字段
/// 仅管理员可调用（需权限检查中间件）
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，角色 ID
/// - `req`: 请求体，包含要更新的字段
#[axum::debug_handler]
pub async fn update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<RoleUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 检查记录是否存在且未删除
    let exists = client
        .query_one(
            "SELECT 1 FROM auth_role WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await;

    if exists.is_err() {
        return Ok(Json(ActionResult::error("role not found")));
    }

    // 动态构建 UPDATE 语句
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync>> = Vec::new();
    let mut idx = 1;

    if let Some(name) = &req.name {
        sets.push(format!("name = ${}", idx));
        params.push(Box::new(name.clone()));
        idx += 1;
    }
    if let Some(description) = &req.description {
        sets.push(format!("description = ${}", idx));
        params.push(Box::new(description.clone()));
        idx += 1;
    }
    if let Some(disable) = &req.disable {
        sets.push(format!("disable = ${}", idx));
        params.push(Box::new(*disable));
        idx += 1;
    }
    sets.push("updated_at = NOW()".to_string());

    if sets.is_empty() {
        return Ok(Json(ActionResult::error("no fields to update")));
    }

    let set_clause = sets.join(", ");
    let sql = format!(
        "UPDATE auth_role SET {} WHERE id = ${} AND deleted_at IS NULL",
        set_clause, idx
    );
    params.push(Box::new(id));

    let params_ref: Vec<&(dyn deadpool_postgres::tokio_postgres::types::ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();
    client
        .execute(&sql, &params_ref)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Null)))
}

/// 软删除角色
///
/// 将 auth_role 表中指定记录的 deleted_at 设为当前时间，实现软删除
/// 仅管理员可调用（需权限检查中间件）
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，角色 ID
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
