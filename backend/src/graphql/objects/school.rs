use async_graphql::SimpleObject;
// use chrono::{DateTime, Utc};

#[derive(SimpleObject)]
pub struct School {
    pub id: i32,
    pub name: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject)]
pub struct CreateSchoolPayload {
    pub school: School,
}

impl CreateSchoolPayload {
    pub fn new(school: School) -> Self {
        Self { school }
    }
}
