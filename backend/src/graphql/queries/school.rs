use crate::application::usecases::school::Usecase;
use crate::graphql::objects::school::School;
use crate::infrastructure::repositories::school::SchoolRepository;
use anyhow::Result;
use async_graphql::{Context, Object};
use sea_orm::DatabaseConnection;

#[derive(Default)]
pub struct SchoolQuery;

#[Object]
impl SchoolQuery {
    async fn schools(&self, ctx: &Context<'_>) -> Result<Vec<School>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let repository = SchoolRepository { conn: conn };
        let usecase = Usecase::new(Box::new(repository));
        let schools = usecase.find_all().await?;
        Ok(schools.into_iter().map(School::from).collect())
    }
}
