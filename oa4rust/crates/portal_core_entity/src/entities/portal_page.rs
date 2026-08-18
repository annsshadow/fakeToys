use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "x_portal_page")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub portal_id: String,
    pub name: String,
    pub content: Option<String>,
    pub status: String,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
