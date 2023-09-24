use std::time::Instant;

use actix::prelude::*;
use actix_session::Session;
use actix_web_actors::ws;

pub struct WsChatSession {
    pub id: usize,
    pub heart_beat: Instant,
    pub session: Session,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.session.insert("socket", "asd");
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.session.remove("socket");
        Running::Stop
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
    }
}
