use actix_web::web::Json;
use entity::{conversation, conversation_account, message};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbConn, DbErr, Set};
use entity::conversation::CreateConversation;
use crate::server::CreateMessage;

pub struct Mutation;

impl Mutation {
    pub async fn create_conversation(db: &DatabaseConnection, conversation: Json<CreateConversation>, sender_id: i32) ->
    Result<conversation::Model, DbErr>
    {
        let db_conversation = conversation::ActiveModel {
            state: Set(conversation.state.to_owned()),
            ..Default::default()
        }.insert(db).await?;

        conversation_account::ActiveModel {
            conversation_id: Set(db_conversation.id.to_owned()),
            account_id: Set(conversation.recipient.id.to_owned()),
        }.insert(db).await?;

        conversation_account::ActiveModel {
            conversation_id: Set(db_conversation.id.to_owned()),
            account_id: Set(sender_id),
        }.insert(db).await?;

        Ok(db_conversation)
    }
}

impl Mutation {
    pub async fn create_message(
        db: &DbConn,
        message: CreateMessage,
    ) -> Result<message::Model, DbErr> {
        log::info!("Adding message to DB {:?}", message);
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
