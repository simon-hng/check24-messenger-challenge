use actix::{Message, Recipient};
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
    pub attachments: Option<Vec<String>>,
}

impl From<entity::message::Model> for NotifyMessage {
    fn from(value: entity::message::Model) -> Self {
        NotifyMessage {
            id: Some(value.id),
            message_type: value.message_type,
            text: value.text,
            created_at: Some(value.created_at),
            recipient_id: value.recipient_id,
            sender_id: value.sender_id,
            conversation_id: value.conversation_id,
            attachments: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotifyRead {
    pub read_at: DateTime,
    pub message_id: Uuid,
    pub sender_id: Uuid,
    pub conversation_id: Uuid,
}

impl From<entity::message::Model> for NotifyRead {
    fn from(value: entity::message::Model) -> Self {
        NotifyRead {
            message_id: value.id,
            read_at: value.read_at.unwrap(),
            sender_id: value.sender_id,
            conversation_id: value.conversation_id,
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
