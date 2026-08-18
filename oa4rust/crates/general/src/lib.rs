mod routes;

use axum::{
    extract::{Extension, Path},
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub async fn area_list(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, level, parent_id FROM district WHERE level = $1 ORDER BY name",
            &[&"province"],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let data: Vec<Value> = rows
        .iter()
        .map(|row| {
            Value::Object(serde_json::Map::from_iter([
                ("id".to_string(), Value::String(row.get("id"))),
                ("name".to_string(), Value::String(row.get("name"))),
                ("level".to_string(), Value::String(row.get("level"))),
                (
                    "\"parentId\"".to_string(),
                    row.get::<_, Option<String>>("parent_id")
                        .map(Value::String)
                        .unwrap_or(Value::Null),
                ),
            ]))
        })
        .collect();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("count".to_string(), Value::Number(serde_json::Number::from(data.len() as i64))),
        ("data".to_string(), Value::Array(data)),
    ])))))
}

pub async fn security_clearance_enable(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM GEN_DICT WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = row.get("cnt");

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("enable".to_string(), Value::Bool(count > 0)),
    ])))))
}

pub async fn is_workday(
    pool: Extension<Pool>,
    axum::extract::Path(date): axum::extract::Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    if date.is_empty() {
        return Ok(Json(ActionResult::error("date is required")));
    }
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_opt(
            "SELECT id FROM GEN_ARA_DISTRICT WHERE deleted_at IS NULL AND id = $1 LIMIT 1",
            &[&date],
        )
        .await
        .map_err(|_| AppError::Internal)?;

    let is_work = row.is_some();

    Ok(Json(ActionResult::success(Value::Object(serde_json::Map::from_iter([
        ("date".to_string(), Value::String(date)),
        ("value".to_string(), Value::Bool(is_work)),
    ])))))
}

pub use routes::general_router;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::general_router(pool)
}
