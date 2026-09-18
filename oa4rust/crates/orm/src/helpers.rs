//! 通用 CRUD 助手
//!
//! 由于 SeaORM 1.x 的泛型约束复杂性，这些助手需要提供具体的 Column 类型。
//! 每个 crate 应根据自身实体定义具体的查询函数。

use sea_orm::EntityTrait;
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

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::entity::prelude::*;

    // 探针实体：仅为满足泛型约束而生，不参与任何真实查询。
    // DeriveEntityModel 要求结构体名必须为 Model（与 *_core_entity 实体同形）。
    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "orm_probe")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}

    #[tokio::test]
    async fn count_active_is_a_db_free_placeholder_returning_zero() {
        // 契约钉死：count_active 是文档化的占位实现，不触库、不读表。
        // 用已断开的连接做守卫——若将来有人"实现"它并真的发起查询，
        // 此测试会失败，提醒维护者占位契约已被破坏。
        let conn = sea_orm::DatabaseConnection::Disconnected;
        let n = count_active::<Entity>(&conn)
            .await
            .expect("placeholder must not fail");
        assert_eq!(n, 0);
    }
}
