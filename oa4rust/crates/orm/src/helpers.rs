//! 通用 CRUD 助手
//!
//! 由于 SeaORM 1.x 的泛型约束复杂性，这些助手需要提供具体的 Column 类型。
//! 每个 crate 应根据自身实体定义具体的查询函数。

use sea_orm::{EntityTrait, QuerySelect};
use shared::error::AppError;

/// 查询记录总数（过滤已删除）
///
/// 注意：调用方需要传入具体的 Entity 类型。
/// 由于 SeaORM 1.x 的限制，此函数仅返回总数，不执行实际查询。
/// 每个 crate 应根据自己的 Entity 定义具体的 count 查询。
pub async fn count_active<E>(_db: &sea_orm::DatabaseConnection) -> Result<i64, AppError>
where
    E: EntityTrait,
{
    // 占位函数 - 实际使用时需要在调用方传入具体的 Entity 和 Column 类型
    Ok(0)
}
