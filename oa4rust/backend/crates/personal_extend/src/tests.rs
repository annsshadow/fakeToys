// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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
    // o2server 成功信封 message 恒为空串（2026-08-25 行为对比结论）
    assert_eq!(result.message, Some(String::new()));

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
    let req: super::password::ChangePasswordRequest =
        serde_json::from_str(r#"{"old_password":"old123","new_password":"new456"}"#).unwrap();
    assert_eq!(req.old_password, "old123");
    assert_eq!(req.new_password, "new456");
}

#[test]
fn test_reset_password_request_deserialize() {
    let req: super::password::ResetPasswordRequest =
        serde_json::from_str(r#"{"credential":"user1","code":"abc","password":"newpass"}"#)
            .unwrap();
    assert_eq!(req.credential, "user1");
    assert_eq!(req.code, "abc");
    assert_eq!(req.password, "newpass");
}

#[test]
fn test_verify_password_request_deserialize() {
    let req: super::password::VerifyPasswordRequest =
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
#[ignore = "requires tokio runtime for SessionManager"]
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
        sm.create_session("test_user".to_string(), "test_token".to_string())
            .await
            .unwrap();

        let session = sm.validate_session("test_token").await;
        assert!(session.is_some());
        assert_eq!(session.unwrap().person_unique, "test_user");
    });
}

// 回归（四轮审计 A3-N9 · crud 归属版）：x_person_face 人脸行的 save/delete 此前只按 id
// 落笔、无归属门 ⇒ 改 crud_save_owned/crud_delete_owned（person_id 属主或 admin）。
#[tokio::test]
async fn face_writes_require_owner() {
    use axum::extract::Extension;
    use shared::error::AppError;
    // 无会话离线臂：两把全 401（先于读库）。
    let dead = {
        let mgr = deadpool_postgres::Manager::new(
            deadpool_postgres::tokio_postgres::Config::new(),
            deadpool_postgres::tokio_postgres::NoTls,
        );
        deadpool_postgres::Pool::builder(mgr).max_size(1).build().unwrap()
    };
    assert!(
        matches!(
            personal_mod::face_save(
                Extension(dead.clone()),
                None,
                axum::extract::Path("f".to_string()),
                axum::Json(serde_json::json!({}))
            )
            .await,
            Err(AppError::Unauthorized)
        ),
        "face_save 无会话必须 401"
    );
    assert!(
        matches!(
            personal_mod::face_delete(
                Extension(dead.clone()),
                None,
                axum::extract::Path("f".to_string())
            )
            .await,
            Err(AppError::Unauthorized)
        ),
        "face_delete 无会话必须 401"
    );
    assert!(
        matches!(
            personal_mod::face_create(
                Extension(dead.clone()),
                None,
                axum::Json(serde_json::json!({}))
            )
            .await,
            Err(AppError::Unauthorized)
        ),
        "face_create 无会话必须 401"
    );

    if !shared::testing::is_db_available().await {
        eprintln!("skipping face_writes_require_owner(real-db arm): DATABASE_URL not reachable");
        return;
    }
    let pool = shared::testing::test_pool();
    const OWNER: &str = "face-gate-owner";
    const OTHER: &str = "face-gate-other";
    const ADMIN: &str = "face-gate-admin";
    const ROLE: &str = "face-gate-role-admin";
    let sess = |p: &str| Some(Extension(shared::testing::test_session(p)));

    // 造 admin + 归属人脸行（person_id=OWNER）。
    {
        let c = pool.get().await.unwrap();
        c.execute(
            "INSERT INTO auth_role (id, name, disable) VALUES ($1,'admin',false) \
             ON CONFLICT (id) DO UPDATE SET name='admin', disable=false",
            &[&ROLE.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person (id, unique_id, name, password_hash) \
             VALUES ($1,$1,'face gate admin','x') ON CONFLICT (id) DO NOTHING",
            &[&ADMIN.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
             VALUES ($1,$2,'face-gate-unit') ON CONFLICT DO NOTHING",
            &[&ADMIN.to_string(), &ROLE.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "INSERT INTO x_person_face (id, person_id) VALUES ('face-gate-1',$1) \
             ON CONFLICT (id) DO NOTHING",
            &[&OWNER.to_string()],
        )
        .await
        .unwrap();
    }

    // 非属主/非 admin → 403 且不落笔。
    assert!(
        matches!(
            personal_mod::face_save(
                Extension(pool.clone()),
                sess(OTHER),
                axum::extract::Path("face-gate-1".to_string()),
                axum::Json(serde_json::json!({"faceName": "hacked"}))
            )
            .await,
            Err(AppError::Forbidden)
        ),
        "face_save 他人必须 403"
    );
    assert!(
        matches!(
            personal_mod::face_delete(
                Extension(pool.clone()),
                sess(OTHER),
                axum::extract::Path("face-gate-1".to_string())
            )
            .await,
            Err(AppError::Forbidden)
        ),
        "face_delete 他人必须 403"
    );
    {
        let c = pool.get().await.unwrap();
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) FROM x_person_face WHERE id='face-gate-1' AND deleted_at IS NULL",
                &[],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(n, 1, "非属主 403 侧不得软删");
    }

    // 属主放行：save + delete。
    assert!(
        personal_mod::face_save(
            Extension(pool.clone()),
            sess(OWNER),
            axum::extract::Path("face-gate-1".to_string()),
            axum::Json(serde_json::json!({"faceName": "owned"}))
        )
        .await
        .is_ok(),
        "属主 face_save 应放行"
    );
    assert!(
        personal_mod::face_delete(
            Extension(pool.clone()),
            sess(OWNER),
            axum::extract::Path("face-gate-1".to_string())
        )
        .await
        .is_ok(),
        "属主 face_delete 应放行"
    );
    {
        let c = pool.get().await.unwrap();
        let n: i64 = c
            .query_one(
                "SELECT COUNT(*) FROM x_person_face WHERE id='face-gate-1' AND deleted_at IS NULL",
                &[],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(n, 0, "属主删后应软删");
    }

    // admin 放行（admin 旁路）：再造一行，admin 删。
    {
        let c = pool.get().await.unwrap();
        c.execute(
            "INSERT INTO x_person_face (id, person_id) VALUES ('face-gate-2',$1) \
             ON CONFLICT (id) DO NOTHING",
            &[&OWNER.to_string()],
        )
        .await
        .unwrap();
    }
    assert!(
        personal_mod::face_delete(
            Extension(pool.clone()),
            sess(ADMIN),
            axum::extract::Path("face-gate-2".to_string())
        )
        .await
        .is_ok(),
        "admin face_delete 应放行（admin 旁路）"
    );

    // face_create 自限定：非 admin 携带他人 personId，落库归属必须被强制为本人。
    {
        let created = personal_mod::face_create(
            Extension(pool.clone()),
            sess(OTHER),
            axum::Json(serde_json::json!({"personId": OWNER, "faceName": "self"})),
        )
        .await
        .expect("非 admin face_create 应放行（自限定为本人）");
        let new_id = created.0.data.as_ref().unwrap()["id"]
            .as_str()
            .unwrap()
            .to_string();
        let c = pool.get().await.unwrap();
        let pid: String = c
            .query_one(
                "SELECT person_id FROM x_person_face WHERE id=$1",
                &[&new_id],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(
            pid, OTHER,
            "非 admin 建档归属必须强制为本人，不得冒他人 personId"
        );
        c.execute("DELETE FROM x_person_face WHERE id=$1", &[&new_id])
            .await
            .unwrap();
    }

    // 收口：清造数，零残留。
    let c = pool.get().await.unwrap();
    c.execute("DELETE FROM x_person_face WHERE id LIKE 'face-gate-%'", &[])
        .await
        .unwrap();
    c.execute("DELETE FROM auth_person_role WHERE person_id=$1", &[&ADMIN.to_string()])
        .await
        .unwrap();
    c.execute("DELETE FROM auth_person WHERE id=$1", &[&ADMIN.to_string()])
        .await
        .unwrap();
    c.execute("DELETE FROM auth_role WHERE id=$1", &[&ROLE.to_string()])
        .await
        .unwrap();
}
