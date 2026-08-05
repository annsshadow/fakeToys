use axum::{Json, Router, routing::get};
use serde_json::Value;

use shared::{error::AppError, response::ActionResult};

pub async fn hello() -> Result<Json<ActionResult<Value>>, AppError> {
    Ok(Json(ActionResult::success(Value::String("hello".to_string()))))
}

pub fn jpush_router() -> Router {
    Router::new()
        .route("/hello/world", get(hello))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_hello_route() {
        let app = jpush_router();

        let response = app
            .oneshot(Request::get("/hello/world").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/jpush/health", axum::routing::get(|| async { "TODO: jpush - real implementation needed" }))
}