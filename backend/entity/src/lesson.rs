use crate::{attendance, course_schedule, member};
use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "lessons")]
#[graphql(name = "Lesson", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub course_schedule_id: i32,
    pub date: Date,
}

#[ComplexObject]
impl Model {
    async fn course_schedule(&self, ctx: &Context<'_>) -> Result<course_schedule::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(course_schedule::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<member::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(member::Entity).all(conn).await?)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "course_schedule::Entity",
        from = "Column::CourseScheduleId",
        to = "course_schedule::Column::Id"
    )]
    CouseSchedule,
    #[sea_orm(
        has_many = "attendance::Entity",
        from = "Column::Id",
        to = "attendance::Column::LessonId"
    )]
    Attendance,
}

impl Related<course_schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CouseSchedule.def()
    }
}

impl Related<attendance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attendance.def()
    }
}

impl Related<member::Entity> for Entity {
    fn to() -> RelationDef {
        attendance::Relation::Member.def()
    }
    fn via() -> Option<RelationDef> {
        Some(attendance::Relation::Lesson.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
