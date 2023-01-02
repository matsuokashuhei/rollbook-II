use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "studios")]
#[graphql(name = "Studio", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub school_id: i32,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn school(&self, ctx: &Context<'_>) -> Result<super::school::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::school::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn time_slots(&self, ctx: &Context<'_>) -> Result<Vec<super::time_slot::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::time_slot::Entity)
            .all(conn)
            .await?)
    }
    async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<super::course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(super::course::Entity).all(conn).await?)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::school::Entity",
        from = "Column::SchoolId",
        to = "super::school::Column::Id"
    )]
    School,
    #[sea_orm(
        has_many = "super::time_slot::Entity",
        from = "Column::Id",
        to = "super::time_slot::Column::StudioId"
    )]
    TimeSlot,
    #[sea_orm(
        has_many = "super::course::Entity",
        from = "Column::Id",
        to = "super::course::Column::StudioId"
    )]
    Course,
}

impl Related<super::school::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::School.def()
    }
}

impl Related<super::time_slot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimeSlot.def()
    }
}

impl Related<super::course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
