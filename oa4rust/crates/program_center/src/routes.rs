use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    applications, current_style, modules_all,
    application_create, application_save,
    agent_create, agent_save,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/program/applications", get(applications))
        .route("/jaxrs/program/appstyle/current/style", get(current_style))
        .route("/jaxrs/program/datastructure/modules/all", get(modules_all))
        .route("/jaxrs/program_center/application/create", post(application_create))
        .route("/jaxrs/program_center/application/save/{id}", post(application_save))
        .route("/jaxrs/program_center/agent/create", post(agent_create))
        .route("/jaxrs/program_center/agent/save/{id}", post(agent_save))
        .layer(Extension(pool))
}
