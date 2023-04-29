use async_graphql::SimpleObject;
// use chrono::{DateTime, Utc};

#[derive(SimpleObject, Clone)]
pub struct School {
    pub id: i32,
    pub name: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

impl From<crate::application::models::school::School> for School {
    fn from(school: crate::application::models::school::School) -> Self {
        School {
            id: school.id,
            name: school.name,
        }
    }
}

#[derive(SimpleObject)]
pub struct CreateSchoolPayload {
    pub school: School,
}

impl CreateSchoolPayload {
    pub fn new(school: School) -> Self {
        Self { school: school }
    }
}
