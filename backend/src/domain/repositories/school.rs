use crate::domain::models::school::{CreateSchool, School};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SchoolRepository: Sync + Send {
    async fn create(&self, school: CreateSchool) -> Result<School>;
    async fn find_one_by_id(&self, id: i32) -> Result<School>;
    async fn find_all(&self) -> Result<Vec<School>>;
}
