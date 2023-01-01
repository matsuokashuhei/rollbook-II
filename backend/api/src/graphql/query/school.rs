use async_graphql::{Context, Object, Result};
use entity::{async_graphql, school};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct SchoolQuery;

#[Object]
impl SchoolQuery {
    async fn schools(&self, ctx: &Context<'_>) -> Result<Vec<school::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let schools = school::Entity::find().all(conn).await?;
        Ok(schools)
    }
}
