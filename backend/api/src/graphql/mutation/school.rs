use async_graphql::{Context, Object, Result};
use entity::{async_graphql, school};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct SchoolMutation;

#[Object]
impl SchoolMutation {
    pub async fn create_school(
        &self,
        ctx: &Context<'_>,
        input: school::CreateSchoolInput,
    ) -> Result<school::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = school::Entity::insert(input.into_active_model())
            .exec(conn)
            .await?;
        let school = school::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(school)
    }
}
