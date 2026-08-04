use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

/// 创建单位请求体
#[derive(Debug, Deserialize)]
pub struct UnitCreateRequest {
    /// 单位名称
    pub name: String,
    /// 父级 ID（顶级单位时为 null）
    pub parent_id: Option<String>,
    /// 级别
    pub level: i32,
}

/// 更新单位请求体
#[derive(Debug, Deserialize)]
pub struct UnitUpdateRequest {
    /// 单位名称
    pub name: Option<String>,
    /// 父级 ID
    pub parent_id: Option<String>,
    /// 级别
    pub level: Option<i32>,
}

/// 获取单位详情
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
        ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
        ("level".to_string(), Value::Number(serde_json::Number::from(row.get("level")))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 获取单位列表（树形结构）
pub async fn list(pool: Extension<Pool>) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, parent_id, level FROM auth_unit \
             WHERE deleted_at IS NULL \
             ORDER BY level, name",
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
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("level".to_string(), Value::Number(serde_json::Number::from(row.get("level")))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Array(data))))
}

/// 创建单位
pub async fn create(
    pool: Extension<Pool>,
    Json(req): Json<UnitCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let id = uuid::Uuid::new_v4().to_string();
    let parent_id = req.parent_id.clone().unwrap_or_default();

    client
        .execute(
            "INSERT INTO auth_unit (id, name, parent_id, level, created_at) \
             VALUES ($1, $2, $3, $4, NOW())",
            &[&id, &req.name, &parent_id, &req.level],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("name".to_string(), Value::String(req.name)),
        ("parentId".to_string(), Value::String(parent_id)),
        ("level".to_string(), Value::Number(serde_json::Number::from(req.level))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 更新单位信息
#[axum::debug_handler]
pub async fn update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<UnitUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let exists = client
        .query_one(
            "SELECT 1 FROM auth_unit WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await;

    if exists.is_err() {
        return Ok(Json(ActionResult::error("unit not found")));
    }

    let name = req.name.clone().unwrap_or_default();
    let parent_id = req.parent_id.clone().unwrap_or_default();
    let level = req.level.unwrap_or(0);

    client
        .execute(
            "UPDATE auth_unit SET name = $1, parent_id = $2, level = $3, updated_at = NOW() \
             WHERE id = $4 AND deleted_at IS NULL",
            &[&name, &parent_id, &level, &id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    Ok(Json(ActionResult::success(Value::Null)))
}

/// 软删除单位
pub async fn delete(
    pool: Extension<Pool>,
    Path(id): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE auth_unit SET deleted_at = NOW() WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("unit not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Null)))
}
