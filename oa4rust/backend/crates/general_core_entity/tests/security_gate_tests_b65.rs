// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
//! R618a 补授权断言：b65 原有 12 写端点仅测无会话 401（src/security_gate_tests_b65.rs）。
//! 本集成测试补：六个 dict/dict_item 写端点对「有会话但非 admin」必须 403（R618a 修复）。
//! 门在首个 DB 写之前，非 admin 分支不落任何写，故不污染连接库。

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

    use general_core_entity::{
        dict_create, dict_delete, dict_item_create, dict_item_delete, dict_item_update, dict_update,
    };

    fn make_db(db: &DatabaseConnection) -> Extension<DatabaseConnection> {
        Extension(db.clone())
    }
    fn empty_json() -> Json<Value> {
        Json(Value::Object(serde_json::Map::new()))
    }
    fn fake_path() -> Path<String> {
        Path("r618a-fake-id".to_string())
    }
    fn nonadmin() -> Option<Extension<Session>> {
        Some(Extension(test_session("r618a-nonadmin")))
    }

    #[tokio::test]
    async fn b65_dict_writes_reject_non_admin() {
        let db = test_sea_orm_pool()
            .await
            .expect("need a reachable PostgreSQL");
        let d = make_db(&db);

        let r = dict_create(d.clone(), nonadmin(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "dict_create 非 admin 应 403"
        );
        let r = dict_update(d.clone(), nonadmin(), fake_path(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "dict_update 非 admin 应 403"
        );
        let r = dict_delete(d.clone(), nonadmin(), fake_path()).await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "dict_delete 非 admin 应 403"
        );
        let r = dict_item_create(d.clone(), nonadmin(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "dict_item_create 非 admin 应 403"
        );
        let r = dict_item_update(d.clone(), nonadmin(), fake_path(), empty_json()).await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "dict_item_update 非 admin 应 403"
        );
        let r = dict_item_delete(d, nonadmin(), fake_path()).await;
        assert!(
            matches!(r, Err(AppError::Forbidden)),
            "dict_item_delete 非 admin 应 403"
        );
    }
}
