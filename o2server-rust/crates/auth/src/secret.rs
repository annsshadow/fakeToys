use axum::{
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use shared::response::ActionResult;

pub async fn check(
    axum::extract::Json(payload): axum::extract::Json<serde_json::Value>,
) -> Json<ActionResult<serde_json::Value>> {
    // TODO: Verify password against PostgreSQL
    let result = ActionResult::success(serde_json::json!({
        "passed": true,
        "message": None::<String>
    }));
    Json(result)
}

pub async fn set(
    axum::extract::Json(payload): axum::extract::Json<serde_json::Value>,
) -> Json<ActionResult<serde_json::Value>> {
    // TODO: Update password in PostgreSQL
    let result = ActionResult::success(serde_json::json!({
        "success": true
    }));
    Json(result)
}

pub async fn cancel(
    axum::extract::Json(payload): axum::extract::Json<serde_json::Value>,
) -> Json<ActionResult<serde_json::Value>> {
    // TODO: Cancel password change
    let result = ActionResult::success(serde_json::json!({
        "success": true
    }));
    Json(result)
}

pub fn router() -> Router {
    Router::new()
        .route("/jaxrs/secret/check", post(check))
        .route("/jaxrs/secret/set", post(set))
        .route("/jaxrs/secret/cancel", post(cancel))
}
