use axum::Router;

use crate::express_router;

pub fn router() -> Router {
    express_router()
}
