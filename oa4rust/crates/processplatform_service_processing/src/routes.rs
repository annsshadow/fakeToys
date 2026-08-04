use axum::Router;

use crate::processplatform_service_processing_router;

pub fn router() -> Router {
    processplatform_service_processing_router()
}
