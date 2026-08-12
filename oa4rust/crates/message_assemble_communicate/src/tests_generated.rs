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

        // GET /jaxrs/message/assemble/communicate/consume/list/{consume}/count/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/list/test-id/count/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/consume/list/{consume}/currentperson/count/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/list/test-id/currentperson/count/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/consume/list/{consume}/person/{person}/count/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/list/test-id/person/test-id/count/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/consume/type/{type}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/type/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/conversation/business/{businessId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/business/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/conversation/list/my
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/list/my")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/conversation/list/with/person
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/list/with/person")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/conversation/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/conversation/{id}/group
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/group")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/conversation/{id}/icon
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/icon")
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

        // GET /jaxrs/message/assemble/communicate/im/conversation/{id}/single
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/single")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/manager/config
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/manager/config")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/msg/collection/list/{page}/size/{size}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/collection/list/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/msg/download/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/download/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/msg/download/{id}/image/width/{width}/height/{height}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/download/test-id/image/width/test-id/height/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/msg/list/object
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/list/object")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/im/msg/list/{page}/size/{size}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/list/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/currentperson/consumed
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/currentperson/consumed")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/currentperson/consumed/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/asc
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/test-id/asc")
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

        // GET /jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/{count}/desc
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/test-id/desc")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/asc
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/count/test-id/asc")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/list/currentperson/count/{count}/desc
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/count/test-id/desc")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/list/currentperson/noim/count/{count}/desc
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/noim/count/test-id/desc")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/asc
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/test-id/asc")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/{count}/desc
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/test-id/desc")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/instant/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/mass/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/mass/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/list/test-id/prev/test-id")
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

        // GET /jaxrs/message/assemble/communicate/mass/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/message/list/paging/{page}/size/{size}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/message/list/paging/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/message/assemble/communicate/receive/{consume}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/receive/test-id")
                    .method(Method::GET)
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
        let app = crate::router(pool);

        // POST /jaxrs/message/assemble/communicate/consume/type/{type}/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/type/test-id/mockputtopost")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/consume/{id}/type/{type}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/test-id/type/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/conversation
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/conversation/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/mockputtopost")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/conversation/{id}/group/quit/self
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/group/quit/self")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/conversation/{id}/read
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/read")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/conversation/{id}/read/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/read/mockputtopost")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/top/cancel")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/conversation/{id}/top/cancel/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/top/cancel/mockputtopost")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/conversation/{id}/top/set
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/top/set")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_6() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // POST /jaxrs/message/assemble/communicate/im/conversation/{id}/top/set/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/top/set/mockputtopost")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/msg
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/msg/clear
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/clear")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/msg/collection
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/collection")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/msg/collection/remove
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/collection/remove")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/msg/revoke/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/revoke/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/im/msg/upload/{conversationId}/type/{type}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/upload/test-id/type/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/mark_read/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mark_read/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/mass/enable/type
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/enable/type")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_7() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // POST /jaxrs/message/assemble/communicate/message/custom/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/message/custom/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/message/assemble/communicate/send
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/send")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_routes_batch_8() {
        let pool = build_test_pool();
        let app = crate::router(pool);

        // DELETE /jaxrs/message/assemble/communicate/im/conversation/{id}/group/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/group/mockdeletetoget")
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // DELETE /jaxrs/message/assemble/communicate/im/conversation/{id}/single/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/single/mockdeletetoget")
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // DELETE /jaxrs/message/assemble/communicate/mass/{id}/mockdeletetoget
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/test-id/mockdeletetoget")
                    .method(Method::DELETE)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}