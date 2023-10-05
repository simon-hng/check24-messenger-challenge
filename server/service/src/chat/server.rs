use std::collections::HashMap;

use actix::prelude::*;
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
    pub addr: Recipient<ServerMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: String,
}

#[derive(Debug)]
pub struct MessageServer {
    db_connection: DatabaseConnection,
    sessions: HashMap<String, Recipient<ServerMessage>>,
}

impl MessageServer {
    pub fn new(app_state: AppState) -> MessageServer {
        MessageServer {
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
        self.sessions.insert(msg.id, msg.addr);
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
        log::info!("Received a chat message");
        let _recipient = self
            .sessions
            .get(&msg.recipient_id.expect("should not happen").to_string())
            .ok_or("Recipient not found")
            .expect("TODO return error");
    }
}
