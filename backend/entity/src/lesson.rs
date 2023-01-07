use async_graphql::{ComplexObject, Context, Result, SimpleObject};
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
    async fn course_schedule(&self, ctx: &Context<'_>) -> Result<crate::course_schedule::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(crate::course_schedule::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::course_schedule::Entity",
        from = "Column::CourseScheduleId",
        to = "crate::course_schedule::Column::Id"
    )]
    CouseSchedule,
}

impl Related<crate::course_schedule::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CouseSchedule.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
