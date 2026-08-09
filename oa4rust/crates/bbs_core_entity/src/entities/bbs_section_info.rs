use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "bbs_section_info")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub forum_id: String,
    pub name: String,
    pub description: Option<String>,
    pub order_number: i32,
    pub disable: bool,
    pub create_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
