use crate::{course, school, time_slot};
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
use sea_orm::{entity::prelude::*, ActiveValue};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "studios")]
#[graphql(name = "Studio", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub school_id: i32,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn school(&self, ctx: &Context<'_>) -> Result<school::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(school::Entity).one(conn).await?.unwrap())
    }
    async fn time_slots(&self, ctx: &Context<'_>) -> Result<Vec<time_slot::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(time_slot::Entity).all(conn).await?)
    }
    async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(course::Entity).all(conn).await?)
    }
}

#[derive(InputObject)]
pub struct CreateStudioInput {
    pub school_id: i32,
    pub name: String,
}

impl CreateStudioInput {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel::from(self)
    }
}

impl From<CreateStudioInput> for ActiveModel {
    fn from(input: CreateStudioInput) -> Self {
        ActiveModel {
            name: ActiveValue::Set(input.name),
            school_id: ActiveValue::Set(input.school_id),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "school::Entity",
        from = "Column::SchoolId",
        to = "school::Column::Id"
    )]
    School,
    #[sea_orm(
        has_many = "time_slot::Entity",
        from = "Column::Id",
        to = "time_slot::Column::StudioId"
    )]
    TimeSlot,
    #[sea_orm(
        has_many = "course::Entity",
        from = "Column::Id",
        to = "course::Column::StudioId"
    )]
    Course,
}

impl Related<school::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::School.def()
    }
}

impl Related<time_slot::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::TimeSlot.def()
    }
}

impl Related<course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
