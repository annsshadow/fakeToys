use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "x_hotpic")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub application: String,
    pub info_id: String,
    pub title: String,
    pub base64: Option<String>,
    pub create_time: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
