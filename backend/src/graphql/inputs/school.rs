use async_graphql::InputObject;

#[derive(InputObject)]
// #[graphql(name = "CreateSchoolInput")]
pub struct CreateSchoolInput {
    pub name: String,
}
