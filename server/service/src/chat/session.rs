use std::time::Instant;

use actix::prelude::*;
use actix_web_actors::ws;

use super::server;

pub struct WsChatSession {
    pub heart_beat: Instant,
    pub addr: Addr<server::MessageServer>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /*
     * TODO: Move this to message handler
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        self.addr.send(server::Connect {
            id,
            addr: addr.recipient(),
        });
    }
    */

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

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Text(msg) => {
                let addr = ctx.address();
                self.addr.send(server::Connect {
                    id: msg.to_string(),
                    addr: addr.recipient(),
                });
            }
            ws::Message::Binary(_) => todo!(),
            ws::Message::Continuation(_) => todo!(),
            ws::Message::Ping(_) => todo!(),
            ws::Message::Pong(_) => todo!(),
            ws::Message::Close(_) => todo!(),
            ws::Message::Nop => todo!(),
        }
    }
}
