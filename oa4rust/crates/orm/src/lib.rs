//! SeaORM 共享层
//!
//! 提供数据库连接池管理。
//!
//! ## 使用方式
//!
//! 1. 在 main.rs 中创建 SeaORM 连接池：
//! ```ignore
//! use orm::create_sea_orm_pool;
//! let sea_orm_db = create_sea_orm_pool().await?;
//! ```
//!
//! 2. 在每个 crate 中定义 SeaORM 实体：
//! ```ignore
//! use sea_orm::entity::prelude::*;
//!
//! #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
//! #[sea_orm(table_name = "auth_person")]
//! pub struct Model {
//!     #[sea_orm(primary_key)]
//!     pub id: String,
//!     pub name: String,
//!     pub deleted_at: Option<DateTime<Utc>>,
//! }
//!
//! impl EntityName for Entity {
//!     fn table_name(&self) -> sea_query::DynIden {
//!         sea_query::iden!("auth_person")
//!     }
//! }
//! ```
//!
//! 3. 在 handler 中使用：
//! ```ignore
//! pub async fn list(db: Extension<DatabaseConnection>) -> Result<Json<ActionResult<Value>>, AppError> {
//!     let models = person::Entity::find()
//!         .filter(person::Column::DeletedAt.is_null())
//!         .limit(20)
//!         .all(&db)
//!         .await
//!         .map_err(|_| AppError::Internal)?;
//!     // 转换为 ActionResult<Value>
//!     Ok(Json(ActionResult::success(...)))
//! }
//! ```

pub mod helpers;
pub mod pagination;
pub mod soft_delete;

pub use helpers::*;
pub use pagination::*;
pub use soft_delete::*;
