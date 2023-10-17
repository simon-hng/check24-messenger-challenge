use std::time::Instant;

use crate::actor_message::{Connect, NotifyMessage};
use crate::chat::actor_message::Notification;
use actix::prelude::*;
use actix_web_actors::ws;
use entity::sea_orm_active_enums::MessageType;
use sea_orm::prelude::Uuid;
use serde::Deserialize;

use super::server;

pub struct WsChatSession {
    pub heart_beat: Instant,
    pub addr: Addr<server::MessageServer>,
    pub account_id: Option<Uuid>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        Running::Stop
    }
}

impl Handler<Notification> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: Notification, ctx: &mut Self::Context) {
        match msg {
            Notification::Message(NotifyMessage { text, .. }) => {
                ctx.text(text);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum WSMessage {
    ChatMessage {
        text: String,
        recipient_id: Uuid,
        conversation_id: Uuid,
        message_type: MessageType,
    },
    AuthMessage {
        id: Uuid,
        cookie: Option<String>,
    },
}

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
                let message: WSMessage =
                    serde_json::from_str(std::str::from_utf8(msg.as_ref()).unwrap()).unwrap();

                let account_id = match self.account_id.to_owned() {
                    None => {
                        if let WSMessage::AuthMessage { id, .. } = message {
                            let session_addr = ctx.address();
                            self.addr.do_send(Connect {
                                id,
                                addr: session_addr.recipient(),
                            });

                            self.account_id = Some(id);
                        }

                        return;
                    }
                    Some(account) => account,
                };

                match message {
                    WSMessage::ChatMessage {
                        text,
                        recipient_id,
                        conversation_id,
                        message_type,
                    } => {
                        log::debug!("Received chat message\n{}", text);

                        self.addr.do_send(Notification::Message(NotifyMessage {
                            message_type,
                            text,
                            sender_id: account_id,
                            recipient_id,
                            conversation_id,
                        }))
                    }
                    _ => {}
                }
            }
            _ => todo!(),
        }
    }
}
