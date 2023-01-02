use async_graphql::SimpleObject;
// use sea_orm::{
//     entity::prelude::DeriveEntityModel, ActiveModelBehavior, DerivePrimaryKey, DeriveRelation,
//     EntityTrait, EnumIter, PrimaryKeyTrait,
// };
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "instructors")]
#[graphql(name = "Instructor")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

// #[ComplexObject]
// impl Model {}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
