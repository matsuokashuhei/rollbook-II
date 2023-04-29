pub struct School {
    pub id: i32,
    pub name: String,
}

impl From<crate::infrastructure::models::school::Model> for School {
    fn from(model: crate::infrastructure::models::school::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}

pub struct CreateSchool {
    pub name: String,
}

impl From<crate::application::models::school::CreateSchool> for CreateSchool {
    fn from(input: crate::application::models::school::CreateSchool) -> Self {
        Self { name: input.name }
    }
}
