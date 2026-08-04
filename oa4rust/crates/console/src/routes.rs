use axum::Router;

use crate::console_router;

pub fn router() -> Router {
    console_router()
}
