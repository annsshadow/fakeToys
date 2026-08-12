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
        let app = crate::router(pool);

        // GET /jaxrs/identity/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/identity/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/export/export/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/export/export/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/export/result/flag/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/export/result/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/export/zhengwudingding/person
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/export/zhengwudingding/person")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/like
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/like")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/like/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/like/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/like/pinyin
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/like/pinyin")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/like/pinyin/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/direct
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/person/test-id/sup/direct")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/person/{personFlag}/sup/nested
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/person/test-id/sup/nested")
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
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/group/list/pinyininitial
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/pinyininitial")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/pinyininitial/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/role/{roleFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/role/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/{flag}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/{flag}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/{flag}/sub/direct
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/sub/direct")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/{flag}/sub/nested
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/sub/nested")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/{flag}/sup/direct
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/sup/direct")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/list/{flag}/sup/nested
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/list/test-id/sup/nested")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id")
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
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/group/{flag}/add/member
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/add/member")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/{flag}/add/member/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/add/member/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/{flag}/delete/member
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/delete/member")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/{flag}/delete/member/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/delete/member/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/{flag}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/mockdeletetoget")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/group/{flag}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/group/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/like
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/like")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/like/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/like/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/like/pinyin
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/like/pinyin")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/like/pinyin/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_4() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/identity/list/person/{personFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/person/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/pinyininitial
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/pinyininitial")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/pinyininitial/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/unit/{unitFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/unit/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/unitduty/name/{unitDutyName}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/unitduty/name/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/{flag}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/{flag}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/list/{flag}/unitduty/name/{unitDutyName}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/list/test-id/unitduty/name/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/{flag}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/test-id/mockdeletetoget")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_5() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/identity/{flag}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/identity/{flag}/order/before/{followFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/identity/test-id/order/before/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/inputperson/result/flag/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/inputperson/result/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/inputperson/template
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/inputperson/template")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/inputperson/wipe
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/inputperson/wipe")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/loginrecord/{stream}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/loginrecord/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/permissionsetting/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/permissionsetting/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/permissionsetting/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/permissionsetting/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/permissionsetting/{flag}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/permissionsetting/test-id/mockdeletetoget")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/permissionsetting/{flag}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/permissionsetting/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_6() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/personattribute/list/person/{personFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/list/person/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personattribute/list/{flag}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personattribute/list/{flag}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personattribute/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personattribute/{flag}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/test-id/mockdeletetoget")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personattribute/{flag}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personattribute/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/createCode/{cardId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/createCode/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/createQR/{cardId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/createQR/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/listPersonalVCf/{idList}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listPersonalVCf/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/listVCf/{idList}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listVCf/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_7() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/personcard/listgrouptypes
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listgrouptypes")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listpaging/page/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/listpaging/page/{page}/size/{size}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listpaging/page/test-id/size/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/{page}/size/{size}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/listpagingwithgroup/page/test-id/size/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/mylist
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/mylist")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/personcard/{flag}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/personcard/test-id/mockdeletetoget")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/group/{groupFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/group/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/like
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/like")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_8() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/role/list/like/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/like/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/like/pinyin
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/like/pinyin")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/like/pinyin/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/person/{personFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/person/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/pinyininitial
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/pinyininitial")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/pinyininitial/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/{flag}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/list/{flag}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/role/{flag}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/test-id/mockdeletetoget")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_9() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/role/{flag}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/role/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unit/list/{flag}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unit/list/{flag}/sub/nested
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/list/test-id/sub/nested")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/list/test-id/sup/nested")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unit/list/{flag}/sup/nested/type/{type}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/list/test-id/sup/nested/type/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unit/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unit/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitattribute/list/unit/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/list/unit/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitattribute/list/{flag}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitattribute/list/{flag}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitattribute/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_10() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/unitattribute/{flag}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/test-id/mockdeletetoget")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitattribute/{flag}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitattribute/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/distinct/name
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/distinct/name")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/distinct/name/like/{key}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/distinct/name/like/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/list/identity/{identityFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/identity/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/list/like
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/like")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/list/name/{name}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/name/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/list/unit/{unitFlag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/unit/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/list/{flag}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/list/{flag}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_routes_batch_11() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // GET /jaxrs/organization/assemble/control/unitduty/update/member
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/update/member")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/{flag}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/test-id/mockdeletetoget")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/organization/assemble/control/unitduty/{flag}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/unitduty/test-id/mockputtopost")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_12() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // POST /jaxrs/organization/assemble/control/person/list/like
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/organization/assemble/control/person/list/like")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}