use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConversationDTO {
    pub conversation: crate::conversation::Model,
    pub partner: Option<crate::account::Model>,
    pub messages: Option<Vec<crate::message::Model>>,
    pub review: Option<Option<crate::review::Model>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateConversationDTO {
    pub partner_id: Uuid,
}
