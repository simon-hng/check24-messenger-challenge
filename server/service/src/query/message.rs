use crate::Query;
use ::entity::{
    message,
    prelude::{Conversation, Message},
};
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_messages_by_conversation_id(
        db: &DbConn,
        conversation_id: Uuid,
    ) -> Result<Vec<message::Model>, DbErr> {
        let conversation = Conversation::find_by_id(conversation_id).one(db).await?;

        let conversation = conversation
            .ok_or("Failed to find associated account")
            .map_err(|err| DbErr::RecordNotFound(err.to_string()))?;

        conversation.find_related(Message).all(db).await
    }
}
