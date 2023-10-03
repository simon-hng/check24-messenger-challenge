use std::collections::HashMap;

use actix::prelude::*;
use entity::sea_orm_active_enums::MessageType;
use serde::Deserialize;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Debug, Message, Deserialize)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub message_type: Option<MessageType>,
    pub text: String,
    pub sender_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub conversation_id: Option<i32>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: String,
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

#[derive(Debug)]
pub struct MessageServer {
    sessions: HashMap<String, Recipient<Message>>,
}

impl MessageServer {
    pub fn new() -> MessageServer {
        MessageServer {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for MessageServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<ClientMessage> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        let recipient = self
            .sessions
            .get(&msg.recipient_id.expect("should not happen").to_string())
            .ok_or("Recipient not found")
            .expect("TODO return error");

        print!("{:?}", msg)
    }
}
