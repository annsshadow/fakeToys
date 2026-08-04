use axum::Router;

use crate::query_assemble_designer_router;

pub fn router() -> Router {
    query_assemble_designer_router()
}
