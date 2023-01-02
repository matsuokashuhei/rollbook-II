use async_graphql::{Context, InputObject, Object, Result};
use entity::{async_graphql, course};
use sea_orm::{prelude::Date, ActiveValue, DatabaseConnection, EntityTrait};

#[derive(InputObject)]
pub struct CreateCourseInput {
    pub studio_id: i32,
    pub name: String,
    pub price: i32,
    pub instructor_id: i32,
    pub start_date: Date,
    pub end_date: Date,
}

impl CreateCourseInput {
    fn into_active_model(self) -> course::ActiveModel {
        course::ActiveModel {
            studio_id: ActiveValue::Set(self.studio_id),
            name: ActiveValue::Set(self.name),
            price: ActiveValue::Set(self.price),
            instructor_id: ActiveValue::Set(self.instructor_id),
            start_date: ActiveValue::Set(self.start_date),
            end_date: ActiveValue::Set(self.end_date),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct CourseMutation;

#[Object]
impl CourseMutation {
    pub async fn create_course(
        &self,
        ctx: &Context<'_>,
        input: CreateCourseInput,
    ) -> Result<course::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = course::Entity::insert(input.into_active_model())
            .exec(conn)
            .await?;
        let course = course::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(course)
    }
}
