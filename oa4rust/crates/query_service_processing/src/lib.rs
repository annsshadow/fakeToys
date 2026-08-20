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

    if query_type.is_empty() {
        return Err(AppError::BadRequest("query_type is required".to_string()));
    }

    let row = client
        .query_opt(
            "SELECT id, name, query_type, count FROM x_query WHERE query_type = $1 LIMIT 1",
            &[&query_type],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    match row {
        Some(row) => {
            let count: i64 = row
                .get::<_, Option<String>>("count")
                .and_then(|s| s.parse().ok())
                .unwrap_or(1);
            let processed = count > 0;
            let mut map = serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("queryType".to_string(), Value::String(row.get("query_type"))),
                ("count".to_string(), Value::Number(serde_json::Number::from(count))),
                ("processed".to_string(), Value::Bool(processed)),
            ]);
            if let Some(params) = req.params {
                map.insert("params".to_string(), params);
            }
            let data = Value::Object(map);
            Ok(Json(ActionResult::success(data)))
        }
        None => Ok(Json(ActionResult::error("query type not found"))),
    }
}

/// 批量处理查询请求
/// 批量执行多个查询并返回结果
pub async fn batch_process(
    pool: Extension<Pool>,
    axum::extract::Json(req): Json<BatchQueryRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let queries = req.queries.unwrap_or_default();

    if queries.is_empty() {
        return Err(AppError::BadRequest(
            "queries is required and cannot be empty".to_string(),
        ));
    }

    let mut results = Vec::new();
    for q in queries {
        let query_type = q.query_type.unwrap_or_default();
        if query_type.is_empty() {
            results.push(Value::Object(serde_json::Map::from_iter([
                ("queryType".to_string(), Value::String(String::new())),
                ("processed".to_string(), Value::Bool(false)),
                ("error".to_string(), Value::String("query_type is required".to_string())),
            ])));
            continue;
        }

        let row = client
            .query_opt(
                "SELECT id, name, query_type, count FROM x_query WHERE query_type = $1 LIMIT 1",
                &[&query_type],
            )
            .await
            .map_err(|_| AppError::Internal)?;

        match row {
            Some(row) => {
                let count: i64 = row
                    .get::<_, Option<String>>("count")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                let processed = count > 0;
                results.push(Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String(row.get("id"))),
                    ("name".to_string(), Value::String(row.get("name"))),
                    ("queryType".to_string(), Value::String(row.get("query_type"))),
                    ("count".to_string(), Value::Number(serde_json::Number::from(count))),
                    ("processed".to_string(), Value::Bool(processed)),
                ])));
            }
            None => {
                results.push(Value::Object(serde_json::Map::from_iter([
                    ("queryType".to_string(), Value::String(query_type)),
                    ("processed".to_string(), Value::Bool(false)),
                    ("error".to_string(), Value::String("query type not found".to_string())),
                ])));
            }
        }
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("total".to_string(), Value::Number(serde_json::Number::from(results.len() as i64))),
            ("results".to_string(), Value::Array(results)),
        ]),
    ))))
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

    let active = client
        .query_opt(
            "SELECT count(*) as active FROM pg_stat_activity WHERE datname = current_database()",
            &[],
        )
        .await
        .ok()
        .and_then(|r| r.map(|row| row.get::<_, i64>("active")))
        .unwrap_or(0);

    let queued = client
        .query_opt(
            "SELECT COUNT(*) as queued FROM x_query_processing WHERE create_time > NOW() - INTERVAL '1 hour'",
            &[],
        )
        .await
        .ok()
        .and_then(|r| r.map(|row| row.get::<_, i64>("queued")))
        .unwrap_or(0);

    let data = Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        (
            "activeConnections".to_string(),
            Value::Number(serde_json::Number::from(active)),
        ),
        (
            "queuedRequests".to_string(),
            Value::Number(serde_json::Number::from(queued)),
        ),
        (
            "processedCount".to_string(),
            Value::Number(serde_json::Number::from(count)),
        ),
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

    let cleared_count = client
        .execute(
            "DELETE FROM x_query_processing WHERE create_time > NOW() - INTERVAL '1 hour'",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let reset = client
        .execute(
            "UPDATE x_query SET count = '1', update_time = NOW()",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let now = chrono::Utc::now().naive_utc().format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let data = Value::Object(serde_json::Map::from_iter([
        ("reset".to_string(), Value::Bool(reset > 0)),
        ("resetAt".to_string(), Value::String(now)),
        ("clearedCache".to_string(), Value::Bool(cleared_count > 0)),
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
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    query_service_processing_router(pool)
}
