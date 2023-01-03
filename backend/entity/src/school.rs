use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use sea_orm::{entity::prelude::*, ActiveValue};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "schools")]
#[graphql(name = "School", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn studios(&self, ctx: &Context<'_>) -> Result<Vec<super::studio::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(super::studio::Entity).all(conn).await?)
    }
}

#[derive(InputObject)]
pub struct CreateSchoolInput {
    pub name: String,
}

impl From<CreateSchoolInput> for ActiveModel {
    fn from(input: CreateSchoolInput) -> Self {
        ActiveModel {
            name: ActiveValue::Set(input.name),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::studio::Entity",
        from = "Column::Id",
        to = "super::studio::Column::SchoolId"
    )]
    Studio,
}

impl Related<super::studio::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studio.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
