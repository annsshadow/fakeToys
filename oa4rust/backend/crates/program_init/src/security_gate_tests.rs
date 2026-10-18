// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 A3-N6 / §二.7）：系统恢复与进程控制写族
//! （`/api/restore/upload*`、`/api/server/execute|stop`、`/api/external_datasources*`）
//! 此前只要求「有会话」，任一登录用户即可写外部数据源 host/port/账密、触发 restore 落盘、
//! 直接停服；`/api/server/execute|stop` 还注册为 GET（csrf 与 requires_admin 都不覆盖 GET）。
//! `/api/secret/set(+cancel)` 保持匿名豁免设计（未初始化引导流程），但「已初始化」判定
//! 原本在查询出错时 `.unwrap_or(false)` fail-open，即 DB 抖动时匿名可重写密钥——已改 fail-closed。
//! 修复后写族仅 admin（无 DB 时 `is_admin` fail-closed → 403）。
#[cfg(test)]
mod security_gate_tests {
    use axum::extract::Extension;
    use shared::error::AppError;

    use crate::{
        external_datasources_set_cancel, restore_upload, restore_upload_cancel, server_execute,
        server_stop,
    };

    #[tokio::test]
    async fn program_init_ops_writes_forbidden_for_non_admin() {
        let pool = shared::testing::test_pool();
        let sess = shared::testing::test_session("u-nonadmin");

        let cases: [(&str, Result<(), AppError>); 5] = [
            (
                "external_datasources_set_cancel",
                external_datasources_set_cancel(Extension(pool.clone()), Extension(sess.clone()))
                    .await
                    .map(|_| ()),
            ),
            (
                "restore_upload",
                restore_upload(
                    Extension(pool.clone()),
                    Extension(sess.clone()),
                    axum::body::Bytes::new(),
                )
                .await
                .map(|_| ()),
            ),
            (
                "restore_upload_cancel",
                restore_upload_cancel(Extension(pool.clone()), Extension(sess.clone()))
                    .await
                    .map(|_| ()),
            ),
            (
                "server_execute",
                server_execute(Extension(pool.clone()), Extension(sess.clone()))
                    .await
                    .map(|_| ()),
            ),
            (
                "server_stop",
                server_stop(Extension(pool.clone()), Extension(sess.clone()))
                    .await
                    .map(|_| ()),
            ),
        ];
        for (name, res) in cases {
            assert!(
                matches!(res, Err(AppError::Forbidden)),
                "{name} 非 admin 必须 403，实得 {res:?}"
            );
        }
    }
}
