use crate::actor_message::NotifyMessage;
use crate::Mutation;
use entity::message;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, Set};

impl Mutation {
    pub async fn create_message(
        db: &DbConn,
        message: NotifyMessage,
    ) -> Result<message::Model, DbErr> {
        log::info!("Adding message to DB {:?}", message);
        message::ActiveModel {
            message_type: Set(message.message_type.to_owned()),
            conversation_id: Set(message.conversation_id.to_owned()),
            recipient_id: Set(message.recipient_id.to_owned()),
            sender_id: Set(message.sender_id.to_owned()),
            text: Set(message.text.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
