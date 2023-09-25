use std::collections::HashMap;

use actix::prelude::*;
use actix_identity::Identity;
use entity::sea_orm_active_enums::MessageType;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage {
    pub message_type: Option<MessageType>,
    pub text: String,
    pub sender_id: Option<i32>,
    pub recipient_id: Option<i32>,
    pub conversation_id: Option<i32>,
}

#[derive(Message)]
#[rtype(String)]
pub struct Connect {
    pub identity: Identity,
    pub addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub identity: Identity,
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
    type Result = String;

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        let id = msg.identity.id().expect("Not logged in");
        self.sessions.insert(id, msg.addr);

        id
    }
}

impl Handler<Disconnect> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        let id = msg.identity.id().expect("Not logged in");
        self.sessions.remove(&id);
    }
}

impl Handler<ClientMessage> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        let recipient = self
            .sessions
            .get(&msg.recipient_id.expect("should not happen").to_string());
    }
}
