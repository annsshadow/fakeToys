use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

use crate::pagination::page_result;

/// 创建角色请求体（契约路径 POST /jaxrs/role）
#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RoleCreateRequest {
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
}

/// 更新角色请求体（契约路径 PUT /jaxrs/role/{flag}）
#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct RoleUpdateRequest {
    /// 角色名称
    pub name: Option<String>,
    /// 角色描述
    pub description: Option<String>,
    /// 是否禁用（true=禁用，false=启用）
    pub disable: Option<bool>,
}

#[utoipa::path(
    get,
    path = "/jaxrs/role/{flag}",
    params(
        ("flag" = String, Path, description = "Role flag (id or name)")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "control"
)]
pub async fn get(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let row = match client
        .query_one(
            "SELECT id, name, description, disable FROM auth_role \
             WHERE (id = $1 OR name = $1) AND deleted_at IS NULL",
            &[&flag],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return Ok(Json(ActionResult::error("role not found"))),
    };

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
        ("disable".to_string(), Value::Bool(row.get("disable"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 游标分页查询角色；flag 为上一页末条 name（空或 '-' 从头），count 为返回条数
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
                "SELECT id, name, description, disable FROM auth_role \
                 WHERE deleted_at IS NULL AND (name > $1 OR $1 = '' OR $1 = '-') \
                 ORDER BY name ASC LIMIT $2",
                &[&flag, &count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    } else {
        client
            .query(
                "SELECT id, name, description, disable FROM auth_role \
                 WHERE deleted_at IS NULL AND (name < $1 OR $1 = '' OR $1 = '-') \
                 ORDER BY name DESC LIMIT $2",
                &[&flag, &count],
            )
            .await
            .map_err(|_| AppError::Internal)?
    };

    let total: i64 = client
        .query_one("SELECT COUNT(*) as count FROM auth_role WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let mut data: Vec<Value> = rows
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

    if !is_next {
        data.reverse();
    }

    Ok((total, data))
}

#[utoipa::path(
    get,
    path = "/jaxrs/role/list/{flag}/next/{count}",
    params(
        ("flag" = String, Path, description = "Pagination cursor flag"),
        ("count" = i64, Path, description = "Number of items to return")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "control"
)]
pub async fn list_next(
    pool: Extension<Pool>,
    Path((flag, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (total, data) = query_page(&pool, &flag, count, true).await?;
    Ok(page_result(total, data, true))
}

#[utoipa::path(
    get,
    path = "/jaxrs/role/list/{flag}/prev/{count}",
    params(
        ("flag" = String, Path, description = "Pagination cursor flag"),
        ("count" = i64, Path, description = "Number of items to return")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "control"
)]
pub async fn list_prev(
    pool: Extension<Pool>,
    Path((flag, count)): Path<(String, i64)>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let (total, data) = query_page(&pool, &flag, count, false).await?;
    Ok(page_result(total, data, false))
}

#[utoipa::path(
    post,
    path = "/jaxrs/role",
    request_body = RoleCreateRequest,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "control"
)]
pub async fn create(
    pool: Extension<Pool>,
    Json(req): Json<RoleCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.name.trim().is_empty() {
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

#[utoipa::path(
    put,
    path = "/jaxrs/role/{flag}",
    params(
        ("flag" = String, Path, description = "Role flag (id or name)")
    ),
    request_body = RoleUpdateRequest,
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "control"
)]
pub async fn update(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
    Json(req): Json<RoleUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE auth_role SET name = COALESCE($1, name), description = COALESCE($2, description), \
             disable = COALESCE($3, disable), updated_at = NOW() \
             WHERE (id = $4 OR name = $4) AND deleted_at IS NULL",
            &[&req.name, &req.description, &req.disable, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("role not found")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = match client
        .query_one(
            "SELECT id, name, description, disable FROM auth_role WHERE (id = $1 OR name = $1) AND deleted_at IS NULL",
            &[&flag],
        )
        .await
    {
        Ok(row) => row,
        Err(_) => return Ok(Json(ActionResult::error("role not found"))),
    };

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(row.get("id"))),
        ("name".to_string(), Value::String(row.get("name"))),
        ("description".to_string(), Value::String(row.get::<_, Option<String>>("description").unwrap_or_default())),
        ("disable".to_string(), Value::Bool(row.get("disable"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

#[utoipa::path(
    delete,
    path = "/jaxrs/role/{flag}",
    params(
        ("flag" = String, Path, description = "Role flag (id or name)")
    ),
    responses(
        (status = 200, description = "Success", body = serde_json::Value),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    ),
    tag = "control"
)]
pub async fn delete(
    pool: Extension<Pool>,
    Path(flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE auth_role SET deleted_at = NOW() \
             WHERE (id = $1 OR name = $1) AND deleted_at IS NULL",
            &[&flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("role not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}
