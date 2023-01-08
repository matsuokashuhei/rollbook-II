use crate::{attendance, course_schedule, member, time_slot};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use sea_orm::{entity::prelude::*, ActiveValue};

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
    async fn time_slot(&self, ctx: &Context<'_>) -> Result<time_slot::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(time_slot::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<member::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(member::Entity).all(conn).await?)
    }
}

#[derive(InputObject)]
pub struct CreateLessonInput {
    pub course_schedule_id: i32,
    pub date: Date,
}

impl CreateLessonInput {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel::from(self)
    }
}

impl From<CreateLessonInput> for ActiveModel {
    fn from(input: CreateLessonInput) -> Self {
        ActiveModel {
            course_schedule_id: ActiveValue::Set(input.course_schedule_id),
            date: ActiveValue::Set(input.date),
            ..Default::default()
        }
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

impl Related<time_slot::Entity> for Entity {
    fn to() -> RelationDef {
        course_schedule::Relation::TimeSlot.def()
    }
    fn via() -> Option<RelationDef> {
        Some(course_schedule::Relation::Lesson.def().rev())
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
