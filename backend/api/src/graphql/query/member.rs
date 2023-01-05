use async_graphql::{Context, Object, Result};
use entity::{async_graphql, member};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct MemberQuery;

#[Object]
impl MemberQuery {
    async fn members(&self, ctx: &Context<'_>) -> Result<Vec<member::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let members = member::Entity::find().all(conn).await?;
        Ok(members)
    }
}
