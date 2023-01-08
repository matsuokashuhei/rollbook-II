use crate::{course, member};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use sea_orm::{entity::prelude::*, ActiveValue};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "members_courses")]
#[graphql(name = "MembersCourse", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub member_id: i32,
    pub course_id: i32,
    pub start_date: Date,
    pub end_date: Date,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn member(&self, ctx: &Context<'_>) -> Result<member::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(member::Entity).one(conn).await?.unwrap())
    }
    async fn course(&self, ctx: &Context<'_>) -> Result<course::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(course::Entity).one(conn).await?.unwrap())
    }
}
#[derive(InputObject)]
pub struct CreateMembersCourseInput {
    pub member_id: i32,
    pub course_id: i32,
    pub start_date: Date,
    pub end_date: Date,
}

impl CreateMembersCourseInput {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel::from(self)
    }
}

impl From<CreateMembersCourseInput> for ActiveModel {
    fn from(input: CreateMembersCourseInput) -> Self {
        ActiveModel {
            member_id: ActiveValue::Set(input.member_id),
            course_id: ActiveValue::Set(input.course_id),
            start_date: ActiveValue::Set(input.start_date),
            end_date: ActiveValue::Set(input.end_date),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "member::Entity",
        from = "Column::MemberId",
        to = "member::Column::Id"
    )]
    Member,
    #[sea_orm(
        belongs_to = "course::Entity",
        from = "Column::CourseId",
        to = "course::Column::Id"
    )]
    Course,
}

impl Related<member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl Related<course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
