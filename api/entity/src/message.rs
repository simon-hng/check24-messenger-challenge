//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use super::sea_orm_active_enums::MessageType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "message")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub message_type: Option<MessageType>,
    pub text: String,
    pub read_at: Option<DateTime>,
    pub created_at: DateTime,
    pub sender_id: Option<i32>,
    pub conversation_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::account::Entity",
        from = "Column::ConversationId",
        to = "super::account::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Account,
    #[sea_orm(
        belongs_to = "Entity",
        from = "Column::SenderId",
        to = "Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    SelfRef,
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
