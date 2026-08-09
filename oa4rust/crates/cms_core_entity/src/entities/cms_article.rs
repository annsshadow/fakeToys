use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "x_cms_article")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub category_id: String,
    pub title: String,
    pub content: Option<String>,
    pub author_id: String,
    pub status: String,
    pub publish_time: Option<DateTime>,
    pub create_time: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
