use axum::{
    extract::Extension,
    Json, Router, routing::get,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod tests_generated;


pub fn organization_assemble_express_router(pool: Pool) -> axum::Router {
    routes::router(pool)
}

#[axum::debug_handler]
pub async fn get_express_config(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM ORG_UNIT WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = row.get("cnt");

    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(true)),
        ("syncInterval".to_string(), Value::Number(serde_json::Number::from(300i64))),
        ("maxRecords".to_string(), Value::Number(serde_json::Number::from(count))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_organization_units(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let rows = client
        .query(
            "SELECT id, name, superior, level FROM ORG_UNIT WHERE deleted_at IS NULL ORDER BY level, name",
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
                ("type".to_string(), row.get::<_, Option<i32>>("level").map(|v| Value::Number(serde_json::Number::from(v))).unwrap_or(Value::Null)),
                ("parent".to_string(), row.get::<_, Option<String>>("superior").map(Value::String).unwrap_or(Value::Null)),
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

#[axum::debug_handler]
pub async fn sync_organization_data(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let client = pool.get().await.map_err(|_| AppError::Internal)?;
    let row = client
        .query_one(
            "SELECT COUNT(*) as cnt FROM ORG_UNIT WHERE deleted_at IS NULL",
            &[],
        )
        .await
        .map_err(|_| AppError::Internal)?;
    let count: i64 = row.get("cnt");

    let data = Value::Object(serde_json::Map::from_iter([
        ("synced".to_string(), Value::Bool(true)),
        ("syncedRecords".to_string(), Value::Number(serde_json::Number::from(count))),
        ("lastSyncTime".to_string(), Value::String("".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn get_express_status(
    pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("status".to_string(), Value::String("running".to_string())),
        ("lastSync".to_string(), Value::String("".to_string())),
        ("errors".to_string(), Value::Number(serde_json::Number::from(0i64))),
        ("warnings".to_string(), Value::Number(serde_json::Number::from(0i64))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::organization_assemble_express_router(pool)
}
