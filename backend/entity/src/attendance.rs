use crate::{lesson, member};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use sea_orm::{entity::prelude::*, ActiveValue};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "attendances")]
#[graphql(name = "Attendance", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub lesson_id: i32,
    pub member_id: i32,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn member(&self, ctx: &Context<'_>) -> Result<member::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(member::Entity).one(conn).await?.unwrap())
    }
}

#[derive(InputObject)]
pub struct CreateAttendanceInput {
    pub lesson_id: i32,
    pub member_id: i32,
}

impl CreateAttendanceInput {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel::from(self)
    }
}

impl From<CreateAttendanceInput> for ActiveModel {
    fn from(input: CreateAttendanceInput) -> Self {
        ActiveModel {
            lesson_id: ActiveValue::Set(input.lesson_id),
            member_id: ActiveValue::Set(input.member_id),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "lesson::Entity",
        from = "Column::LessonId",
        to = "lesson::Column::Id"
    )]
    Lesson,
    #[sea_orm(
        belongs_to = "member::Entity",
        from = "Column::MemberId",
        to = "member::Column::Id"
    )]
    Member,
}

impl Related<lesson::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lesson.def()
    }
}

impl Related<member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
