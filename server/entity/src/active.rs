use crate::account;
use crate::sea_orm_active_enums::ConversationState;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewConversation {
    pub state: Option<ConversationState>,
    pub recipient: account::Model,
}