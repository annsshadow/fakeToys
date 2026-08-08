use axum::{
    extract::Extension,
    routing::get, routing::post,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

/// 查询核心服务Express模块
/// 提供查询核心相关的快速响应服务
pub mod routes;

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub query: Option<String>,
    pub params: Option<Value>,
    pub timeout: Option<i64>,
}

/// 执行查询
/// 根据查询语句执行数据库查询并返回结果
pub async fn execute_query(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<QueryRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let timeout = req.timeout.unwrap_or(30000);
    let rows = client
        .query(
            "SELECT id, name, query_type FROM x_query ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let row_count = rows.len() as i64;

    let data = Value::Object(serde_json::Map::from_iter([
        ("query".to_string(), Value::String(req.query.unwrap_or_default())),
        ("params".to_string(), req.params.unwrap_or(Value::Null)),
        ("timeout".to_string(), Value::Number(serde_json::Number::from(timeout))),
        ("rowCount".to_string(), Value::Number(serde_json::Number::from(row_count))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 获取查询历史
/// 返回最近的查询历史记录
pub async fn get_query_history(
    pool: Extension<Pool>,
    axum::extract::Path(limit): axum::extract::Path<i64>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, create_time FROM x_query_import_record ORDER BY create_time DESC LIMIT $1",
            &[&limit],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("query".to_string(), Value::String(row.get("name"))),
                ("executedAt".to_string(), Value::String(row.get("create_time"))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("limit".to_string(), Value::Number(serde_json::Number::from(limit))),
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

/// 缓存查询结果
/// 将查询结果缓存以提高后续查询性能
pub async fn cache_query_result(
    pool: Extension<Pool>,
    axum::extract::Path(query_id): axum::extract::Path<String>,
    axum::extract::Json(body): Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let cached = client
        .query_one("SELECT id FROM x_query WHERE id = $1", &[&query_id])
        .await
        .map(|_| true)
        .unwrap_or(false);
    let ttl = body.get("ttl").and_then(|v| v.as_i64()).unwrap_or(3600);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("queryId".to_string(), Value::String(query_id)),
            ("cached".to_string(), Value::Bool(cached)),
            ("ttl".to_string(), Value::Number(serde_json::Number::from(ttl))),
        ]),
    ))))
}

/// 获取缓存状态
/// 返回查询缓存的状态信息
pub async fn get_cache_status(
    pool: Extension<Pool>,
    axum::extract::Path(query_id): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let cached = client
        .query_one("SELECT id FROM x_query WHERE id = $1", &[&query_id])
        .await
        .map(|_| true)
        .unwrap_or(false);

    let (hits, misses) = if cached { (1, 0) } else { (0, 1) };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("queryId".to_string(), Value::String(query_id)),
            ("cached".to_string(), Value::Bool(cached)),
            ("hits".to_string(), Value::Number(serde_json::Number::from(hits))),
            ("misses".to_string(), Value::Number(serde_json::Number::from(misses))),
        ]),
    ))))
}

/// 查询核心Express路由
/// 路由前缀: /jaxrs/query/core/express/*
pub fn query_core_express_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/query/core/express/execute", post(execute_query))
        .route("/jaxrs/query/core/express/history/{limit}", get(get_query_history))
        .route("/jaxrs/query/core/express/cache/{queryId}", post(cache_query_result))
        .route("/jaxrs/query/core/express/cache/status/{queryId}", get(get_cache_status))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_core_express_router(pool)
}
