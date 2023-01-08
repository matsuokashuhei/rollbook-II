use async_graphql::{Context, Object, Result};
use entity::{async_graphql, course};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct CourseMutation;

#[Object]
impl CourseMutation {
    pub async fn create_course(
        &self,
        ctx: &Context<'_>,
        input: course::CreateCourseInput,
    ) -> Result<course::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = course::Entity::insert(input.into_active_model())
            .exec(conn)
            .await?;
        let course = course::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(course)
    }
}
