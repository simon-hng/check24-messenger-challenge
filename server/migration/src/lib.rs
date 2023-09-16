pub use sea_orm_migration::prelude::*;

mod m20230915_172016_create_account;
mod m20230915_210044_create_conversation;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230915_172016_create_account::Migration),
            Box::new(m20230915_210044_create_conversation::Migration),
        ]
    }
}
