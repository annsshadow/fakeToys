use axum::extract::{Extension, Path};
use axum::Json;
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use shared::error::AppError;
use shared::response::ActionResult;

use crate::pagination::page_result;

/// 创建人员请求体（契约路径 POST /jaxrs/person）
#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
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

/// 更新人员请求体（契约路径 PUT /jaxrs/person/{flag}）
#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
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

/// 按 flag 解析人员（flag 可为 id、unique_id 或 name）
fn person_flag_clause(param_index: usize) -> String {
    format!(
        "((id = ${param_index}) OR (unique_id = ${param_index}) OR (name = ${param_index}))",
        param_index = param_index
    )
}

#[utoipa::path(
    get,
    path = "/jaxrs/person/{flag}",
    params(
        ("flag" = String, Path, description = "Person flag (id, unique_id, or name)")
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

    let where_clause = format!(
        "SELECT id, unique_id, name, mobile, email, locked FROM auth_person WHERE {} AND deleted_at IS NULL",
        person_flag_clause(1)
    );
    let row = match client
        .query_one(&where_clause, &[&flag])
        .await
    {
        Ok(row) => row,
        Err(_) => return Ok(Json(ActionResult::error("person not found"))),
    };

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

/// 游标分页基础查询：按唯一标识建索引导向，next/prev 双向
///
/// flag 为上一页末条/下一条的 unique_id，count 为返回条数。
/// next: 按 unique_id ASC 取大于游标的前 count 条；
/// prev: 按 unique_id DESC 取小于游标的前 count 条（返回前恢复升序）。
async fn query_page(
    pool: &Extension<Pool>,
    flag: &str,
    count: i64,
    is_next: bool,
) -> Result<(i64, Vec<Value>), AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let count = count.clamp(1, 100);

    let (rows, _order_dir) = if is_next {
        let limit = count;
        let rows = client
            .query(
                "SELECT id, unique_id, name, mobile, email, locked FROM auth_person \
                 WHERE deleted_at IS NULL AND (unique_id > $1 OR $1 = '' OR $1 = '-') \
                 ORDER BY unique_id ASC LIMIT $2",
                &[&flag, &limit],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        (rows, ())
    } else {
        let limit = count;
        let rows = client
            .query(
                "SELECT id, unique_id, name, mobile, email, locked FROM auth_person \
                 WHERE deleted_at IS NULL AND (unique_id < $1 OR $1 = '' OR $1 = '-') \
                 ORDER BY unique_id DESC LIMIT $2",
                &[&flag, &limit],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        (rows, ())
    };

    let total: i64 = client
        .query_one("SELECT COUNT(*) as count FROM auth_person WHERE deleted_at IS NULL", &[])
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    let mut data: Vec<Value> = rows
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

    if !is_next {
        data.reverse();
    }

    Ok((total, data))
}

#[utoipa::path(
    get,
    path = "/jaxrs/person/list/{flag}/next/{count}",
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
    path = "/jaxrs/person/list/{flag}/prev/{count}",
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
    path = "/jaxrs/person",
    request_body = PersonCreateRequest,
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
    Json(req): Json<PersonCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.unique_id.trim().is_empty() || req.name.trim().is_empty() || req.password.is_empty() {
        return Ok(Json(ActionResult::error("unique_id, name and password are required")));
    }

    if req.unique_id.len() > 255 || req.name.len() > 255 {
        return Ok(Json(ActionResult::error("unique_id and name must not exceed 255 characters")));
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

    // 密码哈希：统一使用双算法兼容方案（bcrypt + 前缀，兼容既有 MD5/DES 校验）
    let password_hash = auth::password::hash_password(&req.password);

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

#[utoipa::path(
    put,
    path = "/jaxrs/person/{flag}",
    params(
        ("flag" = String, Path, description = "Person flag (id, unique_id, or name)")
    ),
    request_body = PersonUpdateRequest,
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
    Json(req): Json<PersonUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let where_clause = format!(
        "UPDATE auth_person SET name = COALESCE($1, name), mobile = COALESCE($2, mobile), \
         email = COALESCE($3, email), locked = COALESCE($4, locked), updated_at = NOW() \
         WHERE {} AND deleted_at IS NULL",
        person_flag_clause(5)
    );

    let result = client
        .execute(
            &where_clause,
            &[&req.name, &req.mobile, &req.email, &req.locked, &flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("person not found")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let where_clause = format!(
        "SELECT id, unique_id, name, mobile, email, locked FROM auth_person WHERE {} AND deleted_at IS NULL",
        person_flag_clause(1)
    );
    let row = match client.query_one(&where_clause, &[&flag]).await {
        Ok(row) => row,
        Err(_) => return Ok(Json(ActionResult::error("person not found"))),
    };

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

#[utoipa::path(
    delete,
    path = "/jaxrs/person/{flag}",
    params(
        ("flag" = String, Path, description = "Person flag (id, unique_id, or name)")
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

    let where_clause = format!(
        "UPDATE auth_person SET deleted_at = NOW() WHERE {} AND deleted_at IS NULL",
        person_flag_clause(1)
    );

    let result = client
        .execute(&where_clause, &[&flag])
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("person not found or already deleted")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([("deleted".to_string(), Value::Bool(true))]),
    ))))
}
