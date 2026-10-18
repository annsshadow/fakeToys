// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 安全审计回归（2026-10 四轮 · 第六十四批）：general_assemble_control 写面 2 把工具写收会话门。
//!
//! - `office_html_to_word`（POST `/office` 与 `/office/html/to/word`，INSERT `x_general_assemble_office`）
//! - `qrcode_width_width_height_height_text_text`（POST `/qrcode/width/.../text/...`，INSERT `x_general_assemble_qrcode`）
//!
//! 二者是按 body/path 参数做按需转换并落库的工具写，非配置/系统数据，按"仅要求有效会话"档收
//! （无会话 401）。`creator` 取请求体/硬编码 `system`，归属门无意义（冒名残留记 §五 待修）。
//!
//! DB 依赖臂：`cargo test --lib -p general_assemble_control -- --ignored`
#[cfg(test)]
#[allow(clippy::module_inception)]
mod security_gate_tests_b64 {
    use axum::extract::{Extension, Json, Path};
    use deadpool_postgres::tokio_postgres::{Config, NoTls};
    use deadpool_postgres::{Manager, Pool};
    use serde_json::json;
    use shared::error::AppError;

    fn dead_pool() -> Pool {
        let mgr = Manager::new(Config::new(), NoTls);
        Pool::builder(mgr).max_size(1).build().unwrap()
    }

    /// 无会话臂：dead pool + None 会话，2 把必须全 401（门在库前，绝不落 Internal）。
    #[tokio::test]
    async fn b64_writes_reject_without_session() {
        let pool = dead_pool();
        let cases: Vec<(&str, Result<(), AppError>)> = vec![
            (
                "office_html_to_word",
                crate::office_html_to_word(
                    Extension(pool.clone()),
                    None,
                    Json(json!({ "htmlContent": "<p>hi</p>", "creator": "system" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "qrcode_width_width_height_height_text_text",
                crate::qrcode_width_width_height_height_text_text(
                    Extension(pool.clone()),
                    None,
                    Path((256u32, 256u32, "hello".to_string())),
                )
                .await
                .map(|_| ()),
            ),
        ];
        assert_eq!(cases.len(), 2, "本批 2 把必须逐把点名");
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

    /// 有会话臂：有效会话可过门（dead pool 下落到写库 Internal，而非 401）。
    #[tokio::test]
    async fn b64_writes_accept_valid_session_past_gate() {
        let pool = dead_pool();
        let s = shared::testing::test_session("b64-session");
        let ext = Some(Extension(s));
        for (name, res) in [
            (
                "office_html_to_word",
                crate::office_html_to_word(
                    Extension(pool.clone()),
                    ext.clone(),
                    Json(json!({ "htmlContent": "<p>hi</p>", "creator": "system" })),
                )
                .await
                .map(|_| ()),
            ),
            (
                "qrcode_width_width_height_height_text_text",
                crate::qrcode_width_width_height_height_text_text(
                    Extension(pool.clone()),
                    ext.clone(),
                    Path((256u32, 256u32, "hello".to_string())),
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
