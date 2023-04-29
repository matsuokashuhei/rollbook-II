use crate::graphql::inputs::school::CreateSchoolInput;

#[derive(Clone)]
pub struct School {
    pub id: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct CreateSchool {
    pub name: String,
}

impl From<CreateSchoolInput> for CreateSchool {
    fn from(input: CreateSchoolInput) -> Self {
        Self { name: input.name }
    }
}

impl From<crate::domain::models::school::School> for School {
    fn from(school: crate::domain::models::school::School) -> Self {
        School {
            id: school.id,
            name: school.name,
        }
    }
}
