use axum::Router;

use crate::query_express_router;

pub fn router() -> Router {
    query_express_router()
}
