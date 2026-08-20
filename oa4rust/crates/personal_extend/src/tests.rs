use super::avatar;
use super::personal as personal_mod;
use crate::routes::personal_extend_router;
use auth::SessionManager;
use deadpool_postgres::Pool;
use shared::middleware::extract_token_from_headers;
use shared::response::ActionResult;

#[test]
fn test_extract_bearer_token_valid() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::AUTHORIZATION,
        axum::http::HeaderValue::from_static("Bearer test-token-123"),
    );
    let token = extract_token_from_headers(&headers).unwrap();
    assert_eq!(token, "test-token-123");
}

#[test]
fn test_extract_bearer_token_missing() {
    let headers = axum::http::HeaderMap::new();
    assert!(extract_token_from_headers(&headers).is_none());
}

#[test]
fn test_extract_bearer_token_invalid_format() {
    let mut headers = axum::http::HeaderMap::new();
    headers.insert(
        axum::http::header::AUTHORIZATION,
        axum::http::HeaderValue::from_static("InvalidToken"),
    );
    assert!(extract_token_from_headers(&headers).is_none());
}

#[test]
fn test_action_result_serialization() {
    let result: ActionResult<String> = ActionResult::success("test data".to_string());
    assert_eq!(result.r#type, Some("success".to_string()));
    assert_eq!(result.data, Some("test data".to_string()));
    assert_eq!(result.message, None);

    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "success");
    assert_eq!(json["data"], "test data");
}

#[test]
fn test_action_result_error_serialization() {
    let result: ActionResult<String> = ActionResult::error("something went wrong");
    assert_eq!(result.r#type, Some("error".to_string()));
    assert_eq!(result.message, Some("something went wrong".to_string()));
    assert_eq!(result.data, None);

    let json = serde_json::to_value(&result).unwrap();
    assert_eq!(json["type"], "error");
    assert_eq!(json["message"], "something went wrong");
}

#[test]
fn test_person_info_serialization() {
    let info = personal::PersonInfo {
        id: "1".to_string(),
        unique: "user1".to_string(),
        name: "张三".to_string(),
        mobile: Some("13800138000".to_string()),
        email: Some("zhangsan@example.com".to_string()),
        icon: Some("https://example.com/avatar.jpg".to_string()),
    };

    let json = serde_json::to_value(&info).unwrap();
    assert_eq!(json["name"], "张三");
    assert_eq!(json["mobile"], "13800138000");
    assert_eq!(json["email"], "zhangsan@example.com");
    assert_eq!(json["icon"], "https://example.com/avatar.jpg");
}

#[test]
fn test_get_extension_jpg() {
    assert_eq!(avatar::get_extension("image/jpeg", &None), "jpg");
}

#[test]
fn test_get_extension_png() {
    assert_eq!(avatar::get_extension("image/png", &None), "png");
}

#[test]
fn test_get_extension_from_filename() {
    assert_eq!(
        avatar::get_extension("", &Some("photo.JPG".to_string())),
        "jpg"
    );
}

#[test]
fn test_get_extension_unknown() {
    assert_eq!(
        avatar::get_extension("application/octet-stream", &None),
        "bin"
    );
}

#[test]
fn test_password_change_request_deserialize() {
    let req: password::ChangePasswordRequest =
        serde_json::from_str(r#"{"old_password":"old123","new_password":"new456"}"#).unwrap();
    assert_eq!(req.old_password, "old123");
    assert_eq!(req.new_password, "new456");
}

#[test]
fn test_reset_password_request_deserialize() {
    let req: password::ResetPasswordRequest =
        serde_json::from_str(r#"{"credential":"user1","code":"abc","password":"newpass"}"#)
            .unwrap();
    assert_eq!(req.credential, "user1");
    assert_eq!(req.code, "abc");
    assert_eq!(req.password, "newpass");
}

#[test]
fn test_verify_password_request_deserialize() {
    let req: password::VerifyPasswordRequest =
        serde_json::from_str(r#"{"credential":"user1","password":"pass123"}"#).unwrap();
    assert_eq!(req.credential, "user1");
    assert_eq!(req.password, "pass123");
}

#[test]
fn test_update_personal_request_partial() {
    let req: personal_mod::UpdatePersonalRequest =
        serde_json::from_str(r#"{"name":"李四"}"#).unwrap();
    assert_eq!(req.name, Some("李四".to_string()));
    assert_eq!(req.mobile, None);
    assert_eq!(req.email, None);
}

#[test]
fn test_update_personal_request_full() {
    let req: personal_mod::UpdatePersonalRequest = serde_json::from_str(
        r#"{"name":"李四","mobile":"13900139000","email":"lisi@example.com"}"#,
    )
    .unwrap();
    assert_eq!(req.name, Some("李四".to_string()));
    assert_eq!(req.mobile, Some("13900139000".to_string()));
    assert_eq!(req.email, Some("lisi@example.com".to_string()));
}

#[test]
fn test_avatar_info_serialization() {
    let info = avatar::AvatarInfo {
        id: "avatar-1".to_string(),
        person_unique: "user1".to_string(),
        mime_type: "image/jpeg".to_string(),
        size: 10240,
        url: "/uploads/avatars/avatar-1.jpg".to_string(),
    };

    let json = serde_json::to_value(&info).unwrap();
    assert_eq!(json["mime_type"], "image/jpeg");
    assert_eq!(json["size"], 10240);
    assert_eq!(json["url"], "/uploads/avatars/avatar-1.jpg");
}

#[test]
fn test_router_builds() {
    let pool = Pool::builder(deadpool_postgres::Manager::new(
        deadpool_postgres::tokio_postgres::Config::new(),
        deadpool_postgres::tokio_postgres::NoTls,
    ))
    .build()
    .unwrap();

    let router = personal_extend_router(pool, SessionManager::new());
    // 验证路由成功构建
    let _ = router;
}

#[test]
fn test_session_manager_integration() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let sm = SessionManager::new();
        sm.create_session("test_user".to_string(), "test_token".to_string()).await.unwrap();

        let session = sm.validate_session("test_token").await;
        assert!(session.is_some());
        assert_eq!(session.unwrap().person_unique, "test_user");
    });
}
