use actix_web::web::Json;
use entity::{conversation, message};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbConn, DbErr, Set};
use entity::conversation::CreateConversation;
use crate::server::CreateMessage;

pub struct Mutation;

impl Mutation {
    pub async fn create_conversation(db: &DatabaseConnection, conversation: Json<CreateConversation>) ->
    Result<conversation::Model, DbErr>
    {
        conversation::ActiveModel {
            state: Set(conversation.state.to_owned()),
            ..Default::default()
        }.insert(db).await
    }
}

impl Mutation {
    pub async fn create_message(
        db: &DbConn,
        message: CreateMessage,
    ) -> Result<message::Model, DbErr> {
        message::ActiveModel {
            message_type: Set(message.message_type.to_owned()),
            conversation_id: Set(message.conversation_id.to_owned()),
            recipient_id: Set(message.recipient_id.to_owned()),
            sender_id: Set(message.sender_id.to_owned()),
            text: Set(message.text.to_owned()),
            ..Default::default()
        }.insert(db).await
    }
}
