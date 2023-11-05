use crate::chat::actor_message::*;
use actix::prelude::*;
use actix_web_actors::ws;
use sea_orm::prelude::Uuid;
use serde_json::json;
use std::time::Instant;

use super::server;

pub struct WsNotifierSession {
    pub heart_beat: Instant,
    pub addr: Addr<server::NotificationServer>,
    pub account_id: Option<Uuid>,
}

impl Actor for WsNotifierSession {
    type Context = ws::WebsocketContext<Self>;

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        Running::Stop
    }
}

impl Handler<Notification> for WsNotifierSession {
    type Result = ();

    fn handle(&mut self, msg: Notification, ctx: &mut Self::Context) {
        let notification = serde_json::to_string(&msg).unwrap();
        ctx.text(notification);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsNotifierSession {
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

                let _account_id = match self.account_id.to_owned() {
                    None => {
                        if let Notification::Auth(NotifyAuth { id, .. }) = message {
                            let session_addr = ctx.address();
                            self.addr.do_send(Connect {
                                id,
                                addr: session_addr.recipient(),
                            });

                            self.account_id = Some(id);
                            let response = json!({
                                "type": "Confirm_auth",
                                "id": id,
                            });
                            ctx.text(serde_json::to_string(&response).unwrap());
                        }

                        return;
                    }
                    Some(account) => account,
                };

                self.addr.do_send(message);
            }
            _ => todo!(),
        }
    }
}
