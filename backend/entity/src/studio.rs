use async_graphql::{ComplexObject, Context, Result, SimpleObject};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "studios")]
#[graphql(complex, name = "Studio")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub school_id: i32,
}

#[ComplexObject]
impl Model {
    async fn school(&self, ctx: &Context<'_>) -> Result<super::school::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self
            .find_related(super::school::Entity)
            .one(conn)
            .await?
            .unwrap())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::school::Entity",
        from = "Column::SchoolId",
        to = "super::school::Column::Id"
    )]
    School,
}

impl Related<super::school::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::School.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
