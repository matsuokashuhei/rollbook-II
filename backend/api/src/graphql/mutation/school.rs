use async_graphql::{Context, InputObject, Object, Result};
use entity::{async_graphql, school};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

#[derive(InputObject)]
pub struct CreateSchoolInput {
    pub name: String,
}

impl CreateSchoolInput {
    fn into_active_model(self) -> school::ActiveModel {
        school::ActiveModel {
            name: ActiveValue::Set(self.name),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct SchoolMutation;

#[Object]
impl SchoolMutation {
    pub async fn create_school(
        &self,
        ctx: &Context<'_>,
        input: CreateSchoolInput,
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
