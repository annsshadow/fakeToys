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
        group_flag_add_member, group_flag_delete_member, group_flag_mockdeletetoget,
        identity_flag_mockdeletetoget, personcard_createCode_cardId, personcard_createQR_cardId,
        personcard_flag_mockdeletetoget, permissionsetting_flag_mockdeletetoget,
        role_flag_mockdeletetoget, unitduty_flag_mockdeletetoget,
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
}
