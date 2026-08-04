use axum::Router;

use crate::{
    get_portal, list_portal, list_portal_category, portal_router
};

pub fn router() -> Router {
    portal_router()
}
