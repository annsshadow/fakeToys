use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "x_org_identity")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub person_id: String,
    pub name: String,
    pub type_: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
