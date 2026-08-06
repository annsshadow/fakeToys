use axum::Router;

use crate::portal_router;

pub fn router() -> Router {
    portal_router()
}
