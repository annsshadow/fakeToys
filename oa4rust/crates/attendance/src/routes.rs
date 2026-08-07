use axum::{
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

pub fn attendance_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/attendance/admin/list/all", get(crate::list_admins))
        .route("/jaxrs/attendance/employee/config/list/all", get(crate::list_employee_configs))
        .route("/jaxrs/attendance/statistical/cycle/list/all", get(crate::list_statistical_cycles))
        .route("/jaxrs/attendance/record/list", get(crate::list_check_in_records))
        .route("/jaxrs/attendance/rule/list", get(crate::list_schedule_rules))
        .route("/jaxrs/attendance/appeal/list", get(crate::list_appeal_records))
        .route("/jaxrs/attendance/appeal/submit", post(crate::submit_appeal))
        .route("/jaxrs/attendance/appeal/audit", post(crate::audit_appeal))
        .route("/jaxrs/attendance/appeal/archive/{id}", post(crate::archive_appeal))
        .layer(axum::Extension(pool))
}
