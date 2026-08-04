use axum::Router;

use crate::correlation_service_processing_router;

pub fn router() -> Router {
    correlation_service_processing_router()
}
