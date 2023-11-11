use crate::m20230915_172016_create_account::Account;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Enquiry {
    Table,
    Id,
    EnquirerId,
    Title,
    Description,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Enquiry::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Enquiry::Id)
                            .uuid()
                            .extra("DEFAULT uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Enquiry::EnquirerId).uuid().not_null())
                    .col(ColumnDef::new(Enquiry::Title).string().not_null())
                    .col(ColumnDef::new(Enquiry::Description).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("enquirer_id")
                            .from(Enquiry::Table, Enquiry::EnquirerId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Enquiry::Table).to_owned())
            .await
    }
}
