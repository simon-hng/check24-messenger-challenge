use actix::{Message, Recipient};
use entity::sea_orm_active_enums::MessageType;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct NotifyMessage {
    pub message_type: MessageType,
    pub text: String,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub conversation_id: i32,
}

#[derive(Debug, Clone)]
pub struct NotifyReceived {
    pub sender_id: i32,
}

#[derive(Debug, Message, Clone)]
#[rtype(result = "()")]
pub enum Notification {
    Message(NotifyMessage),
    Received(NotifyReceived),
}

#[derive(Message)]
#[rtype(result = "i32")]
pub struct Connect {
    pub id: i32,
    pub addr: Recipient<Notification>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i32,
}
