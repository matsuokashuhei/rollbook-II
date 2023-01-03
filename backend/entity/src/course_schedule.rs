use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "course_schedules")]
#[graphql(name = "CourseSchedule", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub course_id: i32,
    pub time_slot_id: i32,
    pub start_date: ChronoDate,
    pub end_date: ChronoDate,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn course(&self, ctx: &Context<'_>) -> Result<super::course::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::course::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn time_slot(&self, ctx: &Context<'_>) -> Result<super::time_slot::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::time_slot::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::course::Entity",
        from = "Column::CourseId",
        to = "super::course::Column::Id"
    )]
    Course,
    #[sea_orm(
        belongs_to = "super::time_slot::Entity",
        from = "Column::TimeSlotId",
        to = "super::time_slot::Column::Id"
    )]
    TimeSlot,
}

impl Related<super::course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl Related<super::time_slot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimeSlot.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
