use axum::{
    extract::Extension,
    Json,
};
use deadpool_postgres::Pool;
use serde_json::Value;
use shared::{error::AppError, response::ActionResult};

mod routes;

#[axum::debug_handler]
pub async fn state_summary(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("totalProcesses".to_string(), Value::Number(serde_json::Number::from(128))),
        ("running".to_string(), Value::Number(serde_json::Number::from(42))),
        ("completed".to_string(), Value::Number(serde_json::Number::from(81))),
        ("expired".to_string(), Value::Number(serde_json::Number::from(5))),
    ]));
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn state_running(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        ("runningCount".to_string(), Value::Number(serde_json::Number::from(42))),
        (
            "applications".to_string(),
            Value::Array(vec![
                Value::String("OA审批".to_string()),
                Value::String("报销流程".to_string()),
                Value::String("请假流程".to_string()),
            ]),
        ),
    ]));
    Ok(Json(ActionResult::success(data)))
}

#[axum::debug_handler]
pub async fn state_organization(
    _pool: Extension<Pool>,
) -> Result<Json<ActionResult<Value>>, AppError> {
    let data = Value::Object(serde_json::Map::from_iter([
        (
            "organizations".to_string(),
            Value::Array(vec![
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String("org-001".to_string())),
                    ("name".to_string(), Value::String("研发部".to_string())),
                    ("count".to_string(), Value::Number(serde_json::Number::from(15))),
                ])),
                Value::Object(serde_json::Map::from_iter([
                    ("id".to_string(), Value::String("org-002".to_string())),
                    ("name".to_string(), Value::String("产品部".to_string())),
                    ("count".to_string(), Value::Number(serde_json::Number::from(8))),
                ])),
            ]),
        ),
    ]));
    Ok(Json(ActionResult::success(data)))
}

pub fn process_bam_router(pool: Pool) -> axum::Router {
    routes::process_bam_router(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use deadpool_postgres::{Manager, Pool};
    use tower::ServiceExt;

    fn create_test_pool() -> Pool {
        let mut config = tokio_postgres::Config::default();
        config.host("localhost").port(5432).user("postgres").password("postgres").dbname("test");
        let manager = Manager::new(config, tokio_postgres::NoTls);
        Pool::builder(manager).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_process_bam_state_summary() {
        let pool = create_test_pool();
        let app = process_bam_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/state/summary")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("success"));
        assert!(json.get("data").is_some());
        let data = json.get("data").unwrap();
        assert_eq!(data.get("totalProcesses").and_then(|v| v.as_i64()), Some(128));
        assert_eq!(data.get("running").and_then(|v| v.as_i64()), Some(42));
    }

    #[tokio::test]
    async fn test_process_bam_state_running() {
        let pool = create_test_pool();
        let app = process_bam_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/state/running")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("success"));
        assert!(json.get("data").is_some());
        let data = json.get("data").unwrap();
        assert_eq!(data.get("runningCount").and_then(|v| v.as_i64()), Some(42));
        assert_eq!(
            data.get("applications").and_then(|v| v.as_array()).map(|a| a.len()),
            Some(3)
        );
    }

    #[tokio::test]
    async fn test_process_bam_state_organization() {
        let pool = create_test_pool();
        let app = process_bam_router(pool);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/process/state/organization")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json.get("type").and_then(|v| v.as_str()), Some("success"));
        assert!(json.get("data").is_some());
        let data = json.get("data").unwrap();
        let orgs = data.get("organizations").unwrap().as_array().unwrap();
        assert_eq!(orgs.len(), 2);
        assert_eq!(orgs[0].get("name").and_then(|v| v.as_str()), Some("研发部"));
    }
}

pub fn router(pool: deadpool_postgres::Pool) -> axum::Router {
    crate::process_bam_router(pool)
}
