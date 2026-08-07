use axum::Router;

use crate::{
    applications, current_style, modules_all, program_center_router,
};

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    program_center_router().layer(axum::extract::Extension(pool))
}

