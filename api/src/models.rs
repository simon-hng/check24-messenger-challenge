use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::ConversationState"]
pub enum ConversationState {
    Quoted,
    Rejected,
    Accepted,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::MessageType"]
pub enum MessageType {
    QuoteOffer,
    RejectQuoteMessage,
    StandardMessage,
    AcceptQuoteMessage,
}

#[derive(Debug, diesel_derive_enum::DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::SenderType"]
pub enum SenderType {
    Customer,
    ServiceProvider,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub name: String,
    pub picture: Option<Vec<u8>>,
    pub account_type: SenderType,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = conversation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Conversation {
    pub id: i32,
    pub customer_name: String,
    pub service_provider_name: String,
    pub state: ConversationState,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Account))]
#[diesel(belongs_to(Conversation))]
#[diesel(table_name = account_conversation)]
#[diesel(primary_key(account_id, conversation_id))]
pub struct AccountConversation {
    pub account_id: i32,
    pub conversation_id: i32,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = message)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub message_type: MessageType,
    pub text: Option<String>,
    pub sender_type: SenderType,
    pub read_at: Option<chrono::NaiveDateTime>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
