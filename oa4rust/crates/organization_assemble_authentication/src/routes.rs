use axum::Router;
use deadpool_postgres::Pool;

use crate::organization_assemble_authentication_router;

pub fn router(pool: Pool) -> Router {
    organization_assemble_authentication_router().layer(axum::extract::Extension(pool))
}
