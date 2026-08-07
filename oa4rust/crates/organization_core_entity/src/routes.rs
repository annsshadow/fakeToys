use axum::Router;

use crate::{
    bind_list, custom_list, definition_list, group_list, identity_list,
    organization_core_entity_router, person_list,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::organization_core_entity_router(pool)
}

