use crate::{course_schedule, instructor, lesson, member, members_course, studio, time_slot};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use sea_orm::{entity::prelude::*, ActiveValue};

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
    async fn instructor(&self, ctx: &Context<'_>) -> Result<instructor::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(instructor::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn studio(&self, ctx: &Context<'_>) -> Result<studio::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(studio::Entity).one(conn).await?.unwrap())
    }
    async fn course_schedules(&self, ctx: &Context<'_>) -> Result<Vec<course_schedule::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(course_schedule::Entity).all(conn).await?)
    }
    // async fn time_slots(&self, ctx: &Context<'_>) -> Result<Vec<time_slot::Model>> {
    //     let conn = ctx.data::<DatabaseConnection>().unwrap();
    //     Ok(self
    //         .find_related(time_slot::Entity)
    //         .all(conn)
    //         .await?)
    // }
    async fn time_slot(&self, ctx: &Context<'_>) -> Result<time_slot::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(time_slot::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn lessons(&self, ctx: &Context<'_>) -> Result<Vec<lesson::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(lesson::Entity).all(conn).await?)
    }
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<member::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(member::Entity).all(conn).await?)
    }
}

#[derive(InputObject)]
pub struct CreateCourseInput {
    pub studio_id: i32,
    pub name: String,
    pub price: i32,
    pub instructor_id: i32,
    pub start_date: Date,
    pub end_date: Date,
}

impl CreateCourseInput {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel::from(self)
    }
}

impl From<CreateCourseInput> for ActiveModel {
    fn from(input: CreateCourseInput) -> Self {
        ActiveModel {
            studio_id: ActiveValue::Set(input.studio_id),
            name: ActiveValue::Set(input.name),
            price: ActiveValue::Set(input.price),
            instructor_id: ActiveValue::Set(input.instructor_id),
            start_date: ActiveValue::Set(input.start_date),
            end_date: ActiveValue::Set(input.end_date),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "instructor::Entity",
        from = "Column::InstructorId",
        to = "instructor::Column::Id"
    )]
    Instructor,
    #[sea_orm(
        belongs_to = "studio::Entity",
        from = "Column::StudioId",
        to = "studio::Column::Id"
    )]
    Studio,
    #[sea_orm(
        has_many = "course_schedule::Entity",
        from = "Column::Id",
        to = "course_schedule::Column::CourseId"
    )]
    CourseSchedule,
}

impl Related<instructor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Instructor.def()
    }
}

impl Related<studio::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studio.def()
    }
}

impl Related<course_schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseSchedule.def()
    }
}

impl Related<time_slot::Entity> for Entity {
    fn to() -> RelationDef {
        course_schedule::Relation::TimeSlot.def()
    }
    fn via() -> Option<RelationDef> {
        Some(course_schedule::Relation::Course.def().rev())
    }
}

impl Related<member::Entity> for Entity {
    fn to() -> RelationDef {
        members_course::Relation::Member.def()
    }
    fn via() -> Option<RelationDef> {
        Some(members_course::Relation::Course.def().rev())
    }
}

impl Related<lesson::Entity> for Entity {
    fn to() -> RelationDef {
        course_schedule::Relation::Lesson.def()
    }
    fn via() -> Option<RelationDef> {
        Some(course_schedule::Relation::Course.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
