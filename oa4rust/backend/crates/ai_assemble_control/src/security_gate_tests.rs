// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第六十二批）：ai_assemble_control 写面 3 把。
//!
//! - `file_upload`（INSERT `x_ai_file`）、`file_copy_file`（INSERT … SELECT … `x_ai_file`）、
//!   `chat_write_completion_extra`（UPDATE `x_ai_chat`）→ 会话存在档（无会话 401）。
//!
//! `creator` 硬写 `"system"`/按 id 更新，无可用归属列，仅收"必须有效会话"档（admin/owner 收紧记 §五）。
//!
//! DB 依赖臂：`cargo test --lib -p ai_assemble_control -- --ignored`
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests {
    use axum::extract::Extension;
    use axum::Json;
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::Value;
    use shared::error::AppError;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    /// 无会话臂：dead pool + None 会话，3 把必须全 401。
    #[tokio::test]
    async fn b62_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "file_upload",
                crate::file_upload(Extension(pool.clone()), None, None)
                    .await
                    .map(|_| ()),
            ),
            (
                "file_copy_file",
                crate::file_copy_file(Extension(pool.clone()), None, None)
                    .await
                    .map(|_| ()),
            ),
            (
                "chat_write_completion_extra",
                crate::chat_write_completion_extra(
                    Extension(pool.clone()),
                    None,
                    Some(Json(Value::Object(serde_json::Map::from_iter([(
                        "id".to_string(),
                        Value::String("b62-id".to_string()),
                    )])))),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 3, "本批 3 把必须逐把点名");
        let leaked: Vec<String> = cases
            .iter()
            .filter(|(_, r)| !matches!(r, Err(AppError::Unauthorized)))
            .map(|(k, r)| format!("{k} => {:?}", r.as_ref().err()))
            .collect();
        assert!(
            leaked.is_empty(),
            "无会话未回 401（出现 Internal 就是门插在读库之后）：{leaked:?}"
        );
    }

    /// 有会话臂：有效会话可过门（dead pool 下落到写库 Internal，而非 401/403）。
    #[tokio::test]
    async fn b62_writes_accept_valid_session_past_gate() {
        let pool = dead_pool();
        let s = shared::testing::test_session("b62-session");
        let ext = Some(Extension(s));
        // 仅验证"门已通过"：不再 Unauthorized（落到写库 Internal 即证会话档成立）。
        for (name, res) in [
            (
                "file_upload",
                crate::file_upload(Extension(pool.clone()), ext.clone(), None)
                    .await
                    .map(|_| ()),
            ),
            (
                "file_copy_file",
                crate::file_copy_file(Extension(pool.clone()), ext.clone(), None)
                    .await
                    .map(|_| ()),
            ),
            (
                "chat_write_completion_extra",
                crate::chat_write_completion_extra(
                    Extension(pool.clone()),
                    ext.clone(),
                    Some(Json(Value::Object(serde_json::Map::from_iter([(
                        "id".to_string(),
                        Value::String("b62-id".to_string()),
                    )])))),
                )
                .await
                .map(|_| ()),
            ),
        ] {
            assert!(
                !matches!(res, Err(AppError::Unauthorized)),
                "{name} 有效会话应过门（实得 {res:?}）"
            );
        }
    }
}
