use crate::{attendance, course, lesson, members_course};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use sea_orm::{entity::prelude::*, ActiveValue};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "members")]
#[graphql(name = "Member", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub number: i32,
    pub name: String,
    pub kana: String,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn members_courses(&self, ctx: &Context<'_>) -> Result<Vec<members_course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(members_course::Entity).all(conn).await?)
    }
    async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(course::Entity).all(conn).await?)
    }
    async fn lessons(&self, ctx: &Context<'_>) -> Result<Vec<lesson::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(lesson::Entity).all(conn).await?)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "members_course::Entity",
        from = "Column::Id",
        to = "members_course::Column::MemberId"
    )]
    MembersCourse,
    #[sea_orm(
        has_many = "attendance::Entity",
        from = "Column::Id",
        to = "attendance::Column::MemberId"
    )]
    Attendance,
}

impl Related<members_course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MembersCourse.def()
    }
}

impl Related<course::Entity> for Entity {
    fn to() -> RelationDef {
        members_course::Relation::Course.def()
    }
    fn via() -> Option<RelationDef> {
        Some(members_course::Relation::Member.def().rev())
    }
}

impl Related<attendance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attendance.def()
    }
}

impl Related<lesson::Entity> for Entity {
    fn to() -> RelationDef {
        attendance::Relation::Lesson.def()
    }
    fn via() -> Option<RelationDef> {
        Some(attendance::Relation::Member.def().rev())
    }
}

#[derive(InputObject)]
pub struct CreateMemberInput {
    pub number: i32,
    pub name: String,
    pub kana: String,
}

impl CreateMemberInput {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel::from(self)
    }
}

impl From<CreateMemberInput> for ActiveModel {
    fn from(input: CreateMemberInput) -> Self {
        ActiveModel {
            number: ActiveValue::Set(input.number),
            name: ActiveValue::Set(input.name),
            kana: ActiveValue::Set(input.kana),
            ..Default::default()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
