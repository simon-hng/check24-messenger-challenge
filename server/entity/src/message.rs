//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use super::sea_orm_active_enums::MessageType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub message_type: MessageType,
    pub text: String,
    pub read_at: Option<DateTime>,
    pub created_at: DateTime,
    pub recipient_id: i32,
    pub sender_id: i32,
    pub conversation_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::RecipientId",
        to = "super::account::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Account2,
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::SenderId",
        to = "super::account::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Account1,
    #[sea_orm(
        belongs_to = "super::conversation::Entity",
        from = "Column::ConversationId",
        to = "super::conversation::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Conversation,
}

impl Related<super::conversation::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Conversation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
