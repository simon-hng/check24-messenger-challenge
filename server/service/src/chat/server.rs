use std::collections::HashMap;

use actix::prelude::*;
use actix_web::cookie::{Key};
use entity::{sea_orm_active_enums::MessageType};
use sea_orm::{DatabaseConnection};
use serde::Deserialize;
use entity::app::AppState;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ServerMessage(pub String);

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
#[rtype(result = "()")]
pub struct Connect {
    pub id: i32,
    pub addr: Recipient<ServerMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: i32,
}

pub struct MessageServer {
    db_connection: DatabaseConnection,
    sessions: HashMap<i32, Recipient<ServerMessage>>,
    key: Key,
}

impl MessageServer {
    pub fn new(app_state: AppState, key: Key) -> MessageServer {
        MessageServer {
            key,
            db_connection: app_state.conn,
            sessions: HashMap::new(),
        }
    }
}

impl Actor for MessageServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MessageServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        /*
         TODO: I wanted to check the cookie but this is getting out of hand so we just
               believe the client that it is what they say they are
         */
        self.sessions.insert(msg.id.to_owned(), msg.addr);
        log::info!("Added {} to the session", msg.id);
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

    fn handle(&mut self, msg: CreateMessage, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("Received a chat message {:?}", msg);
        let recipient = self
            .sessions
            .get(&msg.recipient_id)
            .ok_or("Recipient not found")
            .expect("TODO return error");
    }
}
