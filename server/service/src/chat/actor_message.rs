use actix::{Message, Recipient};
use entity::message::Model;
use entity::sea_orm_active_enums::MessageType;
use sea_orm::prelude::{DateTime, Uuid};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotifyMessage {
    pub id: Option<Uuid>,
    pub message_type: MessageType,
    pub text: String,
    pub created_at: Option<DateTime>,
    pub recipient_id: Uuid,
    pub sender_id: Uuid,
    pub conversation_id: Uuid,
}

impl From<Model> for NotifyMessage {
    fn from(value: Model) -> Self {
        NotifyMessage {
            id: Some(value.id),
            message_type: value.message_type,
            text: value.text,
            created_at: Some(value.created_at),
            recipient_id: value.recipient_id,
            sender_id: value.sender_id,
            conversation_id: value.conversation_id,
        }
    }
}

// TODO: This is probably not even necessary
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotifyReceived {
    pub message_id: Uuid,
    pub recipient_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotifyRead {
    pub read_at: DateTime,
    pub message_id: Uuid,
    pub recipient_id: Uuid,
}

impl From<Model> for NotifyRead {
    fn from(value: Model) -> Self {
        NotifyRead {
            message_id: value.id,
            read_at: value.read_at.unwrap(),
            recipient_id: value.recipient_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotifyAuth {
    pub id: Uuid,
    pub cookie: Option<String>,
}

#[derive(Debug, Message, Clone, Deserialize, Serialize)]
#[rtype(result = "()")]
#[serde(tag = "type")]
pub enum Notification {
    Message(NotifyMessage),
    Received(NotifyReceived),
    Read(NotifyRead),
    Auth(NotifyAuth),
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: Uuid,
    pub addr: Recipient<Notification>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
}
