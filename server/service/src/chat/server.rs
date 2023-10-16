use std::collections::HashMap;

use actix::prelude::*;
use actix_web::cookie::Key;
use entity::sea_orm_active_enums::MessageType;
use serde::Deserialize;

#[derive(Debug, Message, Deserialize, Clone)]
#[rtype(result = "()")]
pub struct CreateMessage {
    pub message_type: MessageType,
    pub text: String,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub conversation_id: i32,
}

#[derive(Message)]
#[rtype(result = "i32")]
pub struct Connect {
    pub id: i32,
    pub addr: Recipient<CreateMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i32,
}

pub struct MessageServer {
    sessions: HashMap<i32, Recipient<CreateMessage>>,
    _key: Key,
}

impl MessageServer {
    pub fn new(key: Key) -> MessageServer {
        MessageServer {
            _key: key,
            sessions: HashMap::new(),
        }
    }
}

impl Actor for MessageServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MessageServer {
    type Result = i32;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        /*
        TODO: I wanted to check the cookie but this is getting out of hand so we just
              believe the client that it is what they say they are
         */

        log::info!("Added {} to the sessions", msg.id);
        self.sessions.insert(msg.id.to_owned(), msg.addr);

        msg.id
    }
}

impl Handler<Disconnect> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<CreateMessage> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: CreateMessage, ctx: &mut Self::Context) -> Self::Result {
        log::info!("Forwarding message {:?}", msg);

        let recipient = self
            .sessions
            .get(&msg.recipient_id)
            .ok_or("Recipient not found")
            .expect("TODO return error");

        recipient
            .send(msg)
            .into_actor(self)
            .then(|_res, _act, _ctx| fut::ready(()))
            .wait(ctx);
    }
}
