use std::time::Instant;

use actix::Addr;
use actix_identity::Identity;
use actix_web::*;
use actix_web_actors::ws;

use crate::resource::{
    server::{self, ClientMessage},
    session,
};

#[post("/")]
async fn post_message(
    server: web::Data<Addr<server::MessageServer>>,
    message: web::Json<ClientMessage>,
    user: Option<Identity>,
) -> Result<impl Responder> {
    let user = user
        .ok_or("Not Authenticated")
        .map_err(|err| error::ErrorUnauthorized(err))?;
    let user_id = user.id().map_err(|err| error::ErrorUnauthorized(err))?;
    let user_id: Option<i32> = user_id.parse().ok();

    let mut msg = message.into_inner();
    msg.sender_id = user_id;
    let result = server
        .send(msg)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    // TODO: This should return the newly created resource
    Ok(HttpResponse::Created().body("data"))
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
