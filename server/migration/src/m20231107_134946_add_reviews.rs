use sea_orm_migration::prelude::*;

use crate::m20230915_210044_create_conversation::Conversation;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(DeriveIden)]
pub enum Review {
    Table,
    Id,
    ConversationId,
    Score,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Review::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Review::Id)
                            .uuid()
                            .extra("DEFAULT uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Review::ConversationId).uuid().not_null())
                    .col(ColumnDef::new(Review::Score).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("reviewer_id")
                            .from(Review::Table, Review::ConversationId)
                            .to(Conversation::Table, Conversation::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Review::Table).to_owned())
            .await
    }
}
