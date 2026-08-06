use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use deadpool_postgres::Pool;

use crate::{
    create_reply, create_topic, get_forum, list_forums, list_topics_by_forum,
    get_control_config, list_control_sections, update_control_config,
};

pub fn router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/assemble/control/config/get", get(get_control_config))
        .route("/jaxrs/bbs/assemble/control/sections", get(list_control_sections))
        .route("/jaxrs/bbs/assemble/control/config/update", get(update_control_config))
        .route("/jaxrs/bbs/assemble/control/forum/list", get(list_forums))
        .route("/jaxrs/bbs/assemble/control/forum/{id}", get(get_forum))
        .route("/jaxrs/bbs/assemble/control/topic/create", post(create_topic))
        .route("/jaxrs/bbs/assemble/control/topic/list/{forumId}", get(list_topics_by_forum))
        .route("/jaxrs/bbs/assemble/control/reply/create", post(create_reply))
        .layer(Extension(pool))
}
