use std::time::Instant;

use actix::prelude::*;
use actix_web::cookie::{Cookie, CookieJar};
use actix_web_actors::ws;
use serde::Deserialize;
use entity::account;
use entity::account::Model;
use entity::sea_orm_active_enums::MessageType;

use super::server;

pub struct WsChatSession {
    pub heart_beat: Instant,
    pub addr: Addr<server::MessageServer>,
    pub account: Option<account::Model>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        Running::Stop
    }
}

// Takes message from the server and forwards it to the client
impl Handler<server::ServerMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: server::ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };


        match msg {
            ws::Message::Text(msg) => {
                // TODO: Error handling - unwrap unwrap obv. not good
                let message: WSMessage = serde_json::from_str(std::str::from_utf8(msg.as_ref()).unwrap()).unwrap();

                let account = match self.account.to_owned() {
                    None => {
                        if let WSMessage::AuthMessage { id, .. } = message {
                            let session_addr = ctx.address();
                            self.addr.send(server::Connect {
                                id,
                                addr: session_addr.recipient(),
                            }).into_actor(self).then(|_, _, _| {
                                fut::ready(())
                            }).wait(ctx);
                        }

                        return;
                    }
                    Some(account) => { account }
                };

                match message {
                    WSMessage::ChatMessage { text, recipient_id, conversation_id, message_type } => {
                        log::debug!("Received chat message\n{}", text);
                        self.addr.send(server::CreateMessage {
                            message_type: None,
                            text,
                            sender_id: account.id,
                            recipient_id,
                            conversation_id,
                        }).into_actor(self).then(|_, _, _| {
                            fut::ready(())
                        }).wait(ctx)
                    }
                    _ => {}
                }
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum WSMessage {
    ChatMessage { text: String, recipient_id: i32, conversation_id: i32, message_type: Option<MessageType> },
    AuthMessage { id: i32, cookie: Option<String> },
}
