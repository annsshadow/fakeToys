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
        let app = crate::general_assemble_control_router(pool);

        // GET /jaxrs/general/assemble/control/area/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/area/list/province/{province}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/list/province/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/area/list/province/{province}/city/{city}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/list/province/test-id/city/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/area/list/province/{province}/city/{city}/district/{district}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/list/province/test-id/city/test-id/district/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/area/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/attendscope/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/attendscope/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/attendscope/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/attendscope/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/ecnet/check
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/ecnet/check")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/excel/result/flag/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/excel/result/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/excel/{excelName}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/excel/test-id")
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
        let app = crate::general_assemble_control_router(pool);

        // GET /jaxrs/general/assemble/control/excel/{excelName}/sheetList
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/excel/test-id/sheetList")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/generalfile/download/flag/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/generalfile/download/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/generalfile/flag/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/generalfile/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/generalfile/flag/{flag}/binary/base64
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/generalfile/flag/test-id/binary/base64")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/invoice/download/flag/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/download/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/invoice/get/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/get/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/invoice/list/paging/{page}/size/{size}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/list/paging/test-id/size/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/office/html/to/word/result/flag/{flag}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/office/html/to/word/result/flag/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/permissions/{module}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/permissions/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/qrcode/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/qrcode/list")
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
        let app = crate::general_assemble_control_router(pool);

        // GET /jaxrs/general/assemble/control/qrcode/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/qrcode/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/securityclearance/object
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/object")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/securityclearance/subject
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/subject")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/securityclearance/system
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/system")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/securityclearance/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/status
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/status")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/upgrade/2021090901
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/upgrade/2021090901")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/upgrade/2021090902
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/upgrade/2021090902")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/between/holiday/count/start/{startDate}/end/{endDate}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/between/holiday/count/start/test-id/end/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/between/minutes/start/{start}/end/{end}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/between/minutes/start/test-id/end/test-id")
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
        let app = crate::general_assemble_control_router(pool);

        // GET /jaxrs/general/assemble/control/worktime/forward/days/start/{start}/days/{days}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/forward/days/start/test-id/days/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/forward/minutes/start/{start}/minutes/{minutes}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/forward/minutes/start/test-id/minutes/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/indefined/holiday/{date}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/indefined/holiday/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/indefined/workday/{date}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/indefined/workday/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/is/holiday/{date}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/is/holiday/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/is/workday/{date}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/is/workday/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/is/worktime/{date}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/is/worktime/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/general/assemble/control/worktime/minutes/of/workday
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/worktime/minutes/of/workday")
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
        let app = crate::general_assemble_control_router(pool);

        // POST /jaxrs/general/assemble/control/area/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/area/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/area/update/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/area/update/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/attendscope/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/attendscope/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/attendscope/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/attendscope/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/attendscope/save/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/attendscope/save/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/excel/upload
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/excel/upload")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/excel/upload/with/url
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/excel/upload/with/url")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/invoice/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/invoice/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/delete/test-id")
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
        let app = crate::general_assemble_control_router(pool);

        // POST /jaxrs/general/assemble/control/invoice/update/apply/status/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/update/apply/status/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/invoice/update/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/update/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/invoice/upload
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/upload")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/invoice/upload/for/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/upload/for/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/invoice/upload/with/url
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/invoice/upload/with/url")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/office/html/to/word
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/office/html/to/word")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/qrcode/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/qrcode/delete/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/qrcode/width/{width}/height/{height}/text/{text}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/qrcode/width/test-id/height/test-id/text/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/securityclearance/create
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/create")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/securityclearance/delete/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/delete/test-id")
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
        let app = crate::general_assemble_control_router(pool);

        // POST /jaxrs/general/assemble/control/securityclearance/enable
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/enable")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/securityclearance/update/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/securityclearance/update/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/general/assemble/control/status/update
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/general/assemble/control/status/update")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}