use async_graphql::{ComplexObject, Context, Result, SimpleObject};
// use sea_orm::{
//     entity::prelude::DeriveEntityModel, ActiveModelBehavior, DerivePrimaryKey, DeriveRelation,
//     EntityTrait, EnumIter, PrimaryKeyTrait,
// };
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "instructors")]
#[graphql(name = "Instructor", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<super::course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(super::course::Entity).all(conn).await?)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::course::Entity",
        from = "Column::Id",
        to = "super::course::Column::InstructorId"
    )]
    Course,
}

impl Related<super::course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
