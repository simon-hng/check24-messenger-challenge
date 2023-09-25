use std::time::Instant;

use actix::prelude::*;
use actix_session::Session;
use actix_web_actors::ws;

pub struct WsChatSession {
    pub id: usize,
    pub heart_beat: Instant,
    pub session: Session,
}

struct WhoAmI;

impl Message for WhoAmI {
    type Result = Result<actix::Addr<WsChatSession>, ()>;
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
        let recipient = addr.recipient();
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.session.remove("socket");
        Running::Stop
    }
}

impl Handler<WhoAmI> for WsChatSession {
    type Result = Result<actix::Addr<WsChatSession>, ()>;

    fn handle(&mut self, msg: WhoAmI, ctx: &mut Self::Context) -> Self::Result {
        Ok(ctx.address())
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
