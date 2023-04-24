use crate::application::usecases::school::SchoolUsecase;
use crate::graphql::objects::school::School;
use crate::graphql::{inputs::school::CreateSchoolInput, objects::school::CreateSchoolPayload};
use async_graphql::{Context, Object, Result};

#[derive(Default)]
pub struct SchoolMutation;

#[Object]
impl SchoolMutation {
    pub async fn create_school(
        &self,
        ctx: &Context<'_>,
        input: CreateSchoolInput,
    ) -> Result<CreateSchoolPayload> {
        // // let conn = ctx.data::<DatabaseConnection>().unwrap();
        // let result = school::Entity::insert(input.into_active_model())
        //     .exec(conn)
        //     .await?;
        // let school = school::Entity::find_by_id(result.last_insert_id)
        //     .one(conn)
        //     .await?
        //     .unwrap();
        // Ok(school)
        let school = School {
            id: 1,
            name: "Tachikawa".to_owned(),
        };
        let usecase = SchoolUsecase { repository: None };
        // let usecase = crate::usecase::school::CreateSchoolUsecase::new();
        Ok(CreateSchoolPayload::new(school))
    }
}
