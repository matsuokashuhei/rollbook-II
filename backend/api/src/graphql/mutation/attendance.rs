use async_graphql::{Context, Object, Result};
use entity::{async_graphql, attendance};
use sea_orm::{DatabaseConnection, EntityTrait};

#[derive(Default)]
pub struct AttendanceMutation;

#[Object]
impl AttendanceMutation {
    pub async fn create_attandance(
        &self,
        ctx: &Context<'_>,
        input: attendance::CreateAttendanceInput,
    ) -> Result<attendance::Model> {
        let conn = ctx.data::<DatabaseConnection>().unwrap();
        let result = attendance::Entity::insert(input.into_active_model())
            .exec(conn)
            .await?;
        let attendance = attendance::Entity::find_by_id(result.last_insert_id)
            .one(conn)
            .await?
            .unwrap();
        Ok(attendance)
    }
}
