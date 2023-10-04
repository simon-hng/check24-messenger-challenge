use std::time::Instant;

use actix::Addr;
use actix_identity::Identity;
use actix_web::*;
use actix_web_actors::ws;
use entity::{message, prelude::Message};
use sea_orm::{ActiveValue, EntityTrait};

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

    let _ = server.send(msg).await;

    let message = message::ActiveModel {
        message_type: ActiveValue::Set(message.message_type.clone()),
        conversation_id: ActiveValue::Set(message.conversation_id),
        recipient_id: ActiveValue::Set(message.recipient_id),
        sender_id: ActiveValue::Set(user_id),
        text: ActiveValue::Set(message.text.to_owned()),
        ..Default::default()
    };

    Message::insert(message);

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
