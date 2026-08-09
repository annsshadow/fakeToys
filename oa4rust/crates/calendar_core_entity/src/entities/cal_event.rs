use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "x_cal_event")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub calendar_id: String,
    pub title: String,
    pub content: Option<String>,
    pub location: Option<String>,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub all_day: bool,
    pub visibility: String,
    pub status: String,
    pub createor: String,
    pub create_time: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
