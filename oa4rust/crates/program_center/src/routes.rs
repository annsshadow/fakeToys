use axum::Router;

use crate::{
    applications, current_style, modules_all, program_center_router,
};

pub fn router() -> Router {
    program_center_router()
}
