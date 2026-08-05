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

#[cfg(test)]
mod tests {
    use super::*;
    use deadpool_postgres::Manager;

    #[tokio::test]
    async fn test_neural_generate_model() {
        let pool = Pool::builder(Manager::new(
            tokio_postgres::Config::new(),
            tokio_postgres::NoTls,
        ))
        .build()
        .unwrap();

        let result = neural_generate_model(Extension(pool), Path("test-model".to_string()))
            .await
            .unwrap();

        let action: ActionResult<serde_json::Value> = result.0;
        assert_eq!(action.r#type, Some("success".to_string()));
        assert!(action.data.is_some());
    }
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/query_service/health", axum::routing::get(|| async { "TODO: query_service - real implementation needed" }))
}