use axum::Router;

use crate::message_assemble_communicate_router;

pub fn router() -> Router {
    message_assemble_communicate_router()
}
