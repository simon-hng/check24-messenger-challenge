//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.2

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "account_type")]
pub enum AccountType {
    #[sea_orm(string_value = "customer")]
    Customer,
    #[sea_orm(string_value = "service_provider")]
    ServiceProvider,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "conversation_state")]
pub enum ConversationState {
    #[sea_orm(string_value = "accepted")]
    Accepted,
    #[sea_orm(string_value = "quoted")]
    Quoted,
    #[sea_orm(string_value = "rejected")]
    Rejected,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "message_type")]
pub enum MessageType {
    #[sea_orm(string_value = "accept_quote")]
    AcceptQuote,
    #[sea_orm(string_value = "quote_offer")]
    QuoteOffer,
    #[sea_orm(string_value = "standard")]
    Standard,
}
