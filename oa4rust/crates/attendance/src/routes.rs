use axum::{
    routing::get,
    Router,
};
use deadpool_postgres::Pool;

pub fn attendance_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/attendance/admin/list/all", get(crate::list_admins))
        .route("/jaxrs/attendance/employee/config/list/all", get(crate::list_employee_configs))
        .route("/jaxrs/attendance/statistical/cycle/list/all", get(crate::list_statistical_cycles))
        .layer(axum::Extension(pool))
}
