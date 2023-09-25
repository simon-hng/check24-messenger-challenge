use std::time::Instant;

use actix_session::Session;
use actix_web::*;
use actix_web_actors::ws;
use serde::Deserialize;

use super::session;

#[derive(Deserialize)]
struct Message {
    text: String,
}

#[post("/")]
async fn post_message(message: web::Json<Message>) -> Result<impl Responder> {
    Ok(format!("Sent message {}!", message.text))
}

#[get("/receive")]
async fn receive_messages(
    session: Session,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            heart_beat: Instant::now(),
            session,
        },
        &req,
        stream,
    )
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/message")
            .service(receive_messages)
            .service(post_message),
    );
}
