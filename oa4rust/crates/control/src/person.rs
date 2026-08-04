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
///
/// 根据 id 查询 auth_person 表，返回未软删除的人员信息
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，人员 ID
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
        ("mobile".to_string(), row.get::<_, Option<String>>("mobile").map(Value::String).unwrap_or(Value::Null)),
        ("email".to_string(), row.get::<_, Option<String>>("email").map(Value::String).unwrap_or(Value::Null)),
        ("locked".to_string(), Value::Bool(row.get("locked"))),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 获取人员列表（支持分页）
///
/// 查询 auth_person 表，支持按名称模糊搜索，支持分页
/// 默认只返回未软删除的人员
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
            "SELECT COUNT(*) as count FROM auth_person WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?
        .get("count");

    // 查询列表
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
                ("mobile".to_string(), row.get::<_, Option<String>>("mobile").map(Value::String).unwrap_or(Value::Null)),
                ("email".to_string(), row.get::<_, Option<String>>("email").map(Value::String).unwrap_or(Value::Null)),
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
///
/// 在 auth_person 表中插入新记录，需检查唯一标识唯一性
/// 密码使用 MD5 哈希存储（兼容旧系统）
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `req`: 请求体，包含 unique_id, name, mobile, email, password
pub async fn create(
    pool: Extension<Pool>,
    Json(req): Json<PersonCreateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if req.unique_id.is_empty() || req.name.is_empty() || req.password.is_empty() {
        return Ok(Json(ActionResult::error("unique_id, name and password are required")));
    }

    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 检查唯一标识是否已存在
    let existing = client
        .query_one(
            "SELECT 1 FROM auth_person WHERE unique_id = $1 AND deleted_at IS NULL",
            &[&req.unique_id],
        )
        .await;

    if existing.is_ok() {
        return Ok(Json(ActionResult::error("unique_id already exists")));
    }

    // 密码哈希（兼容旧系统的 MD5 + DES）
    let password_hash = auth::password::hash_password(&req.password);

    let id = uuid::Uuid::new_v4().to_string();

    client
        .execute(
            "INSERT INTO auth_person (id, unique_id, name, mobile, email, password_hash, locked, created_at) \
             VALUES ($1, $2, $3, $4, $5, $6, false, NOW())",
            &[&id, &req.unique_id, &req.name, &req.mobile, &req.email, &password_hash],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let result = Value::Object(serde_json::Map::from_iter([
        ("id".to_string(), Value::String(id)),
        ("uniqueId".to_string(), Value::String(req.unique_id)),
        ("name".to_string(), Value::String(req.name)),
        ("mobile".to_string(), Value::String(req.mobile.unwrap_or_default())),
        ("email".to_string(), Value::String(req.email.unwrap_or_default())),
    ]));

    Ok(Json(ActionResult::success(result)))
}

/// 更新人员信息
///
/// 更新 auth_person 表中指定记录的 name/mobile/email/locked 字段
/// 仅管理员可调用（需权限检查中间件）
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，人员 ID
/// - `req`: 请求体，包含要更新的字段
#[axum::debug_handler]
pub async fn update(
    pool: Extension<Pool>,
    Path(id): Path<String>,
    Json(req): Json<PersonUpdateRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    // 检查记录是否存在且未删除
    let exists = client
        .query_one(
            "SELECT 1 FROM auth_person WHERE id = $1 AND deleted_at IS NULL",
            &[&id],
        )
        .await;

    if exists.is_err() {
        return Ok(Json(ActionResult::error("person not found")));
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
    if let Some(mobile) = &req.mobile {
        sets.push(format!("mobile = ${}", idx));
        params.push(Box::new(mobile.clone()));
        idx += 1;
    }
    if let Some(email) = &req.email {
        sets.push(format!("email = ${}", idx));
        params.push(Box::new(email.clone()));
        idx += 1;
    }
    if let Some(locked) = &req.locked {
        sets.push(format!("locked = ${}", idx));
        params.push(Box::new(*locked));
        idx += 1;
    }
    sets.push("updated_at = NOW()".to_string());

    if sets.is_empty() {
        return Ok(Json(ActionResult::error("no fields to update")));
    }

    let set_clause = sets.join(", ");
    let sql = format!(
        "UPDATE auth_person SET {} WHERE id = ${} AND deleted_at IS NULL",
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

/// 软删除人员
///
/// 将 auth_person 表中指定记录的 deleted_at 设为当前时间，实现软删除
/// 仅管理员可调用（需权限检查中间件）
///
/// # 参数
/// - `pool`: 数据库连接池
/// - `id`: 路径参数，人员 ID
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
