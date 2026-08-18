use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "x_cms_view")]
pub struct Model {
    #[sea_orm(primary_key, column_name = "id")]
    pub xid: String,
    #[sea_orm(column_name = "name")]
    pub xname: String,
    #[sea_orm(column_name = "app_id")]
    pub xapp_id: String,
    pub deleted_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
