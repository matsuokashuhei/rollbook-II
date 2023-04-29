use crate::{
    application::models::school::{CreateSchool, School},
    domain::repositories::school::SchoolRepository,
};
use anyhow::Result;

pub struct Usecase<'a> {
    pub repository: Box<dyn SchoolRepository + 'a>,
}

impl<'a> Usecase<'a> {
    pub fn new(repository: Box<dyn SchoolRepository + 'a>) -> Self {
        Usecase {
            repository: repository,
        }
    }
    pub async fn create(&self, school: CreateSchool) -> Result<School> {
        let school = self
            .repository
            .create(crate::domain::models::school::CreateSchool::from(school))
            .await?;
        Ok(School::from(school))
    }
    pub async fn find_all(&self) -> Result<Vec<School>> {
        let schools = self.repository.find_all().await?;
        Ok(schools.into_iter().map(School::from).collect())
    }
}
