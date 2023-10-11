//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use super::sea_orm_active_enums::ConversationState;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateConversation {
    pub state: Option<ConversationState>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub state: Option<ConversationState>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::conversation_account::Entity")]
    ConversationAccount,
    #[sea_orm(has_many = "super::message::Entity")]
    Message,
}

impl Related<super::conversation_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ConversationAccount.def()
    }
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        super::conversation_account::Relation::Account.def()
    }
    fn via() -> Option<RelationDef> {
        Some(
            super::conversation_account::Relation::Conversation
                .def()
                .rev(),
        )
    }
}

impl ActiveModelBehavior for ActiveModel {}
