use async_graphql::{Context, Object, Result};
use entity::{async_graphql, instructor};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct InstructorMutation;

#[Object]
impl InstructorMutation {
    pub async fn create_instructor(
        &self,
        ctx: &Context<'_>,
        input: instructor::CreateInstructorInput,
    ) -> Result<instructor::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = instructor::Entity::insert(input.into_active_model())
            .exec(conn)
            .await?;
        let instructor = instructor::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(instructor)
    }
}
