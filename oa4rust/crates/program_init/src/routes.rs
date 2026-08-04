use axum::Router;

use crate::{
    check, program_init_router, set, set_cancel
};

pub fn router() -> Router {
    program_init_router()
}
