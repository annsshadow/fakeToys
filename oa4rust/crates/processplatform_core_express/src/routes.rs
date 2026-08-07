use axum::Router;

use crate::{processplatform_core_express_router};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::processplatform_core_express_router(pool)
}

