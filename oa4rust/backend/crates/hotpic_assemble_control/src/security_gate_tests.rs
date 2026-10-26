// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! hotpic 轮播图/控制配置 = 管理台内容，此前任一登录用户可增删改（creator 恒写
//! "system"）。修复后仅 admin；无 DB 时 is_admin fail-closed → 非 admin 403。
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::{create_hotpic, delete_hotpic, save_hotpic, update_control_config};

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn hotpic_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");

        assert!(
            matches!(
                create_hotpic(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(crate::HotpicRequest {
                        title: Some("t".to_string()),
                        imageUrl: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "create_hotpic 非 admin 必须 403"
        );
        assert!(
            matches!(
                save_hotpic(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("h1".to_string()),
                    axum::Json(crate::HotpicRequest {
                        title: Some("t".to_string()),
                        imageUrl: None,
                    })
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "save_hotpic 非 admin 必须 403"
        );
        assert!(
            matches!(
                delete_hotpic(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::extract::Path("h1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "delete_hotpic 非 admin 必须 403"
        );
        assert!(
            matches!(
                update_control_config(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::Json(serde_json::json!({}))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "update_control_config 非 admin 必须 403"
        );
    }

    // 回归（四轮审计 GET 写面队列 · hotpic 2 把）：user_hotpic_changeTitle 与
    // user_hotpic_id 的 DELETE 分支均按 id 改/删热图，无归属门 ⇒ 收归属门。
    #[tokio::test]
    async fn hotpic_user_writes_require_owner() {
        // 无会话离线臂：changeTitle 401；user_hotpic_id(DELETE) 401。
        let dead = {
            let mgr = deadpool_postgres::Manager::new(
                deadpool_postgres::tokio_postgres::Config::new(),
                deadpool_postgres::tokio_postgres::NoTls,
            );
            deadpool_postgres::Pool::builder(mgr)
                .max_size(1)
                .build()
                .unwrap()
        };
        assert!(
            matches!(
                crate::user_hotpic_changeTitle(
                    Extension(dead.clone()),
                    None,
                    axum::extract::Json(serde_json::json!({"id": "h", "title": "t"}))
                )
                .await,
                Err(AppError::Unauthorized)
            ),
            "changeTitle 无会话必须 401"
        );
        assert!(
            matches!(
                crate::user_hotpic_id(
                    Extension(dead.clone()),
                    None,
                    axum::extract::Path("h".to_string()),
                    axum::http::Method::DELETE
                )
                .await,
                Err(AppError::Unauthorized)
            ),
            "user_hotpic_id DELETE 无会话必须 401"
        );

        if !shared::testing::is_db_available().await {
            eprintln!(
                "skipping hotpic_user_writes_require_owner(real-db arm): DATABASE_URL not reachable"
            );
            return;
        }
        let pool = shared::testing::test_pool();
        const OWNER: &str = "hotpic-gate-owner";
        const OTHER: &str = "hotpic-gate-other";
        const ADMIN: &str = "hotpic-gate-admin";
        const ROLE: &str = "hotpic-gate-role-admin";
        let sess = |p: &str| Some(Extension(shared::testing::test_session(p)));

        // 造 admin + 归属热图行。
        {
            let c = pool.get().await.unwrap();
            c.execute(
                "INSERT INTO auth_role (id, name, disable) VALUES ($1,'admin',false) \
                 ON CONFLICT (id) DO UPDATE SET name='admin', disable=false",
                &[&ROLE.to_string()],
            )
            .await
            .unwrap();
            shared::testing::upsert_fixture_person(&c, ADMIN, ADMIN, "hotpic gate admin").await;
            c.execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
                 VALUES ($1,$2,'hotpic-gate-unit') ON CONFLICT DO NOTHING",
                &[&ADMIN.to_string(), &ROLE.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO x_hotpic (id, application, info_id, title, creator, create_time) \
                 VALUES ('hotpic-gate-1','hotpic-gate-app','hotpic-gate-info','t',$1,NOW()) \
                 ON CONFLICT (id) DO NOTHING",
                &[&OWNER.to_string()],
            )
            .await
            .unwrap();
        }

        // 非属主/非 admin → 403 且不落笔（title 不变、未软删）。
        assert!(
            matches!(
                crate::user_hotpic_changeTitle(
                    Extension(pool.clone()),
                    sess(OTHER),
                    axum::extract::Json(
                        serde_json::json!({"id": "hotpic-gate-1", "title": "hacked"})
                    )
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "changeTitle 他人必须 403"
        );
        assert!(
            matches!(
                crate::user_hotpic_id(
                    Extension(pool.clone()),
                    sess(OTHER),
                    axum::extract::Path("hotpic-gate-1".to_string()),
                    axum::http::Method::DELETE
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "DELETE 他人必须 403"
        );
        {
            let c = pool.get().await.unwrap();
            let t: String = c
                .query_one("SELECT title FROM x_hotpic WHERE id='hotpic-gate-1'", &[])
                .await
                .unwrap()
                .get(0);
            assert_eq!(t, "t", "非属主 403 侧不得改 title");
            let n: i64 = c
                .query_one(
                    "SELECT COUNT(*) FROM x_hotpic WHERE id='hotpic-gate-1' AND deleted_at IS NULL",
                    &[],
                )
                .await
                .unwrap()
                .get(0);
            assert_eq!(n, 1, "非属主 403 侧不得软删");
        }

        // 属主放行：改 title + 软删。
        assert!(
            crate::user_hotpic_changeTitle(
                Extension(pool.clone()),
                sess(OWNER),
                axum::extract::Json(serde_json::json!({"id": "hotpic-gate-1", "title": "owned"}))
            )
            .await
            .is_ok(),
            "属主 changeTitle 应放行"
        );
        assert!(
            crate::user_hotpic_id(
                Extension(pool.clone()),
                sess(OWNER),
                axum::extract::Path("hotpic-gate-1".to_string()),
                axum::http::Method::DELETE
            )
            .await
            .is_ok(),
            "属主 DELETE 应放行"
        );
        {
            let c = pool.get().await.unwrap();
            let n: i64 = c
                .query_one(
                    "SELECT COUNT(*) FROM x_hotpic WHERE id='hotpic-gate-1' AND deleted_at IS NULL",
                    &[],
                )
                .await
                .unwrap()
                .get(0);
            assert_eq!(n, 0, "属主删后应软删");
        }

        // admin 放行（admin 旁路）：再造一行，admin 改 + 删。
        {
            let c = pool.get().await.unwrap();
            c.execute(
                "INSERT INTO x_hotpic (id, application, info_id, title, creator, create_time) \
                 VALUES ('hotpic-gate-2','hotpic-gate-app','hotpic-gate-info','t',$1,NOW()) \
                 ON CONFLICT (id) DO NOTHING",
                &[&OWNER.to_string()],
            )
            .await
            .unwrap();
        }
        assert!(
            crate::user_hotpic_changeTitle(
                Extension(pool.clone()),
                sess(ADMIN),
                axum::extract::Json(serde_json::json!({"id": "hotpic-gate-2", "title": "admin"}))
            )
            .await
            .is_ok(),
            "admin changeTitle 应放行（admin 旁路）"
        );
        assert!(
            crate::user_hotpic_id(
                Extension(pool.clone()),
                sess(ADMIN),
                axum::extract::Path("hotpic-gate-2".to_string()),
                axum::http::Method::DELETE
            )
            .await
            .is_ok(),
            "admin DELETE 应放行（admin 旁路）"
        );

        // 收口：清造数，零残留。
        let c = pool.get().await.unwrap();
        c.execute("DELETE FROM x_hotpic WHERE id LIKE 'hotpic-gate-%'", &[])
            .await
            .unwrap();
        c.execute(
            "DELETE FROM auth_person_role WHERE person_id=$1",
            &[&ADMIN.to_string()],
        )
        .await
        .unwrap();
        c.execute("DELETE FROM auth_person WHERE id=$1", &[&ADMIN.to_string()])
            .await
            .unwrap();
        c.execute("DELETE FROM auth_role WHERE id=$1", &[&ROLE.to_string()])
            .await
            .unwrap();
    }
}
