use std::time::Instant;

use actix::Addr;
use actix_identity::Identity;
use actix_web::*;
use actix_web_actors::ws;
use serde::Deserialize;

use super::{server, session};

#[derive(Deserialize)]
struct Message {
    text: String,
}

#[post("/")]
async fn post_message(message: web::Json<Message>) -> Result<impl Responder> {
    Ok(format!("Sent message {}!", message.text))
}

#[post("/send")]
async fn send_message(
    server: web::Data<Addr<server::MessageServer>>,
    json: web::Json<server::ClientMessage>,
) -> Result<impl Responder> {
    server.send(json.into_inner());

    Ok("ok")
}

#[get("/receive")]
async fn receive_messages(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<server::MessageServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            heart_beat: Instant::now(),
            addr: server.get_ref().clone(),
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
