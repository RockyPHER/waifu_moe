pub use sea_orm_migration::prelude::*;

mod m20241203_015742_first_migrate;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241203_015742_first_migrate::Migration),
        ]
    }
}
