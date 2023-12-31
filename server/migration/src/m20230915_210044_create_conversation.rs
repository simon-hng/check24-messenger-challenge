use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

use crate::m20230915_172016_create_account::Account;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden, EnumIter)]
pub enum ConversationState {
    Table,
    #[iden = "quoted"]
    Quoted,
    #[iden = "rejected"]
    Rejected,
    #[iden = "accepted"]
    Accepted,
}

#[derive(DeriveIden)]
pub enum Conversation {
    Table,
    Id,
    State,
    CreatedAt,
}

#[derive(DeriveIden)]
pub enum ConversationAccount {
    Table,
    ConversationId,
    AccountId,
}

#[derive(Iden, EnumIter)]
pub enum MessageType {
    Table,
    #[iden = "standard"]
    Standard,
    #[iden = "quote_offer"]
    Quote,
    #[iden = "accept_quote"]
    Accept,
    #[iden = "reject_quote"]
    Reject,
}

#[derive(DeriveIden)]
pub enum Message {
    Table,
    Id,
    Text,
    MessageType,
    ReadAt,
    CreatedAt,
    RecipientId,
    SenderId,
    ConversationId,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ConversationState::Table)
                    .values(ConversationState::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Conversation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Conversation::Id)
                            .uuid()
                            .extra("DEFAULT uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Conversation::State)
                            .enumeration(
                                ConversationState::Table,
                                ConversationState::iter().skip(1),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Conversation::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ConversationAccount::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ConversationAccount::ConversationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ConversationAccount::AccountId)
                            .uuid()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(ConversationAccount::AccountId)
                            .col(ConversationAccount::ConversationId),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("conversation_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .from(
                                ConversationAccount::Table,
                                ConversationAccount::ConversationId,
                            )
                            .to(Conversation::Table, Conversation::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("account_id")
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade)
                            .from(ConversationAccount::Table, ConversationAccount::AccountId)
                            .to(Account::Table, Account::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(MessageType::Table)
                    .values(MessageType::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .uuid()
                            .extra("DEFAULT uuid_generate_v4()")
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Message::MessageType)
                            .not_null()
                            .enumeration(MessageType::Table, MessageType::iter().skip(1)),
                    )
                    .col(ColumnDef::new(Message::Text).string().not_null())
                    .col(ColumnDef::new(Message::ReadAt).date_time())
                    .col(
                        ColumnDef::new(Message::CreatedAt)
                            .date_time()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Message::RecipientId).uuid().not_null())
                    .col(ColumnDef::new(Message::SenderId).uuid().not_null())
                    .col(ColumnDef::new(Message::ConversationId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("recipient_id")
                            .from(Message::Table, Message::RecipientId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("sender_id")
                            .from(Message::Table, Message::SenderId)
                            .to(Account::Table, Account::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("conversation_id")
                            .from(Message::Table, Message::ConversationId)
                            .to(Conversation::Table, Conversation::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ConversationAccount::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Conversation::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().if_exists().name(MessageType::Table).to_owned())
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .if_exists()
                    .name(ConversationState::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
