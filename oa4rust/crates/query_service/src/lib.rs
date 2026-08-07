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
    _pool: Extension<Pool>,
    Path(model_flag): Path<String>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::Object(
        serde_json::Map::from_iter([
            ("modelFlag".to_string(), Value::String(model_flag)),
            ("generating".to_string(), Value::Bool(true)),
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
