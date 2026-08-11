use axum::Router;
use deadpool_postgres::Pool;

use crate::organization_assemble_personal_router;

pub fn router(pool: Pool) -> axum::Router {
    organization_assemble_personal_router().layer(axum::extract::Extension(pool))
}
