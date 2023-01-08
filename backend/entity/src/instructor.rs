use crate::course;
use async_graphql::{ComplexObject, Context, InputObject, Result, SimpleObject};
// use sea_orm::{
//     entity::prelude::DeriveEntityModel, ActiveModelBehavior, DerivePrimaryKey, DeriveRelation,
//     EntityTrait, EnumIter, PrimaryKeyTrait,
// };
use sea_orm::{entity::prelude::*, ActiveValue};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[sea_orm(table_name = "instructors")]
#[graphql(name = "Instructor", complex)]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub created_at: ChronoDateTime,
    pub updated_at: ChronoDateTime,
}

#[ComplexObject]
impl Model {
    async fn courses(&self, ctx: &Context<'_>) -> Result<Vec<course::Model>> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        Ok(self.find_related(course::Entity).all(conn).await?)
    }
}

#[derive(InputObject)]
pub struct CreateInstructorInput {
    pub name: String,
    pub email: String,
    pub phone_number: String,
}

impl CreateInstructorInput {
    pub fn into_active_model(self) -> ActiveModel {
        ActiveModel::from(self)
    }
}

impl From<CreateInstructorInput> for ActiveModel {
    fn from(input: CreateInstructorInput) -> Self {
        ActiveModel {
            name: ActiveValue::Set(input.name),
            email: ActiveValue::Set(input.email),
            phone_number: ActiveValue::Set(input.phone_number),
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "course::Entity",
        from = "Column::Id",
        to = "course::Column::InstructorId"
    )]
    Course,
}

impl Related<course::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Course.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
