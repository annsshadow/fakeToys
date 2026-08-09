use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "x_org_custom")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub identity_id: String,
    pub field_name: String,
    pub field_value: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
