use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "schools")]
// #[graphql(complex, name = "School")]
#[graphql(name="School")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
}

impl ActiveModelBehavior for ActiveModel {}
