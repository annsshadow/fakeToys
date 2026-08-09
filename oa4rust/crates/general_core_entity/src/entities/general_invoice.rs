use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "x_general_invoice")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub number: String,
    pub date: String,
    pub amount: f64,
    pub status: String,
    pub creator: Option<String>,
    pub create_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
