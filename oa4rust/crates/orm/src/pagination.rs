//! 游标分页助手（占位）
//!
//! 由于 SeaORM 1.x 的泛型约束复杂性，分页逻辑需要在每个 crate 中具体实现。
//! 参考 `crates/control/src/pagination.rs` 中的现有模式。

use shared::error::AppError;

/// 占位函数 - 实际分页逻辑在各自 crate 中实现
pub async fn cursor_list<E>(
    _db: &sea_orm::DatabaseConnection,
    _cursor: &str,
    _limit: i64,
    _is_next: bool,
) -> Result<(i64, Vec<E>, String), AppError>
where
    E: Send,
{
    Ok((0, Vec::new(), "next".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn cursor_list_is_a_db_free_placeholder_returning_empty_page() {
        // 契约钉死：占位实现返回 (0, [], "next") 且完全不使用 db/cursor/limit 入参。
        // 用断开的连接做守卫：任何试图真实读库的"实现"都会在此暴露。
        let conn = sea_orm::DatabaseConnection::Disconnected;
        let (total, items, cursor) = cursor_list::<i32>(&conn, "abc", 20, false)
            .await
            .expect("placeholder must not fail");
        assert_eq!(total, 0);
        assert!(items.is_empty());
        assert_eq!(cursor, "next");
    }
}
