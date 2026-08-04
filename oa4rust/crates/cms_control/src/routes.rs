use axum::Router;

use crate::cms_control_router;

pub fn router() -> Router {
    cms_control_router()
}
