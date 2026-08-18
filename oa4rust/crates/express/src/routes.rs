use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    express_group_list, express_identity_list, express_person_list, express_person_with_identity,
    express_person_with_unit, express_role_list, express_unit_list, get_express_info,
    list_express_companies, subscribe_express,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/express/query", get(get_express_info))
        .route("/jaxrs/express/companies", get(list_express_companies))
        .route("/jaxrs/express/subscribe", post(subscribe_express))
        // Batch query endpoints (R17-R24, no authentication required)
        .route("/jaxrs/express/person/list", post(express_person_list))
        .route("/jaxrs/express/unit/list", post(express_unit_list))
        .route("/jaxrs/express/identity/list", post(express_identity_list))
        .route("/jaxrs/express/group/list", post(express_group_list))
        .route("/jaxrs/express/role/list", post(express_role_list))
        .route("/jaxrs/express/person/with/unit", post(express_person_with_unit))
        .route(
            "/jaxrs/express/person/with/identity",
            post(express_person_with_identity),
        )
        .layer(Extension(pool))
}
