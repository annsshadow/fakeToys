#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, Method, StatusCode};
    use shared::testing::test_pool;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn test_send_message() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/send")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "send_message route should be registered");
    }

    #[tokio::test]
    async fn test_receive_list() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/receive/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "receive_list route should be registered");
    }

    #[tokio::test]
    async fn test_mark_read() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mark_read/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "mark_read route should be registered");
    }

    #[tokio::test]
    async fn test_consume_list_consume_count_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/list/test-id/count/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "consume_list_consume_count_count route should be registered");
    }

    #[tokio::test]
    async fn test_consume_list_consume_currentperson_count_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/list/test-id/currentperson/count/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "consume_list_consume_currentperson_count_count route should be registered");
    }

    #[tokio::test]
    async fn test_consume_list_consume_person_person_count_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/list/test-id/person/test-id/count/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "consume_list_consume_person_person_count_count route should be registered");
    }

    #[tokio::test]
    async fn test_consume_type_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/type/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "consume_type_type route should be registered");
    }

    #[tokio::test]
    async fn test_consume_type_type_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/type/test-id/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "consume_type_type_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_consume_id_type_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/consume/test-id/type/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "consume_id_type_type route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_business_businessId() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/business/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_business_businessId route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_list_my() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/list/my")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_list_my route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_list_with_person() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/list/with/person")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_list_with_person route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id_group() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/group")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_group route should be registered");
    }

    // SKIPPED: im_conversation_id_group_mockdeletetoget not accessible
    #[tokio::test]
    async fn test_im_conversation_id_group_quit_self() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/group/quit/self")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_group_quit_self route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id_icon() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/icon")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_icon route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id_read() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/read")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_read route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id_read_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/read/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_read_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id_single() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/single")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_single route should be registered");
    }

    // SKIPPED: im_conversation_id_single_mockdeletetoget not accessible
    #[tokio::test]
    async fn test_im_conversation_id_top_cancel() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/top/cancel")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_top_cancel route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id_top_cancel_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/top/cancel/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_top_cancel_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id_top_set() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/top/set")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_top_set route should be registered");
    }

    #[tokio::test]
    async fn test_im_conversation_id_top_set_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/conversation/test-id/top/set/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_conversation_id_top_set_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_im_manager_config() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/manager/config")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_manager_config route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_clear() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/clear")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_clear route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_collection() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/collection")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_collection route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_collection_list_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/collection/list/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_collection_list_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_collection_remove() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/collection/remove")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_collection_remove route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_download_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/download/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_download_id route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_download_id_image_width_width_height_height() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/download/test-id/image/width/test-id/height/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_download_id_image_width_width_height_height route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_list_object() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/list/object")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_list_object route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_list_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/list/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_list_page_size_size route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_revoke_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/revoke/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_revoke_id route should be registered");
    }

    #[tokio::test]
    async fn test_im_msg_upload_conversationId_type_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/im/msg/upload/test-id/type/test-id")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "im_msg_upload_conversationId_type_type route should be registered");
    }

    #[tokio::test]
    async fn test_instant_currentperson_consumed() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/currentperson/consumed")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_currentperson_consumed route should be registered");
    }

    #[tokio::test]
    async fn test_instant_currentperson_consumed_all() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/all")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_currentperson_consumed_all route should be registered");
    }

    #[tokio::test]
    async fn test_instant_currentperson_consumed_mockputtopost() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/currentperson/consumed/mockputtopost")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_currentperson_consumed_mockputtopost route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_consumed_count_count_asc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/test-id/asc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_currentperson_consumed_count_count_asc route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_consumed_count_count_desc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/consumed/count/test-id/desc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_currentperson_consumed_count_count_desc route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_count_count_asc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/count/test-id/asc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_currentperson_count_count_asc route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_count_count_desc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/count/test-id/desc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_currentperson_count_count_desc route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_noim_count_count_desc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/noim/count/test-id/desc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_currentperson_noim_count_count_desc route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_not_consumed_count_count_asc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/test-id/asc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_currentperson_not_consumed_count_count_asc route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_currentperson_not_consumed_count_count_desc() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/currentperson/not/consumed/count/test-id/desc")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_currentperson_not_consumed_count_count_desc route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_instant_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/instant/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "instant_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_mass_enable_type() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/enable/type")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "mass_enable_type route should be registered");
    }

    #[tokio::test]
    async fn test_mass_list_id_next_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/list/test-id/next/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "mass_list_id_next_count route should be registered");
    }

    #[tokio::test]
    async fn test_mass_list_id_prev_count() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/list/test-id/prev/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "mass_list_id_prev_count route should be registered");
    }

    #[tokio::test]
    async fn test_mass_id() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/mass/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "mass_id route should be registered");
    }

    // SKIPPED: mass_id_mockdeletetoget not accessible
    #[tokio::test]
    async fn test_message_custom_create() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/message/custom/create")
                    .method("POST")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "message_custom_create route should be registered");
    }

    #[tokio::test]
    async fn test_message_list_paging_page_size_size() {
        let pool = shared::testing::test_pool();
        let app = crate::router(pool);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/message/assemble/communicate/message/list/paging/test-id/size/test-id")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND,
            "message_list_paging_page_size_size route should be registered");
    }

}