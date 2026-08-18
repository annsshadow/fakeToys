use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
#[sea_orm(table_name = "x_mind_version_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub mind_id: String,
    pub name: String,
    pub folder_id: String,
    pub description: Option<String>,
    pub creator: String,
    pub creator_unit: Option<String>,
    pub file_version: i32,
    pub shared: bool,
    pub create_time: Option<DateTime>,
    pub update_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
