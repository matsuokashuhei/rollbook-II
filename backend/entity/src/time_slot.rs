use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum DayOfWeek {
    Monday = 1,
    Tuesday = 2,
    Wednesday = 3,
    Thursday = 4,
    Friday = 5,
    Saturday = 6,
    Sunday = 7,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "time_slots")]
#[graphql(name = "TimeSlot", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub studio_id: i32,
    // pub day_of_week: DayOfWeek,
    pub day_of_week: i32,
    pub start_time: String,
    pub end_time: String,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn studio(&self, ctx: &Context<'_>) -> Result<super::studio::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::studio::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn course_schedules(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<super::course_schedule::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::course_schedule::Entity)
            .all(conn)
            .await?)
    }
    // async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<super::course::Model>> {
    //     let conn = ctx.data::<DatabaseConnection>().unwrap();
    //     Ok(self.find_related(super::course::Entity).all(conn).await?)
    // }
    async fn course(&self, ctx: &Context<'_>) -> Result<Option<super::course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(super::course::Entity).one(conn).await?)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::studio::Entity",
        from = "Column::StudioId",
        to = "super::studio::Column::Id"
    )]
    Studio,
    #[sea_orm(
        has_many = "super::course_schedule::Entity",
        from = "Column::Id",
        to = "super::course_schedule::Column::TimeSlotId"
    )]
    CourseSchedule,
}

impl Related<super::studio::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studio.def()
    }
}

impl Related<super::course_schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseSchedule.def()
    }
}

impl Related<super::course::Entity> for Entity {
    fn to() -> RelationDef {
        super::course_schedule::Relation::Course.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::course_schedule::Relation::TimeSlot.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
