use async_graphql::{Context, Object, Result};
use entity::{async_graphql, studio};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct StudioMutation;

#[Object]
impl StudioMutation {
    pub async fn create_studio(
        &self,
        ctx: &Context<'_>,
        input: studio::CreateStudioInput,
    ) -> Result<studio::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = studio::Entity::insert(studio::ActiveModel::from(input))
            .exec(conn)
            .await?;
        let studio = studio::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(studio)
    }
}
