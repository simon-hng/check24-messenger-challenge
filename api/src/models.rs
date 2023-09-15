use serde::{Deserialize, Serialize};

pub enum ConversationState {
    Quoted,
    Rejected,
    Accepted,
}

pub enum MessageType {
    QuoteOffer,
    RejectQuoteMessage,
    StandardMessage,
    AcceptQuoteMessage,
}

pub enum SenderType {
    Customer,
    ServiceProvider,
}

pub struct Account {
    pub id: i32,
    pub name: String,
    pub picture: Option<Vec<u8>>,
    pub account_type: SenderType,
}

pub struct Conversation {
    pub id: i32,
    pub customer_name: String,
    pub service_provider_name: String,
    pub state: ConversationState,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct AccountConversation {
    pub account_id: i32,
    pub conversation_id: i32,
}

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
