#[cfg(test)]
mod tests {
    use crate::{EmpowerCreateRequest, EmpowerInfo, EmpowerListResult, EmpowerUpdateRequest};
    use axum::{body::Body, http::Request, Router};
    use auth::SessionManager;
    use shared::{
        middleware::{authorize_middleware, auth_middleware, rate_limit_middleware, SecurityState},
        response::ActionResult,
    };
    use tower::ServiceExt;

    fn build_app(pool: deadpool_postgres::Pool) -> Router {
        let sm = SessionManager::new();
        let state = SecurityState {
            session_manager: sm.clone(),
            rate_limiter: shared::rate_limit::RateLimiter::new(),
            pool: pool.clone(),
        };
        super::super::router::router(pool, sm)
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                authorize_middleware,
            ))
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ))
            .layer(axum::middleware::from_fn_with_state(
                state.clone(),
                rate_limit_middleware,
            ))
    }

    #[test]
    fn test_routes_exist() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let pool = {
                let mgr = deadpool_postgres::Manager::new(
                    deadpool_postgres::tokio_postgres::Config::new(),
                    deadpool_postgres::tokio_postgres::NoTls,
                );
                deadpool_postgres::Pool::builder(mgr).max_size(1).build().unwrap()
            };
            let app = build_app(pool);

            let paths = [
                ("POST", "/jaxrs/person/empower"),
                ("GET", "/jaxrs/person/empower/some-id"),
                ("PUT", "/jaxrs/person/empower/some-id"),
                ("DELETE", "/jaxrs/person/empower/some-id"),
                ("POST", "/jaxrs/person/empower/some-id/enable"),
                ("POST", "/jaxrs/person/empower/some-id/disable"),
                ("POST", "/jaxrs/person/empower/manager"),
                ("PUT", "/jaxrs/person/empower/manager/some-id"),
                ("DELETE", "/jaxrs/person/empower/manager/some-id"),
                ("POST", "/jaxrs/person/empower/manager/list/paging/1/size/10"),
                ("GET", "/jaxrs/person/empower/list/currentperson"),
                ("GET", "/jaxrs/person/empower/list/currentperson/enable"),
                ("GET", "/jaxrs/person/empower/list/to"),
                ("GET", "/jaxrs/person/empower/list/to/enable"),
            ];

            for (method, path) in &paths {
                let req = Request::builder()
                    .method(*method)
                    .uri(path.to_string())
                    .body(Body::empty())
                    .unwrap();
                let resp = app.clone().oneshot(req).await.unwrap();
                let status = resp.status();
                assert_ne!(
                    status, 404,
                    "Route {} {} should exist (got {})",
                    method, path, status
                );
            }
        });
    }

    #[test]
    fn test_create_request_deserialize() {
        let req: EmpowerCreateRequest =
            serde_json::from_str(r#"{"to_person": "user-2", "role_id": "role-1"}"#).unwrap();
        assert_eq!(req.to_person, "user-2");
        assert_eq!(req.role_id, Some("role-1".to_string()));

        let req2: EmpowerCreateRequest =
            serde_json::from_str(r#"{"to_person": "user-2"}"#).unwrap();
        assert_eq!(req2.to_person, "user-2");
        assert!(req2.role_id.is_none());
    }

    #[test]
    fn test_update_request_deserialize() {
        let req: EmpowerUpdateRequest =
            serde_json::from_str(r#"{"role_id": "role-2", "enabled": false}"#).unwrap();
        assert_eq!(req.role_id, Some("role-2".to_string()));
        assert_eq!(req.enabled, Some(false));

        let req2: EmpowerUpdateRequest = serde_json::from_str(r#"{}"#).unwrap();
        assert!(req2.role_id.is_none());
        assert!(req2.enabled.is_none());
    }

    #[test]
    fn test_action_result_empower_serialization() {
        let info = EmpowerInfo {
            id: "emp-1".to_string(),
            from_person: "user-a".to_string(),
            to_person: "user-b".to_string(),
            role_id: Some("role-1".to_string()),
            enabled: true,
            created_at: None,
            updated_at: None,
        };
        let result: ActionResult<EmpowerInfo> = ActionResult::success(info);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        assert_eq!(result.data.as_ref().unwrap().id, "emp-1");
    }

    #[test]
    fn test_list_result_serialization() {
        let items = vec![];
        let result: ActionResult<Vec<EmpowerInfo>> = ActionResult::java_success(items, 2, 0);
        assert_eq!(result.r#type, Some("success".to_string()));
        assert!(result.data.is_some());
        assert_eq!(result.count, Some(2));
        assert_eq!(result.size, Some(0));
    }
}
