use sea_orm_migration::prelude::*;

use crate::m20230915_172016_create_account::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Account::Table)
                    .modify_column(ColumnDef::new(Account::Picture).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Account::Table)
                    .modify_column(ColumnDef::new(Account::Picture).binary())
                    .to_owned(),
            )
            .await
    }
}
