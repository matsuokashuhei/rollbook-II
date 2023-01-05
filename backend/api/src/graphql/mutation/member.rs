use async_graphql::{Context, Object, Result};
use entity::{async_graphql, member};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct MemberMutation;

#[Object]
impl MemberMutation {
    pub async fn create_member(
        &self,
        ctx: &Context<'_>,
        input: member::CreateMemberInput,
    ) -> Result<member::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = member::Entity::insert(member::ActiveModel::from(input))
            .exec(conn)
            .await?;
        let member = member::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(member)
    }
}
