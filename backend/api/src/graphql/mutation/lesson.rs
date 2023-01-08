use async_graphql::{Context, Object, Result};
use entity::{async_graphql, lesson};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct LessonMutation;

#[Object]
impl LessonMutation {
    pub async fn create_lesson(
        &self,
        ctx: &Context<'_>,
        input: lesson::CreateLessonInput,
    ) -> Result<lesson::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = lesson::Entity::insert(input.into_active_model())
            .exec(conn)
            .await?;
        let lesson = lesson::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(lesson)
    }
}
