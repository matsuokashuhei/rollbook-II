use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "courses")]
#[graphql(name = "Course", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub studio_id: i32,
    pub name: String,
    pub instructor_id: i32,
    pub price: i32,
    pub start_date: ChronoDate,
    pub end_date: ChronoDate,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn instructor(&self, ctx: &Context<'_>) -> Result<super::instructor::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::instructor::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn studio(&self, ctx: &Context<'_>) -> Result<super::studio::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::studio::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::instructor::Entity",
        from = "Column::InstructorId",
        to = "super::instructor::Column::Id"
    )]
    Instructor,
    #[sea_orm(
        belongs_to = "super::studio::Entity",
        from = "Column::StudioId",
        to = "super::studio::Column::Id"
    )]
    Studio,
}

impl Related<super::instructor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Instructor.def()
    }
}

impl Related<super::studio::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studio.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
