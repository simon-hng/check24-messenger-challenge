use actix::{Message, Recipient};
use entity::sea_orm_active_enums::MessageType;
use sea_orm::prelude::Uuid;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct NotifyMessage {
    pub id: Option<Uuid>,
    pub message_type: MessageType,
    pub text: String,
    pub sender_id: Uuid,
    pub recipient_id: Uuid,
    pub conversation_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct NotifyReceived {
    pub id: Uuid,
    pub message_id: Uuid,
    pub sender_id: Uuid,
}

#[derive(Debug, Clone)]
pub struct NotifyRead {
    pub id: Uuid,
    pub message_id: Uuid,
    pub sender_id: Uuid,
}

#[derive(Debug, Message, Clone)]
#[rtype(result = "()")]
pub enum Notification {
    Message(NotifyMessage),
    Received(NotifyReceived),
    Read(NotifyRead),
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
