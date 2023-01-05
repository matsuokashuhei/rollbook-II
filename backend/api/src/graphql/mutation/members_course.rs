use async_graphql::{Context, Object, Result};
use entity::{async_graphql, members_course};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct MembersCourseMutation;

#[Object]
impl MembersCourseMutation {
    pub async fn create_members_course(
        &self,
        ctx: &Context<'_>,
        input: members_course::CreateMembersCourseInput,
    ) -> Result<members_course::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = members_course::Entity::insert(input.into_active_model())
            .exec(conn)
            .await?;
        let members_course = members_course::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(members_course)
    }
}
