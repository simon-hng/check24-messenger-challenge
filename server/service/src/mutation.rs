use entity::message;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};
use crate::server::ClientMessage;

pub struct Mutation;

impl Mutation {
    pub async fn create_message(
        db: &DbConn,
        message: ClientMessage,
    ) -> Result<message::ActiveModel, DbErr> {
        message::ActiveModel {
            message_type: Set(message.message_type.to_owned()),
            conversation_id: Set(message.conversation_id.to_owned()),
            recipient_id: Set(message.recipient_id.to_owned()),
            sender_id: Set(message.sender_id.to_owned()),
            text: Set(message.text.to_owned()),
            ..Default::default()
        }
        .save(db)
        .await
    }
}
