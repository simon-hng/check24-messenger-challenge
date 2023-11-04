use crate::m20230915_210044_create_conversation::Message;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum File {
    Table,
    Id,
    MessageId,
    Name,
    Object,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(File::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(File::Id)
                            .uuid()
                            .extra("DEFAULT uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(File::Name).string())
                    .col(ColumnDef::new(File::Object).binary().not_null())
                    .col(ColumnDef::new(File::MessageId).uuid())
                    .foreign_key(
                        ForeignKey::create()
                            .name("message_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .from(File::Table, File::MessageId)
                            .to(Message::Table, Message::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(File::Table).to_owned())
            .await?;

        Ok(())
    }
}
