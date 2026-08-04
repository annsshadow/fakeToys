use axum::Router;

use crate::portal_assemble_designer_router;

pub fn router() -> Router {
    portal_assemble_designer_router()
}
