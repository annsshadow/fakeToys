use axum::{
    extract::Extension,
    Json,
    routing::get,
    Router,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

pub mod forum;
pub mod section;
pub mod subject;

pub fn bbs_router(pool: Pool) -> Router {
    Router::new()
        .route("/jaxrs/bbs/forum/view/all", get(forum::view_all))
        .route("/jaxrs/bbs/section/viewforum/{forumId}", get(section::view_forum))
        .route("/jaxrs/bbs/subject/top/{sectionId}", get(subject::top))
        .layer(Extension(pool))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_action_result_success_serialization() {
        let result: ActionResult<serde_json::Value> =
            ActionResult::success(serde_json::json!({"count": 2, "data": []}));

        let json = serde_json::to_value(&result).unwrap();
        assert_eq!(json["type"], "success");
        assert!(json["data"].is_object());
    }

    #[test]
    fn test_bbs_router_builds() {
        let pool = Pool::builder(deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        ))
        .build()
        .unwrap();

        let _ = bbs_router(pool);
    }

    #[test]
    fn test_forum_handler_returns_error_without_db() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = Pool::builder(deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            ))
            .build()
            .unwrap();

            let result: Result<Json<ActionResult<Value>>, AppError> =
                forum::view_all(Extension(pool)).await;

            match result {
                Ok(_) => panic!("expected error without DB"),
                Err(AppError::Internal) => {}
                Err(_) => panic!("expected Internal error"),
            }
        });
    }
}

pub fn router(_pool: deadpool_postgres::Pool) -> axum::Router {
    axum::Router::new()
        .route("/bbs/health", axum::routing::get(|| async { "TODO: bbs - real implementation needed" }))
}