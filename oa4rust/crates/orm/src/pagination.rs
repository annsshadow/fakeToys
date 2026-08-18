//! 游标分页助手（占位）
//!
//! 由于 SeaORM 1.x 的泛型约束复杂性，分页逻辑需要在每个 crate 中具体实现。
//! 参考 `crates/control/src/pagination.rs` 中的现有模式。

use shared::error::AppError;

/// 占位函数 - 实际分页逻辑在各自 crate 中实现
pub async fn cursor_list<E>(_db: &sea_orm::DatabaseConnection, _cursor: &str, _limit: i64, _is_next: bool) -> Result<(i64, Vec<E>, String), AppError>
where
    E: Send,
{
    Ok((0, Vec::new(), "next".to_string()))
}
