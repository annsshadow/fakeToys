use axum::{Json, Router, routing::get};
use serde_json::Value;

use shared::{error::AppError, response::ActionResult};

pub fn cms_control_router() -> Router {
    Router::new()
}
