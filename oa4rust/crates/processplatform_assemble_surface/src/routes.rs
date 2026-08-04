use axum::Router;

use crate::processplatform_assemble_surface_router;

pub fn router() -> Router {
    processplatform_assemble_surface_router()
}
