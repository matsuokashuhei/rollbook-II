use async_graphql::{Context, InputObject, Object, Result};
use entity::{async_graphql, studio};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};

#[derive(InputObject)]
pub struct CreateStudioInput {
    pub school_id: i32,
    pub name: String,
}

impl CreateStudioInput {
    fn into_active_model(self) -> studio::ActiveModel {
        studio::ActiveModel {
            name: ActiveValue::Set(self.name),
            school_id: ActiveValue::Set(self.school_id),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct StudioMutation;

#[Object]
impl StudioMutation {
    pub async fn create_studio(
        &self,
        ctx: &Context<'_>,
        input: CreateStudioInput,
    ) -> Result<studio::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = studio::Entity::insert(input.into_active_model())
            .exec(conn)
            .await?;
        let studio = studio::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(studio)
    }
}
