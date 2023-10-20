use crate::actor_message::{NotifyMessage, NotifyRead};
use crate::Mutation;
use entity::message;
use entity::message::Model;
use entity::prelude::Message;
use sea_orm::{ActiveModelTrait, DbConn, DbErr, EntityTrait, Set};

impl Mutation {
    pub async fn create_message(db: &DbConn, message: NotifyMessage) -> Result<Model, DbErr> {
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

    pub async fn update_message_read(db: &DbConn, notify: NotifyRead) -> Result<Model, DbErr> {
        let message: Option<Model> = Message::find_by_id(notify.message_id).one(db).await?;

        let mut message: message::ActiveModel = message.unwrap().into();
        message.read_at = Set(Some(notify.read_at));

        message.update(db).await
    }
}
