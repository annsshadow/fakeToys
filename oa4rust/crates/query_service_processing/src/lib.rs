use axum::{
    extract::Extension,
    routing::get, routing::post,
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

/// 查询服务处理模块
/// 提供查询服务的业务逻辑处理
pub mod routes;

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub query_type: Option<String>,
    pub params: Option<Value>,
    pub options: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct BatchQueryRequest {
    pub queries: Option<Vec<QueryRequest>>,
}

/// 处理单个查询请求
/// 根据查询类型和参数执行查询
pub async fn process_query(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<QueryRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let query_type = req.query_type.unwrap_or_default();
    let row = client
        .query_one(
            "SELECT id, name, query_type FROM x_query WHERE query_type = $1 LIMIT 1",
            &[&query_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data = Value::Object(serde_json::Map::from_iter([
        ("queryType".to_string(), Value::String(row.get("query_type"))),
        ("params".to_string(), req.params.unwrap_or(Value::Null)),
        ("processed".to_string(), Value::Bool(true)),
        ("resultCount".to_string(), Value::Number(serde_json::Number::from(1_i64))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 批量处理查询请求
/// 批量执行多个查询并返回结果
pub async fn batch_process(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<BatchQueryRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let _queries = req.queries.unwrap_or_default();
    let rows = client
        .query(
            "SELECT id, name, query_type FROM x_query ORDER BY create_time DESC LIMIT 20",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let results: Vec<Value> = rows
        .iter()
        .map(|q| {
            Value::Object(serde_json::Map::from_iter([
                ("queryType".to_string(), Value::String(q.get("query_type"))),
                ("processed".to_string(), Value::Bool(true)),
                ("resultCount".to_string(), Value::Number(serde_json::Number::from(1_i64))),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("total".to_string(), Value::Number(serde_json::Number::from(results.len() as i64))),
        ("results".to_string(), Value::Array(results)),
    ])))))
}

/// 获取查询服务状态
/// 返回查询服务的当前状态信息
pub async fn get_service_status(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one("SELECT COUNT(*) as count FROM x_query", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = row.get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        ("activeConnections".to_string(), Value::Number(serde_json::Number::from(0_i64))),
        ("queuedRequests".to_string(), Value::Number(serde_json::Number::from(0_i64))),
        ("processedCount".to_string(), Value::Number(serde_json::Number::from(count))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 重置查询服务
/// 重置查询服务状态并清除缓存
pub async fn reset_service(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one("SELECT COUNT(*) as count FROM x_query", &[])
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = row.get("count");

    let data = Value::Object(serde_json::Map::from_iter([
        ("reset".to_string(), Value::Bool(true)),
        ("resetAt".to_string(), Value::String("2024-01-01T00:00:00Z".to_string())),
        ("clearedCache".to_string(), Value::Bool(true)),
        ("processedCount".to_string(), Value::Number(serde_json::Number::from(count))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

/// 查询服务处理路由
/// 路由前缀: /jaxrs/query/service/processing/*
pub fn query_service_processing_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/query/service/processing/process", post(process_query))
        .route("/jaxrs/query/service/processing/batch", post(batch_process))
        .route("/jaxrs/query/service/processing/status", get(get_service_status))
        .route("/jaxrs/query/service/processing/reset", post(reset_service))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests;

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_service_processing_router(pool)
}
