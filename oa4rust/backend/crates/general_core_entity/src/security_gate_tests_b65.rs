// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 批次 65 安全门禁回归：general_core_entity 的 12 个 sea_orm CRUD 真写处理（dict / dict_item /
//! file / invoice 的 create/update/delete）。这些写操作对审计尺子不可见（尺子只扫 deadpool
//! SQL 字面量），故属于 R7 的"只读"桶，但实为无门禁的已认证写。本批全部加 session 门：
//! `require_session(session)?` 置于首行，无会话即返回 401，DB 写在门之后。
//!
//! 证明方式（离线、不污染 DB）：
//! 1. `b65_writes_reject_without_session`：12 个处理各以 `None` 会话 + 真实 DatabaseConnection
//!    （仅占位，门在 DB 访问前返回 401，绝不落查询）调用，断言 `Err(AppError::Unauthorized)`。
//! 2. `b65_require_session_gate_logic`：纯单元校验 `require_session` —— 无会话拒绝、有会话放行。
//!    无需 DB，避免写副作用。

#[cfg(test)]
mod tests {
    use axum::extract::{Extension, Json, Path};
    use sea_orm::DatabaseConnection;
    use serde_json::Value;
    use shared::{
        error::AppError,
        session::Session,
        testing::{test_sea_orm_pool, test_session},
    };

    use crate::{
        dict_create, dict_delete, dict_item_create, dict_item_delete, dict_item_update,
        dict_update, file_create, file_delete, file_update, invoice_create, invoice_delete,
        invoice_update,
    };

    fn make_db(db: &DatabaseConnection) -> Extension<DatabaseConnection> {
        Extension(db.clone())
    }

    fn empty_json() -> Json<Value> {
        Json(Value::Object(serde_json::Map::new()))
    }

    fn fake_path() -> Path<String> {
        Path("b65-fake-id".to_string())
    }

    #[tokio::test]
    async fn b65_writes_reject_without_session() {
        let db = test_sea_orm_pool()
            .await
            .expect("need a reachable PostgreSQL for the DatabaseConnection placeholder");
        let db_ext = make_db(&db);
        let none: Option<Extension<Session>> = None;

        // dict_create
        let r = dict_create(db_ext.clone(), none.clone(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "dict_create 无会话应 401: r"
        );
        // dict_update
        let r = dict_update(db_ext.clone(), none.clone(), fake_path(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "dict_update 无会话应 401: r"
        );
        // dict_delete
        let r = dict_delete(db_ext.clone(), none.clone(), fake_path()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "dict_delete 无会话应 401: r"
        );
        // dict_item_create
        let r = dict_item_create(db_ext.clone(), none.clone(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "dict_item_create 无会话应 401: r"
        );
        // dict_item_update
        let r = dict_item_update(db_ext.clone(), none.clone(), fake_path(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "dict_item_update 无会话应 401: r"
        );
        // dict_item_delete
        let r = dict_item_delete(db_ext.clone(), none.clone(), fake_path()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "dict_item_delete 无会话应 401: r"
        );
        // file_create
        let r = file_create(db_ext.clone(), none.clone(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "file_create 无会话应 401: r"
        );
        // file_update
        let r = file_update(db_ext.clone(), none.clone(), fake_path(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "file_update 无会话应 401: r"
        );
        // file_delete
        let r = file_delete(db_ext.clone(), none.clone(), fake_path()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "file_delete 无会话应 401: r"
        );
        // invoice_create
        let r = invoice_create(db_ext.clone(), none.clone(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "invoice_create 无会话应 401: r"
        );
        // invoice_update
        let r = invoice_update(db_ext.clone(), none.clone(), fake_path(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "invoice_update 无会话应 401: r"
        );
        // invoice_delete
        let r = invoice_delete(db_ext.clone(), none.clone(), fake_path()).await;
        assert!(
            matches!(r, Err(AppError::Unauthorized)),
            "invoice_delete 无会话应 401: r"
        );
    }

    #[test]
    fn b65_require_session_gate_logic() {
        // 无会话 → 拒绝
        let none: Option<Extension<Session>> = None;
        assert!(crate::require_session(none).is_err());
        // 有会话 → 放行（返回会话本身）
        let some = Some(Extension(test_session("b65-unit")));
        assert!(crate::require_session(some).is_ok());
    }
}
