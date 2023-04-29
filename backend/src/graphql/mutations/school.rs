use crate::application::models::school::CreateSchool;
use crate::application::usecases::school::Usecase;
use crate::graphql::objects::school::School;
use crate::graphql::{inputs::school::CreateSchoolInput, objects::school::CreateSchoolPayload};
use crate::infrastructure::repositories::school::SchoolRepository;
use anyhow::Result;
use async_graphql::{Context, Object};
use sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct SchoolMutation;

#[Object]
impl SchoolMutation {
    pub async fn create_school(
        &self,
        ctx: &Context<'_>,
        input: CreateSchoolInput,
    ) -> Result<CreateSchoolPayload> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let repository = SchoolRepository { conn: conn };
        let usecase = Usecase::new(Box::new(repository));
        let school = usecase.create(CreateSchool::from(input)).await?;
        Ok(CreateSchoolPayload::new(School::from(school)))
    }
}
