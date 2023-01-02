use async_graphql::{Context, Object, Result};
use entity::{async_graphql, instructor};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct InstructorQuery;

#[Object]
impl InstructorQuery {
    async fn instructors(&self, ctx: &Context<'_>) -> Result<Vec<instructor::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(instructor::Entity::find().all(conn).await?)
    }
}
