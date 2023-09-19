//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use super::sea_orm_active_enums::ConversationState;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "conversation")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub state: Option<ConversationState>,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

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
