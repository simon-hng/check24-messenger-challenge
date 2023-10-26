use crate::Query;
use ::entity::{conversation, message, prelude::Message};
use sea_orm::*;

impl Query {
    pub async fn find_messages_by_conversation(
        db: &DbConn,
        conversation: conversation::Model,
    ) -> Result<Vec<message::Model>, DbErr> {
        conversation.find_related(Message).all(db).await
    }
}
