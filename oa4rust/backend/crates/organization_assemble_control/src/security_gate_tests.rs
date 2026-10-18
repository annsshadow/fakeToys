// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归测试（2026-10-13 二轮复核）：
//! org 结构写族越权修复——组成员增删、群组/身份/角色/权限配置/人员/单位
//! 软删、职务排序与删除、人员 qr_code/card_code 写、身份排序，均为组织结构
//! 管理操作。此前中间件兜底 Authenticated，任一登录用户即可提权（把自己拉进
//! 高权群组 / 删权限配置）或破坏结构数据。修复后非 admin 会话一律 403
//! （无 DB 时 is_admin fail-closed 返回 false，断言不依赖数据库）。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use axum::extract::Path;
    use shared::error::AppError;

    use crate::{
        export_export_all, export_zhengwudingding_person, group_flag_add_member,
        group_flag_delete_member, group_flag_mockdeletetoget, identity_flag_mockdeletetoget,
        permissionsetting_flag_mockdeletetoget, personcard_createCode_cardId,
        personcard_createQR_cardId, personcard_flag_mockdeletetoget, role_flag_mockdeletetoget,
        unitduty_flag_mockdeletetoget,
    };

    fn build_session(person_unique: &str) -> shared::session::Session {
        shared::testing::test_session(person_unique)
    }

    #[tokio::test]
    async fn org_structure_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-nonadmin");

        assert!(
            matches!(
                group_flag_add_member(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("g1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "group 加成员非 admin 必须 403"
        );
        assert!(
            matches!(
                group_flag_delete_member(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("g1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "group 删成员非 admin 必须 403"
        );
        assert!(
            matches!(
                group_flag_mockdeletetoget(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("g1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "群组软删非 admin 必须 403"
        );
        assert!(
            matches!(
                identity_flag_mockdeletetoget(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("i1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "身份软删非 admin 必须 403"
        );
        assert!(
            matches!(
                permissionsetting_flag_mockdeletetoget(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("p1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "权限配置软删非 admin 必须 403"
        );
        assert!(
            matches!(
                personcard_flag_mockdeletetoget(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("c1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "人员记录软删非 admin 必须 403"
        );
        assert!(
            matches!(
                role_flag_mockdeletetoget(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("r1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "角色软删非 admin 必须 403"
        );
        assert!(
            matches!(
                unitduty_flag_mockdeletetoget(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("d1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "职务删除非 admin 必须 403"
        );
        assert!(
            matches!(
                personcard_createQR_cardId(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("u1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "人员 qr_code 写非 admin 必须 403"
        );
        assert!(
            matches!(
                personcard_createCode_cardId(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    Path("u1".to_string())
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "人员 card_code 写非 admin 必须 403"
        );
    }

    /// 回归（四轮审计 GET 写面队列）：org 的两把导出写面此前不取会话、无门禁，
    /// 任意登录用户可触发全量组织/人员数据导出（x_org_export 建 type='all' /
    /// 'zhengwudingding' 的导出任务）。收 admin 门后：
    /// - 无会话臂（连不上库的 pool）必须 401（出现 Internal 就说明门插在读库之后）；
    /// - 真库臂：非 admin 403 且不落笔；admin 放行且各落 1 行。
    #[tokio::test]
    async fn org_export_writes_require_admin() {
        // 无会话臂：离线 pool，先于任何读库回 401。
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
        let nosess: Vec<(&str, Result<_, AppError>)> = vec![
            (
                "export_export_all",
                export_export_all(Extension(dead.clone()), None).await,
            ),
            (
                "export_zhengwudingding_person",
                export_zhengwudingding_person(Extension(dead.clone()), None).await,
            ),
        ];
        for (name, r) in nosess {
            assert!(
                matches!(r, Err(AppError::Unauthorized)),
                "{name} 无会话必须 401（出现 Internal 即门插在读库之后），实得 {:?}",
                r.as_ref().err()
            );
        }

        if !shared::testing::is_db_available().await {
            eprintln!(
                "skipping org_export_writes_require_admin(real-db arm): DATABASE_URL not reachable"
            );
            return;
        }
        let pool = shared::testing::test_pool();
        // 计数 helper（独立 async fn，避免闭包返回 async 块的借用生命周期问题）。
        async fn cnt(pool: &deadpool_postgres::Pool, ty: &str) -> i64 {
            pool.get()
                .await
                .unwrap()
                .query_one(
                    "SELECT COUNT(*) FROM x_org_export WHERE type = $1",
                    &[&ty.to_string()],
                )
                .await
                .unwrap()
                .get::<_, i64>(0)
        }
        const ADMIN: &str = "org-exp-admin";
        const ROLE: &str = "org-exp-role-admin";
        // 造 admin（对齐 is_admin 口径：unique_id 命中 + admin 角色）。
        {
            let c = pool.get().await.unwrap();
            c.execute(
                "INSERT INTO auth_role (id, name, disable) VALUES ($1, 'admin', false) \
                 ON CONFLICT (id) DO UPDATE SET name = 'admin', disable = false",
                &[&ROLE.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO auth_person (id, unique_id, name, password_hash) \
                 VALUES ($1, $1, 'org export admin', 'x') ON CONFLICT (id) DO NOTHING",
                &[&ADMIN.to_string()],
            )
            .await
            .unwrap();
            c.execute(
                "INSERT INTO auth_person_role (person_id, role_id, unit_id) \
                 VALUES ($1, $2, 'org-exp-unit') ON CONFLICT DO NOTHING",
                &[&ADMIN.to_string(), &ROLE.to_string()],
            )
            .await
            .unwrap();
        }

        // 非 admin：403 且不落笔（type='all' / 'zhengwudingding' 计数不变）。
        let non = build_session("org-exp-nonadmin");
        let before_all = cnt(&pool, "all").await;
        let before_zw = cnt(&pool, "zhengwudingding").await;
        assert!(
            matches!(
                export_export_all(Extension(pool.clone()), Some(Extension(non.clone()))).await,
                Err(AppError::Forbidden)
            ),
            "export_export_all 非 admin 必须 403"
        );
        assert!(
            matches!(
                export_zhengwudingding_person(
                    Extension(pool.clone()),
                    Some(Extension(non.clone()))
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "export_zhengwudingding_person 非 admin 必须 403"
        );
        assert_eq!(
            cnt(&pool, "all").await,
            before_all,
            "非 admin 403 侧不得落笔(all)"
        );
        assert_eq!(
            cnt(&pool, "zhengwudingding").await,
            before_zw,
            "非 admin 403 侧不得落笔(zhengwudingding)"
        );

        // admin：放行且各落 1 行。
        let admin = build_session(ADMIN);
        assert!(
            export_export_all(Extension(pool.clone()), Some(Extension(admin.clone())))
                .await
                .is_ok(),
            "admin export_export_all 应放行"
        );
        assert!(
            export_zhengwudingding_person(Extension(pool.clone()), Some(Extension(admin.clone())))
                .await
                .is_ok(),
            "admin export_zhengwudingding_person 应放行"
        );
        assert_eq!(
            cnt(&pool, "all").await,
            before_all + 1,
            "admin 应落 1 行(all)"
        );
        assert_eq!(
            cnt(&pool, "zhengwudingding").await,
            before_zw + 1,
            "admin 应落 1 行(zhengwudingding)"
        );

        // 收口：清掉本臂落笔与造数，零残留。
        let c = pool.get().await.unwrap();
        c.execute(
            "DELETE FROM x_org_export WHERE type IN ('all','zhengwudingding')",
            &[],
        )
        .await
        .unwrap();
        c.execute(
            "DELETE FROM auth_person_role WHERE person_id = $1",
            &[&ADMIN.to_string()],
        )
        .await
        .unwrap();
        c.execute(
            "DELETE FROM auth_person WHERE id = $1",
            &[&ADMIN.to_string()],
        )
        .await
        .unwrap();
        c.execute("DELETE FROM auth_role WHERE id = $1", &[&ROLE.to_string()])
            .await
            .unwrap();
    }

    /// 第 36 批回归臂：三员（threemember）写面收 admin 门（A3-N9，x_org_person 为组织安全数据）。
    /// 离线：无会话 401（先于读库）；非 admin（is_admin fail-closed）403。
    #[tokio::test]
    async fn threemember_writes_require_admin() {
        let pool = shared::testing::test_pool();
        let sess = build_session("u-threemember-nonadmin");
        let body = || axum::Json(serde_json::json!({}));
        let path = || Path("t1".to_string());

        // 无会话 → 401
        assert!(
            matches!(
                crate::u2_person::threemember_create(Extension(pool.clone()), None, body()).await,
                Err(AppError::Unauthorized)
            ),
            "threemember create 无会话必须 401"
        );
        assert!(
            matches!(
                crate::u2_person::threemember_save(Extension(pool.clone()), None, path(), body())
                    .await,
                Err(AppError::Unauthorized)
            ),
            "threemember save 无会话必须 401"
        );
        assert!(
            matches!(
                crate::u2_person::threemember_delete(Extension(pool.clone()), None, path()).await,
                Err(AppError::Unauthorized)
            ),
            "threemember delete 无会话必须 401"
        );

        // 非 admin → 403
        assert!(
            matches!(
                crate::u2_person::threemember_create(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    body()
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "threemember create 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::u2_person::threemember_save(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    path(),
                    body()
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "threemember save 非 admin 必须 403"
        );
        assert!(
            matches!(
                crate::u2_person::threemember_delete(
                    Extension(pool.clone()),
                    Some(Extension(sess.clone())),
                    path()
                )
                .await,
                Err(AppError::Forbidden)
            ),
            "threemember delete 非 admin 必须 403"
        );
    }
}
