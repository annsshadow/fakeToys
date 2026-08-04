use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;
use deadpool_postgres::tokio_postgres::types::ToSql;

/// 创建单位请求�?#[derive(Debug, Deserialize)]
pub struct UnitCreateRequest {
    /// 单位名称
    pub name: String,
    /// 父单�?ID（顶级单位传 null�?    pub parent_id: Option<String>,
    /// 层级
    pub level: i32,
}

/// 更新单位请求�?#[derive(Debug, Deserialize)]
pub struct UnitUpdateRequest {
    /// 单位名称
    pub name: Option<String>,
    /// 父单�?ID
    pub parent_id: Option<String>,
    /// 层级
    pub level: Option<i32>,
}

/// 获取单位详情
///
/// 根据 id 查询 auth_unit 表，返回未软删除的单位信息�?///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，单�?ID
pub async fn get(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = client
        .query_one(
            "SELECT id, name, parent_id, level FROM auth_unit WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::NotFound)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("parentId".to_string(), row.get::<_, Option<String>>("parent_id").map(Value::String).unwrap_or(Value::Null)),
        ("level".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("level")))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 获取单位列表（树形结构）
///
/// 查询 auth_unit 表，按层级排序返回所有未软删除的单位，用于前端渲染树形结构�?///
/// # 参数
/// - `pool`: 数据库连接池
pub async fn list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, parent_id, level FROM auth_unit WHERE deleted_at IS NULL ORDER BY level, name",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), row.get::<_, Option<String>>("parent_id").map(Value::String).unwrap_or(Value::Null)),
                ("level".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("level")))),
            ]))
        })
        .collect();

    let result = Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 创建单位
///
/// �?auth_unit 表中插入新记录，需检查名称唯一性�?/// 仅管理员可调用（需权限检查中间件）�?///
/// # 参数
/// - `pool`: 数据库连接池
/// - `req`: 请求体，包含 name、parent_id、level
pub async fn create(
    pool: Extension<Pool>,
    Json(req): Json<UnitCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 检查名称是否已存在
    let existing = client
        .query_one(
            "SELECT 1 FROM auth_unit WHERE name = $1 AND deleted_at IS NULL",
            &[&req.name],
        )
        .await;

    if existing.is_ok() {
        return Ok(Json(ActionResult::error("unit name already exists")));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let parent_id: &str = req.parent_id.as_deref().unwrap_or("");

    client
        .execute(
            "INSERT INTO auth_unit (id, name, parent_id, level, created_at, updated_at) \
             VALUES ($1, $2, $3, $4, NOW(), NOW())",
            &[&id, &req.name, &parent_id, &req.level],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(req.name)),
        ("parentId".to_string(), Value::String(req.parent_id.unwrap_or_default())),
        ("level".to_string(), Value::Number(serde_json::Number::from(req.level))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 更新单位信息
///
/// 更新 auth_unit 表中指定记录�?name/parent_id/level 字段�?/// 仅管理员可调用（需权限检查中间件）�?///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，单�?ID
/// - `req`: 请求体，包含要更新的字段
#[axum::debug_handler]\npub async fn update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<UnitUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 检查记录是否存在且未删�?    let exists = client
        .query_one(
            "SELECT 1 FROM auth_unit WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await;

    if exists.is_err() {
        return Ok(Json(ActionResult::error("unit not found")));
    }

    // 动态构�?UPDATE 语句
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn ToSql + Sync>> = Vec::new();
    let mut idx = 1;

    if let Some(name) = &req.name {
        sets.push(format!("name = ${}", idx));
        params.push(Box::new(name.clone()));
        idx += 1;
    }
    if let Some(parent_id) = &req.parent_id {
        sets.push(format!("parent_id = ${}", idx));
        params.push(Box::new(parent_id.clone()));
        idx += 1;
    }
    if let Some(level) = &req.level {
        sets.push(format!("level = ${}", idx));
        params.push(Box::new(*level));
        idx += 1;
    }
    sets.push("updated_at = NOW()".to_string());

    let set_clause = sets.join(", ");
    let sql = format!(
        "UPDATE auth_unit SET {} WHERE id = ${} AND deleted_at IS NULL",
        set_clause, idx
    );
    params.push(Box::new(id));

    let params_ref: Vec<&(dyn ToSql + Sync)> = params.iter().map(|p| p.as_ref()).collect();
    client
        .execute(&sql, &params_ref)
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Null)))
}

/// 软删除单�?///
/// �?auth_unit 表中指定记录�?deleted_at 设为当前时间，实现软删除�?/// 仅管理员可调用（需权限检查中间件）�?///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，单�?ID
pub async fn delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE auth_unit SET deleted_at = NOW(), updated_at = NOW() \
             WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("unit not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Null)))
}

