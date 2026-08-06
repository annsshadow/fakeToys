use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod routes;

#[cfg(test)]
mod tests;

pub fn organization_assemble_express_router(pool: Pool) -> axum::Router {
    routes::router(pool)
}

#[axum::debug_handler]
pub async fn get_express_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(true)),
        ("syncInterval".to_string(), Value::Number(serde_json::Number::from(300i64))),
        ("maxRecords".to_string(), Value::Number(serde_json::Number::from(10000i64))),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_organization_units(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let units = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("dept-001".to_string())),
            ("name".to_string(), Value::String("Engineering".to_string())),
            ("type".to_string(), Value::String("department".to_string())),
            ("parent".to_string(), Value::Null),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("dept-002".to_string())),
            ("name".to_string(), Value::String("Product".to_string())),
            ("type".to_string(), Value::String("department".to_string())),
            ("parent".to_string(), Value::String("dept-001".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(units.len() as i64))),
            ("data".to_string(), Value::Array(units)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn sync_organization_data(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("synced".to_string(), Value::Bool(true)),
        ("syncedRecords".to_string(), Value::Number(serde_json::Number::from(0i64))),
        ("lastSyncTime".to_string(), Value::String("".to_string())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn get_express_status(
    _pool: Extension<Pool>,
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
    routes::router(pool)
}