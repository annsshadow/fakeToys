#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use deadpool_postgres::{Manager, Pool};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use tower::util::ServiceExt;

    fn build_test_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    #[tokio::test]
    async fn test_get_routes_batch_1() {
        let pool = build_test_pool();
        let app = crate::meeting_assemble_control_router(pool);

        // GET /jaxrs/meeting/assemble/control/building/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/building/list/like/pinyin/{key}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list/like/pinyin/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/building/list/like/{key}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list/like/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/building/list/pinyininitial/{key}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/list/pinyininitial/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/building/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/building/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/config/system/config
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/config/system/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/list/{meetingId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/list/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/applied/completed
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/applied/completed")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/applied/processing
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/applied/processing")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/applied/wait
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/applied/wait")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_2() {
        let pool = build_test_pool();
        let app = crate::meeting_assemble_control_router(pool);

        // GET /jaxrs/meeting/assemble/control/meeting/list/apply/{page}/size/{size}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/apply/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/coming/day/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/coming/day/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/invited/completed
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/invited/completed")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/invited/processing
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/invited/processing")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/invited/rejected
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/invited/rejected")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/invited/wait
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/invited/wait")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/wait/accept
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/wait/accept")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/wait/confirm
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/wait/confirm")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/year/test-id/month/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/year/test-id/month/test-id/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_3() {
        let pool = build_test_pool();
        let app = crate::meeting_assemble_control_router(pool);

        // GET /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/year/test-id/month/test-id/day/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/list/year/{year}/month/{month}/day/{day}/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/list/year/test-id/month/test-id/day/test-id/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/meeting/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/openmeeting/list/room
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/openmeeting/list/room")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/room/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/room/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/meeting/assemble/control/room/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/room/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_4() {
        let pool = build_test_pool();
        let app = crate::meeting_assemble_control_router(pool);

        // POST /jaxrs/meeting/assemble/control/config/system/config/manage
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/config/system/config/manage")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/save/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/accept
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/accept")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/add/invite
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/add/invite")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/checkin
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/checkin")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/confirm/allow
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/confirm/allow")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/confirm/deny
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/confirm/deny")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_5() {
        let pool = build_test_pool();
        let app = crate::meeting_assemble_control_router(pool);

        // POST /jaxrs/meeting/assemble/control/meeting/{id}/delete/invite
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/delete/invite")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/manual/completed
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/manual/completed")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/modify/completedtime
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/modify/completedtime")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/modify/starttime
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/modify/starttime")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/meeting/assemble/control/meeting/{id}/reject
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/meeting/test-id/reject")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_routes_batch_6() {
        let pool = build_test_pool();
        let app = crate::meeting_assemble_control_router(pool);

        // DELETE /jaxrs/meeting/assemble/control/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/meeting/assemble/control/delete/test-id")
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}