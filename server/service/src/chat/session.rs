use crate::actor_message::{Connect, NotifyAuth};
use crate::chat::actor_message::Notification;
use actix::prelude::*;
use actix_web_actors::ws;
use sea_orm::prelude::Uuid;
use std::time::Instant;

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
            Notification::Message(notification) => {
                let notification = serde_json::to_string(&notification).unwrap();
                ctx.text(notification);
            }
            _ => {}
        }
    }
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
                let message: Notification =
                    serde_json::from_str(std::str::from_utf8(msg.as_ref()).unwrap()).unwrap();

                let account_id = match self.account_id.to_owned() {
                    None => {
                        if let Notification::Auth(NotifyAuth { id, .. }) = message {
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
                    _ => {}
                }
            }
            _ => todo!(),
        }
    }
}
