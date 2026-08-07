use axum::{
    extract::{Extension, Path},
    Json, Router,
};
use deadpool_postgres::Pool;
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

#[cfg(test)]
mod tests;
