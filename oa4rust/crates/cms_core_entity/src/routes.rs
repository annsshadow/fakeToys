use axum::Router;

use crate::{
    app_config_list, app_list, category_ext_list, category_list, cms_core_entity_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::cms_core_entity_router(pool)
}

