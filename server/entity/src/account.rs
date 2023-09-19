//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use super::sea_orm_active_enums::AccountType;
use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize)]
#[sea_orm(table_name = "account")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub account_name: String,
    pub picture: Option<String>,
    pub account_type: Option<AccountType>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::message::Entity")]
    Message,
}

impl Related<super::message::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Message.def()
    }
}
impl Related<super::conversation::Entity> for Entity {
    fn to() -> RelationDef {
        super::conversation_account::Relation::Conversation.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::conversation_account::Relation::Account.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
