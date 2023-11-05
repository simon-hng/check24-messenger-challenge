use crate::chat::actor_message::NotifyMessage;
use crate::Mutation;
use chrono::Utc;
use entity::prelude::*;
use sea_orm::prelude::*;
use sea_orm::Set;

impl Mutation {
    pub async fn create_message(
        db: &DbConn,
        message: NotifyMessage,
    ) -> Result<entity::message::Model, DbErr> {
        entity::message::ActiveModel {
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

    pub async fn update_message_read(
        db: &DbConn,
        message_id: Uuid,
    ) -> Result<entity::message::Model, DbErr> {
        let message: Option<entity::message::Model> =
            Message::find_by_id(message_id).one(db).await?;

        let mut message: entity::message::ActiveModel = message.unwrap().into();
        message.read_at = Set(Some(Utc::now().naive_utc()));

        message.update(db).await
    }
}
