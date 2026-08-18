use axum::{
    extract::{Extension, Path},
    Json, Router,
};
use deadpool_postgres::Pool;
use serde::Deserialize;
use serde_json::Value;

use shared::error::AppError;
use shared::response::ActionResult;

pub mod routes;

pub async fn neural_generate_model(
    pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let result = client
        .execute(
            "UPDATE x_query_neural_model SET status = 'generating', update_time = NOW() WHERE flag = $1",
            &[&model_flag],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    if result == 0 {
        return Ok(Json(ActionResult::error("model not found")));
    }

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("generating".to_string(), Value::Bool(true)),
        ]),
    ))))
}

pub async fn neural_list_model(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let rows = client
        .query(
            "SELECT id, name, flag, status, creator, create_time FROM x_query_neural_model WHERE deleted_at IS NULL ORDER BY create_time DESC",
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
                ("flag".to_string(), Value::String(row.get("flag"))),
                ("status".to_string(), Value::String(row.get("status"))),
                ("creator".to_string(), Value::String(row.get("creator"))),
                (
                    "createTime".to_string(),
                    Value::String(row.get::<_, String>("create_time")),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
            ("data".to_string(), Value::Array(data)),
        ]),
    ))))
}

pub fn query_service_router(pool: Pool) -> Router {
    routes::build_router(pool)
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::query_service_router(pool)
}

#[derive(Debug, Deserialize)]
pub struct ProcessingExecuteRequest {
    pub query: Option<String>,
    pub model_flag: Option<String>,
    pub params: Option<Value>,
}

/// Validate user-provided query to reject dangerous SQL-like patterns.
/// Returns true if the query is safe to use.
fn validate_query(query: &str) -> bool {
    let dangerous = [
        "SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "ALTER", "CREATE", "TRUNCATE",
    ];
    !dangerous.iter().any(|d| query.to_uppercase().contains(d))
}

#[axum::debug_handler]
pub async fn processing_execute(
    pool: Extension<Pool>,
    Json(req): Json<ProcessingExecuteRequest>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;

    let query = req.query.unwrap_or_default();
    let model_flag = req.model_flag.unwrap_or_default();
    let params = req.params.unwrap_or_default();
    let params_str = serde_json::to_string(&params).unwrap_or_default();

    if query.trim().is_empty() {
        return Ok(Json(ActionResult::error("query is required")));
    }

    if !validate_query(&query) {
        return Ok(Json(ActionResult::error("query contains disallowed SQL keywords")));
    }

    let creator = "system";

    let id = if model_flag.is_empty() {
        let id = uuid::Uuid::new_v4().to_string();
        client
            .execute(
                "INSERT INTO x_query_processing (id, query, model_flag, params, creator, create_time, update_time) \
                 VALUES ($1, $2, $3, $4, $5, NOW(), NOW())",
                &[&id, &query, &model_flag, &params_str, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;
        id
    } else {
        let id = uuid::Uuid::new_v4().to_string();
        let row = client
            .query_opt(
                "INSERT INTO x_query_processing (id, query, model_flag, params, creator, create_time, update_time) \
                 VALUES ($1, $2, $3, $4, $5, NOW(), NOW()) \
                 ON CONFLICT (model_flag) DO UPDATE SET query = EXCLUDED.query, params = EXCLUDED.params, update_time = NOW() \
                 RETURNING id",
                &[&id, &query, &model_flag, &params_str, &creator],
            )
            .await
            .map_err(|_| AppError::Internal)?;

        // Return the existing id if ON CONFLICT fired, otherwise the new id
        let resolved_id = row.map(|r| r.get::<_, String>("id")).unwrap_or(id);
        resolved_id
    };

    let status = if model_flag.is_empty() {
        "pending".to_string()
    } else {
        let model_result = client
            .execute(
                "UPDATE x_query_neural_model SET status = 'processing', update_time = NOW() WHERE flag = $1",
                &[&model_flag],
            )
            .await
            .map_err(|_| AppError::Internal)?;

        if model_result > 0 {
            "processing".to_string()
        } else {
            "pending".to_string()
        }
    };

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("id".to_string(), Value::String(id)),
            ("query".to_string(), Value::String(query)),
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("status".to_string(), Value::String(status)),
            ("executed".to_string(), Value::Bool(true)),
        ]),
    ))))
}

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;

