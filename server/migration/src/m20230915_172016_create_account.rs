use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
};
use sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden, EnumIter)]
pub enum AccountType {
    Table,
    #[iden = "customer"]
    Customer,
    #[iden = "service_provider"]
    ServiceProvider,
}

#[derive(DeriveIden)]
pub enum Account {
    Table,
    Id,
    Name,
    Picture,
    AccountType,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(AccountType::Table)
                    .values(AccountType::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Account::Name).string().not_null())
                    .col(ColumnDef::new(Account::Picture).binary())
                    .col(
                        ColumnDef::new(Account::AccountType)
                            .enumeration(AccountType::Table, AccountType::iter().skip(1)),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_type(Type::drop().if_exists().name(AccountType::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await?;

        Ok(())
    }
}
