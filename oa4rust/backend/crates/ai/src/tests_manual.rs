// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// 手写测试（O14 从 tests_generated.rs 原样迁出；本文件由人工维护，
// 生成器不会覆盖）。新增手写用例请直接加在这里。
#[cfg(test)]
mod tests_manual {
    #![allow(non_snake_case)]

    /// 四轮安全审计 F6 回归：`x_ai_index` 无 creator 列可依据属主，销毁索引条目
    /// 必须是 admin；无 DB 时 `is_admin` fail-closed → 非 admin 会话必须 403。
    #[tokio::test]
    async fn test_index_delete() {
        let result = crate::index_delete(
            axum::extract::Extension(shared::testing::test_pool()),
            axum::extract::Extension(shared::testing::test_session("u-nonadmin")),
            axum::extract::Path("test-id".to_string()),
        )
        .await;
        assert!(
            matches!(result, Err(shared::error::AppError::Forbidden)),
            "index_delete 非 admin 必须 403"
        );
    }
}
