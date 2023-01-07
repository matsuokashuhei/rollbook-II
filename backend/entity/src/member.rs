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
    async fn members_courses(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Vec<crate::members_course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(crate::members_course::Entity)
            .all(conn)
            .await?)
    }
    async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<crate::course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(crate::course::Entity).all(conn).await?)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "crate::members_course::Entity",
        from = "Column::Id",
        to = "crate::members_course::Column::MemberId"
    )]
    MembersCourse,
    #[sea_orm(
        has_many = "crate::attendance::Entity",
        from = "Column::Id",
        to = "crate::attendance::Column::MemberId"
    )]
    Attendance,
}

impl Related<crate::members_course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MembersCourse.def()
    }
}

impl Related<crate::course::Entity> for Entity {
    fn to() -> RelationDef {
        crate::members_course::Relation::Course.def()
    }
    fn via() -> Option<RelationDef> {
        Some(crate::members_course::Relation::Member.def().rev())
    }
}

impl Related<crate::attendance::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Attendance.def()
    }
}

#[derive(InputObject)]
pub struct CreateMemberInput {
    pub number: i32,
    pub name: String,
    pub kana: String,
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
