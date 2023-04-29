use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait SchoolRepository {
    async fn create(&self) -> Result<()>;
    async fn update(&self) -> Result<()>;
    async fn delete(&self) -> Result<()>;
    async fn get(&self) -> Result<()>;
    async fn list(&self) -> Result<()>;
}
