use async_graphql::{InputObject, Object};
use entity::async_graphql::{self, Context, Result};
use entity::school;
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

#[derive(InputObject)]
pub struct CreateSchoolInput {
    pub name: String,
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
        let active_model = school::ActiveModel {
            id: ActiveValue::Set(0),
            name: ActiveValue::Set(input.name),
        };
        let result = school::Entity::insert(active_model).exec(conn).await;
        match result {
            Ok(result) => {
                let id = result.last_insert_id;
                let school = school::Entity::find_by_id(id)
                    .one(conn)
                    .await
                    .unwrap()
                    .unwrap();
                Ok(school)
            }
            Err(_) => Err(async_graphql::Error::new("Failed to create school")),
        }
    }
}
