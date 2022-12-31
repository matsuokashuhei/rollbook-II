// use migration::{Migrator, MigrationTrait};

fn main() {
    // let connection = sea_orm::Database::connect(&database_url).await?
    // Migrator::up(&connection, None).await?
    api::main();
}
