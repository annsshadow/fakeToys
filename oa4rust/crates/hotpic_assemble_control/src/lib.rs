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

pub fn hotpic_assemble_control_router(pool: Pool) -> axum::Router {
    routes::router(pool)
}

#[axum::debug_handler]
pub async fn get_control_config(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("enabled".to_string(), Value::Bool(true)),
        ("cacheEnabled".to_string(), Value::Bool(true)),
        ("defaultScale".to_string(), Value::Number(serde_json::Number::from_f64(1.0).unwrap())),
    ]));

    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn list_control_panels(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let panels = vec![
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("heatmap".to_string())),
            ("name".to_string(), Value::String("Heatmap".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
            ("type".to_string(), Value::String("hotpic".to_string())),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("id".to_string(), Value::String("annotation".to_string())),
            ("name".to_string(), Value::String("Annotation".to_string())),
            ("enabled".to_string(), Value::Bool(false)),
            ("type".to_string(), Value::String("hotpic".to_string())),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(panels.len() as i64))),
            ("data".to_string(), Value::Array(panels)),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn update_control_config(
    _pool: Extension<Pool>,
    body: axum::extract::Json<Value>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let config = body.0;
    tracing::info!("Updating hotpic assemble control config: {:?}", config);

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("updated".to_string(), Value::Bool(true)),
            ("config".to_string(), config),
        ]),
    ))))
}

#[axum::debug_handler]
pub async fn list_control_applications(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let applications = vec![
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String("hr".to_string())),
            ("name".to_string(), Value::String("HR Application".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
        Value::Object(serde_json::Map::from_iter([
            ("application".to_string(), Value::String("finance".to_string())),
            ("name".to_string(), Value::String("Finance Application".to_string())),
            ("enabled".to_string(), Value::Bool(true)),
        ])),
    ];

    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("count".to_string(), Value::Number(serde_json::Number::from(applications.len() as i64))),
            ("data".to_string(), Value::Array(applications)),
        ]),
    ))))
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/hotpic_assemble_control/health", axum::routing::get(|| async { "TODO: hotpic_assemble_control - real implementation needed" }))
}