use crate::domain::{
    models::school::{CreateSchool, School},
    repositories::school::SchoolRepository as ISchoolRepository,
};
use crate::infrastructure::models::school;
use anyhow::{format_err, Result};
use async_trait::async_trait;
use sea_orm::{ActiveValue, DatabaseConnection, DbErr, EntityName, EntityTrait};

pub struct SchoolRepository<'a> {
    pub conn: &'a DatabaseConnection,
}

#[async_trait]
impl<'a> ISchoolRepository for SchoolRepository<'a> {
    async fn create(&self, school: CreateSchool) -> Result<School> {
        let active_model = school::ActiveModel {
            name: ActiveValue::set(school.name),
            ..Default::default()
        };
        let id = school::Entity::insert(active_model)
            .exec(self.conn)
            .await
            .map(|result| result.last_insert_id)
            .map_err(|e| format_err!(e))?;
        Ok(self.find_one_by_id(id).await?)
    }

    async fn find_one_by_id(&self, id: i32) -> Result<School> {
        let model = match school::Entity::find_by_id(id)
            .one(self.conn)
            .await
            .map_err(|e| format_err!(e))?
        {
            Some(model) => model,
            None => {
                let message = format!(
                    "Failed to find record in {} with id={}",
                    school::Entity.table_name(),
                    id
                );
                return Err(format_err!(DbErr::RecordNotFound(message)));
            }
        };
        Ok(School::from(model))
    }

    async fn find_all(&self) -> Result<Vec<School>> {
        let models = school::Entity::find()
            .all(self.conn)
            .await
            .map_err(|e| format_err!(e))?;
        Ok(models.into_iter().map(School::from).collect())
    }
}
