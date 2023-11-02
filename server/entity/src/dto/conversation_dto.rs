use serde::Serialize;

#[derive(Serialize)]
pub struct ConversationDTO {
    pub conversation: crate::conversation::Model,
    pub partner: Option<crate::account::Model>,
    pub messages: Option<Vec<crate::message::Model>>,
    pub unread_messages_count: Option<u64>,
}
