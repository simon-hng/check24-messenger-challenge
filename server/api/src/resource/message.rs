use std::time::Instant;

use actix::Addr;
use actix_identity::Identity;
use actix_web::*;
use actix_web_actors::ws;
use entity::app::AppState;
use sea_orm::TryIntoModel;
use service::server::CreateMessage;
use service::{server, session, Mutation};

#[post("/")]
async fn post_message(
    server: web::Data<Addr<server::MessageServer>>,
    message: web::Json<CreateMessage>,
    user: Option<Identity>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let user = user
        .ok_or("Not Authenticated")
        .map_err(|err| error::ErrorUnauthorized(err))?;
    let user_id = user.id().map_err(|err| error::ErrorUnauthorized(err))?;
    let user_id: i32 = user_id
        .parse()
        .map_err(|err| error::ErrorUnauthorized(err))?;

    let mut msg = message.into_inner();
    msg.sender_id = user_id;

    server
        .send(msg.to_owned())
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    let db_msg = Mutation::create_message(&data.conn, msg)
        .await
        .map(|db_message| db_message.try_into_model().expect("TODO"))
        .map_err(|err| error::ErrorInternalServerError(err))?;

    // TODO: Return db_msg
    Ok(HttpResponse::Created().json(db_msg))
}

#[get("/ws")]
async fn receive_messages(
    req: HttpRequest,
    stream: web::Payload,
    server: web::Data<Addr<server::MessageServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            heart_beat: Instant::now(),
            addr: server.get_ref().clone(),
            account_id: None,
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
