use actix::Addr;
use actix_identity::Identity;
use actix_web::*;
use entity::app::AppState;
use sea_orm::prelude::Uuid;
use service::actor_message::{Notification, NotifyMessage, NotifyRead};
use service::{server, Mutation};

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
    server: web::Data<Addr<server::NotificationServer>>,
    notification: web::Json<NotifyMessage>,
    user: Option<Identity>,
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<impl Responder> {
    let user_id = get_user_id(user)?;
    let conversation_id = path.into_inner();

    let mut notification = notification.into_inner();
    notification.sender_id = user_id;
    notification.conversation_id = conversation_id;

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

#[post("/notify_read/")]
async fn notify_read(
    server: web::Data<Addr<server::NotificationServer>>,
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

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/message").service(post_message));
}
