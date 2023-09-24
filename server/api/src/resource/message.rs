use std::time::Instant;

use actix_session::Session;
use actix_web::{get, web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use super::session;

#[get("/receive")]
async fn receive_messages(
    session: Session,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let socket = ws::start(
        session::WsChatSession {
            id: 0,
            heart_beat: Instant::now(),
            session,
        },
        &req,
        stream,
    );

    socket
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/message").service(receive_messages));
}
