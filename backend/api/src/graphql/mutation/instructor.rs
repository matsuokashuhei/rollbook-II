use async_graphql::{Context, InputObject, Object, Result};
use entity::{async_graphql, instructor};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

#[derive(InputObject)]
pub struct CreateInstructorInput {
    pub name: String,
    pub email: String,
    pub phone_number: String,
}

impl CreateInstructorInput {
    fn into_active_model(self) -> instructor::ActiveModel {
        instructor::ActiveModel {
            name: ActiveValue::Set(self.name),
            email: ActiveValue::Set(self.email),
            phone_number: ActiveValue::Set(self.phone_number),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct InstructorMutation;

#[Object]
impl InstructorMutation {
    pub async fn create_instructor(
        &self,
        ctx: &Context<'_>,
        input: CreateInstructorInput,
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
