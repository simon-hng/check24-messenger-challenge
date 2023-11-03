use crate::sea_orm_active_enums::ConversationState;
use sea_orm::prelude::Uuid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewConversation {
    pub state: Option<ConversationState>,
    pub partner_id: Uuid,
}
