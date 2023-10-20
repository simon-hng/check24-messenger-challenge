use std::time::Instant;

use actix::Addr;
use actix_identity::Identity;
use actix_web::*;
use actix_web_actors::ws;
use entity::app::AppState;
use sea_orm::prelude::Uuid;
use service::actor_message::{Notification, NotifyMessage, NotifyRead};
use service::{server, session, Mutation};

fn get_user_id(user: Option<Identity>) -> Result<Uuid, Error> {
    let user = user
        .ok_or("Not Authenticated")
        .map_err(|err| error::ErrorUnauthorized(err))?;
    let user_id = user.id().map_err(|err| error::ErrorUnauthorized(err))?;
    let user_id = user_id
        .parse()
        .map_err(|err| error::ErrorUnauthorized(err))?;
    Ok(user_id)
}

#[post("/")]
async fn post_message(
    server: web::Data<Addr<server::MessageServer>>,
    notification: web::Json<NotifyMessage>,
    user: Option<Identity>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let user_id = get_user_id(user)?;

    let mut notification = notification.into_inner();
    notification.sender_id = user_id;

    let db_msg = Mutation::create_message(&data.conn, notification.to_owned())
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    let notification: NotifyMessage = db_msg.to_owned().into();

    server
        .send(Notification::Message(notification))
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::Created().json(db_msg))
}

#[post("/read/")]
async fn notify_read(
    server: web::Data<Addr<server::MessageServer>>,
    notification: web::Json<NotifyRead>,
    user: Option<Identity>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let _user_id = get_user_id(user)?;
    let notification = notification.into_inner();
    let db_msg = Mutation::update_message_read(&data.conn, notification)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    let notification: NotifyRead = db_msg.to_owned().into();

    server
        .send(Notification::Read(notification))
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(web::Json(db_msg))
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
