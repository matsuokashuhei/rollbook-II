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
    async fn member(&self, ctx: &Context<'_>) -> Result<crate::member::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(crate::member::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::lesson::Entity",
        from = "Column::LessonId",
        to = "crate::lesson::Column::Id"
    )]
    Lesson,
    #[sea_orm(
        belongs_to = "crate::member::Entity",
        from = "Column::MemberId",
        to = "crate::member::Column::Id"
    )]
    Member,
}

impl Related<crate::lesson::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lesson.def()
    }
}

impl Related<crate::member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Member.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
