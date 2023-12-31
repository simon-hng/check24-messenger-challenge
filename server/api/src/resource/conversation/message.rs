use crate::resource::auth::get_user_id;
use actix::Addr;
use actix_identity::Identity;
use actix_web::*;
use entity::app::AppState;
use entity::sea_orm_active_enums::{ConversationState, MessageType};
use sea_orm::prelude::Uuid;
use service::{
    chat::{actor_message::*, server},
    Mutation, Query,
};

#[get("")]
async fn get_messages(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
    query: web::Query<entity::api::message_api::MessageQueryParams>,
) -> Result<impl Responder> {
    let conversation_id = path.into_inner();

    let conversation = Query::find_conversation_by_id(&data.conn, conversation_id)
        .await
        .map_err(|err| match err {
            sea_orm::DbErr::Conn(message) => error::ErrorServiceUnavailable(message),
            _ => error::ErrorInternalServerError(""),
        })?
        .ok_or(error::ErrorNotFound("Conversation not found"))?;

    let params: Option<entity::api::message_api::MessageQueryParams> =
        query.into_inner().try_into().ok();

    let messages = Query::find_messages_by_conversation(&data.conn, conversation, params)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(web::Json(messages))
}

#[post("")]
async fn post_message(
    data: web::Data<AppState>,
    server: web::Data<Addr<server::NotificationServer>>,
    user: Identity,
    notification: web::Json<NotifyMessage>,
    path: web::Path<Uuid>,
) -> Result<impl Responder> {
    let user_id = get_user_id(user)?;
    let conversation_id = path.into_inner();

    let mut notification = notification.into_inner();
    notification.sender_id = user_id;
    notification.conversation_id = conversation_id;

    let conversation_state = match notification.message_type {
        MessageType::AcceptQuote => Some(ConversationState::Accepted),
        MessageType::RejectQuote => Some(ConversationState::Rejected),
        _ => None,
    };

    if let Some(conversation_state) = conversation_state {
        Mutation::update_conversation_state(&data.conn, conversation_id, conversation_state)
            .await
            .map_err(|err| error::ErrorInternalServerError(err))?;
    }

    let db_msg = Mutation::create_message(&data.conn, notification.to_owned())
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    if let Some(attachments) = notification.attachments.to_owned() {
        Mutation::create_message_attachments(&data.conn, db_msg.id, attachments)
            .await
            .map_err(|err| error::ErrorInternalServerError(err))?;
    }

    let mut out_notification: NotifyMessage = db_msg.to_owned().into();

    // Reattach B64 encoded attachment to notification
    out_notification.attachments = notification.attachments;

    server
        .send(Notification::Message(out_notification))
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::Created().json(db_msg))
}

#[post("/{message_id}/read")]
async fn notify_read(
    server: web::Data<Addr<server::NotificationServer>>,
    user: Identity,
    data: web::Data<AppState>,
    path: web::Path<(Uuid, Uuid)>,
) -> Result<impl Responder> {
    let _user_id = get_user_id(user)?;
    let (_conversation_id, message_id) = path.into_inner();

    let db_msg = Mutation::update_message_read(&data.conn, message_id)
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
    cfg.service(
        web::scope("/message")
            .service(get_messages)
            .service(post_message)
            .service(notify_read),
    );
}
