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
        let app = crate::attendance_assemble_control_router(pool);

        // GET /jaxrs/attendance/assemble/control/attendanceadmin/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceadmin/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceadmin/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceadmin/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/filter/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceappealInfo/manager/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/manager/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceappealInfo/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/filter/list")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/topUnit
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/topUnit")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/unit
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/unit")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/user
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/user")
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
        let app = crate::attendance_assemble_control_router(pool);

        // GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/filter/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/filter/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/list/persons/nonesign
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/list/persons/nonesign")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/list/{file_id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/list/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/mobile/filter/list/page/{page}/count/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/mobile/filter/list/page/test-id/count/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/mobile/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/mobile/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancedetail/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceemployeeconfig/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceemployeeconfig/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceemployeeconfig/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceemployeeconfig/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceimportfileinfo/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceimportfileinfo/list/all")
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
        let app = crate::attendance_assemble_control_router(pool);

        // GET /jaxrs/attendance/assemble/control/attendanceimportfileinfo/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceimportfileinfo/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/topUnit/{name}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/topUnit/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/list/unit/{name}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceschedulesetting/list/unit/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceschedulesetting/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceschedulesetting/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceselfholiday/filter/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceselfholiday/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceselfholiday/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceselfholiday/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceselfholiday/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancesetting/code/{code}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancesetting/code/test-id")
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
        let app = crate::attendance_assemble_control_router(pool);

        // GET /jaxrs/attendance/assemble/control/attendancesetting/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancesetting/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancesetting/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancesetting/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/cycleDetail/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancestatisticalcycle/cycleDetail/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancestatisticalcycle/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancestatisticalcycle/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancestatisticalcycle/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancestatisticrequirelog/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancestatisticrequirelog/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendancestatisticrequirelog/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancestatisticrequirelog/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/attendanceworkdayconfig/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/rule/list
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/rule/list")
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
        let app = crate::attendance_assemble_control_router(pool);

        // GET /jaxrs/attendance/assemble/control/selfholidaysimple/docId/{docId}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/selfholidaysimple/docId/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/personMonth/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitDay/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/topUnitMonth/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/test-id/next/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/unitDay/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/next/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/test-id/next/test-id")
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
        let app = crate::attendance_assemble_control_router(pool);

        // GET /jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/{id}/prev/{count}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/filter/unitMonth/list/test-id/prev/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/person/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/person/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/persons/unit/subnested/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/persons/unit/subnested/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/persons/unit/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/persons/unit/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/topUnit/day/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/topUnit/day/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/topUnit/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/topUnit/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/topUnit/{name}/{date}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/unit/day/topUnit/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{date}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/unit/day/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/unit/day/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/unit/day/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/unit/subnested/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/unit/subnested/test-id/test-id/test-id")
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
        let app = crate::attendance_assemble_control_router(pool);

        // GET /jaxrs/attendance/assemble/control/statisticshow/unit/sum/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/unit/sum/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/unit/topUnit/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/unit/topUnit/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/statisticshow/unit/{name}/{year}/{month}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statisticshow/unit/test-id/test-id/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/uuid/random
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/uuid/random")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/workplace/list/all
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/workplace/list/all")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // GET /jaxrs/attendance/assemble/control/workplace/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/workplace/test-id")
                    .method(Method::GET)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_8() {
        let pool = build_test_pool();
        let app = crate::attendance_assemble_control_router(pool);

        // POST /jaxrs/attendance/assemble/control/attendanceappealInfo/appeal/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/appeal/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendanceappealInfo/archive/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/archive/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendanceappealInfo/audit
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/audit")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendanceappealInfo/check
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/check")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/appeal/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/appeal/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceappealInfo/workflow/sync")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/analyse
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/analyse")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/id/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/analyse/id/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/redo
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/analyse/redo")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/analyse/{startDate}/{endDate}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/analyse/test-id/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_9() {
        let pool = build_test_pool();
        let app = crate::attendance_assemble_control_router(pool);

        // POST /jaxrs/attendance/assemble/control/attendancedetail/archive/{id}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/archive/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/{cycleYear}/{cycleMonth}
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/checkDetailWithPersonByCycle/test-id/test-id")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/mobile/mobilepreview
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/mobile/mobilepreview")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/mobile/my
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/mobile/my")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/mobile/recive
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/mobile/recive")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/recive
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/recive")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancedetail/reciveSingle
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancedetail/reciveSingle")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendancesetting/enable/type
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendancesetting/enable/type")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/attendanceworkdayconfig/filter")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
        // POST /jaxrs/attendance/assemble/control/rule/{id}/toggle
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/rule/test-id/toggle")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_post_routes_batch_10() {
        let pool = build_test_pool();
        let app = crate::attendance_assemble_control_router(pool);

        // POST /jaxrs/attendance/assemble/control/statistic/do
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/jaxrs/attendance/assemble/control/statistic/do")
                    .method(Method::POST)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_ne!(response.status(), StatusCode::NOT_FOUND);
    }

}