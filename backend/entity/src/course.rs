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
    // async fn time_slots(&self, ctx: &Context<'_>) -> Result<Vec<super::time_slot::Model>> {
    //     let conn = ctx.data::<DatabaseConnection>().unwrap();
    //     Ok(self
    //         .find_related(super::time_slot::Entity)
    //         .all(conn)
    //         .await?)
    // }
    async fn time_slot(&self, ctx: &Context<'_>) -> Result<super::time_slot::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::time_slot::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<crate::member::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(super::member::Entity).all(conn).await?)
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
    #[sea_orm(
        has_many = "super::course_schedule::Entity",
        from = "Column::Id",
        to = "super::course_schedule::Column::CourseId"
    )]
    CourseSchedule,
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

impl Related<super::course_schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CourseSchedule.def()
    }
}

impl Related<super::time_slot::Entity> for Entity {
    fn to() -> RelationDef {
        super::course_schedule::Relation::TimeSlot.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::course_schedule::Relation::Course.def().rev())
    }
}

impl Related<crate::member::Entity> for Entity {
    fn to() -> RelationDef {
        crate::members_course::Relation::Member.def()
    }
    fn via() -> Option<RelationDef> {
        Some(crate::members_course::Relation::Course.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
