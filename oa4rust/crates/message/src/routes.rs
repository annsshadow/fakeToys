use axum::Router;

use crate::{
    consume_list, custom_create, message_router, update_single
};

pub fn router() -> Router {
    message_router()
}
