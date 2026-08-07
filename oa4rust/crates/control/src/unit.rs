use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

use crate::pagination::page_result;

/// 创建单位请求体（契约路径 POST /jaxrs/unit）
#[derive(Debug, Deserialize)]
pub struct UnitCreateRequest {
    /// 单位名称
    pub name: String,
    /// 父级 ID（顶级单位时为 null）
    pub parent_id: Option<String>,
    /// 级别
    pub level: i32,
}

/// 更新单位请求体（契约路径 PUT /jaxrs/unit/{flag}）
#[derive(Debug, Deserialize)]
pub struct UnitUpdateRequest {
    /// 单位名称
    pub name: Option<String>,
    /// 父级 ID
    pub parent_id: Option<String>,
    /// 级别
    pub level: Option<i32>,
}

/// 获取单位详情：GET /jaxrs/unit/{flag}
pub async fn get(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = match client
        .query_one(
            "SELECT id, name, parent_id, level FROM auth_unit \
             WHERE (id = $1 OR name = $1) AND deleted_at IS NULL",
            &[&flag],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return Ok(Json(ActionResult::error("unit not found"))),
    };

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
        ("level".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("level")))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 获取单位列表（树形结构）：GET /jaxrs/unit/list
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
                ("level".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("level")))),
            ]))
        })
        .collect();

    let total = data.len() as i64;
    let result = Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(total))),
        ("size".to_string(), Value::Number(serde_json::Number::from(total))),
        ("data".to_string(), Value::Array(data)),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 游标分页查询单位；flag 为上一页末条 name（空或 '-' 从头），count 为返回条数
async fn query_page(
    pool: &Extension<Pool>,
    flag: &str,
    count: i64,
    is_next: bool,
) -> Result<(i64, Vec<Value>), AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count = count.clamp(1, 100);

    let rows = if is_next {
        client
            .query(
                "SELECT id, name, parent_id, level FROM auth_unit \
                 WHERE deleted_at IS NULL AND (name > $1 OR $1 = '' OR $1 = '-') \
                 ORDER BY name ASC LIMIT $2",
                &[&flag, &count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, parent_id, level FROM auth_unit \
                 WHERE deleted_at IS NULL AND (name < $1 OR $1 = '' OR $1 = '-') \
                 ORDER BY name DESC LIMIT $2",
                &[&flag, &count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    let total: i64 = client
        .query_one("SELECT COUNT(*) as count FROM auth_unit WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let mut data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
                ("level".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("level")))),
            ]))
        })
        .collect();

    if !is_next {
        data.reverse();
    }

    Ok((total, data))
}

/// 获取单位列表（下一批）：GET /jaxrs/unit/list/{flag}/next/{count}
pub async fn list_next(
    pool: Extension<Pool>,
    Path((flag, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (total, data) = query_page(&pool, &flag, count, true).await?;
    Ok(page_result(total, data, true))
}

/// 获取单位列表（上一批）：GET /jaxrs/unit/list/{flag}/prev/{count}
pub async fn list_prev(
    pool: Extension<Pool>,
    Path((flag, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (total, data) = query_page(&pool, &flag, count, false).await?;
    Ok(page_result(total, data, false))
}

/// 创建单位：POST /jaxrs/unit
pub async fn create(
    pool: Extension<Pool>,
    Json(req): Json<UnitCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
        return Ok(Json(ActionResult::error("name is required")));
    }

    if req.level < 0 || req.level > 99 {
        return Ok(Json(ActionResult::error("level must be between 0 and 99")));
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

/// 更新单位信息：PUT /jaxrs/unit/{flag}
pub async fn update(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Json(req): Json<UnitUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE auth_unit SET name = COALESCE($1, name), parent_id = COALESCE($2, parent_id), \
             level = COALESCE($3, level), updated_at = NOW() \
             WHERE (id = $4 OR name = $4) AND deleted_at IS NULL",
            &[&req.name, &req.parent_id, &req.level, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("unit not found")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = match client
        .query_one(
            "SELECT id, name, parent_id, level FROM auth_unit WHERE (id = $1 OR name = $1) AND deleted_at IS NULL",
            &[&flag],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return Ok(Json(ActionResult::error("unit not found"))),
    };

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("parentId".to_string(), Value::String(row.get::<_, Option<String>>("parent_id").unwrap_or_default())),
        ("level".to_string(), Value::Number(serde_json::Number::from(row.get::<_, i32>("level")))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 软删除单位：DELETE /jaxrs/unit/{flag}
pub async fn delete(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE auth_unit SET deleted_at = NOW() \
             WHERE (id = $1 OR name = $1) AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("unit not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}
