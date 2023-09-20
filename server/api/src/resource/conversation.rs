use std::time::Instant;

use ::entity::{
    account,
    prelude::{Account, Conversation},
};
use actix::*;
use actix_identity::Identity;
use actix_web::*;
use actix_web_actors::ws;
use sea_orm::*;
use serde::Serialize;

use crate::{
    resource::{server, session},
    AppState,
};

#[derive(Serialize)]
struct ConversationInfo {
    id: i32,
    name: String,
    last_message: Option<String>,
    updated_at: chrono::NaiveDateTime,
    count_unread: i32,
}

#[get("/")]
async fn get_conversations(
    user: Option<Identity>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let user = user.expect("Not authenticated");
    let account = Account::find()
        .filter(account::Column::AccountName.eq(user.id().expect("Id should be set")))
        .one(&data.conn)
        .await
        .unwrap()
        .unwrap();
    // TODO: This panics if no conversations are found. Return 204 - No Content instead

    let conversations = account
        .find_related(Conversation)
        .all(&data.conn)
        .await
        .unwrap();

    Ok(web::Json(conversations))
}

#[derive(Serialize)]
struct ConversationDetail {
    id: i32,
    name: String,
    last_message: Option<String>,
    updated_at: chrono::NaiveDateTime,
    count_unread: i32,
}

#[get("/detail/{id}")]
async fn get_conversation_by_id(
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let conversation_id: i32 = path.into_inner().parse().unwrap();

    let conversation = Conversation::find_by_id(conversation_id)
        .one(&data.conn)
        .await
        .unwrap();

    Ok(web::Json(conversation))
}

#[get("/ws")]
async fn chat_route(
    path: web::Path<String>,
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let conversation_id = path.into_inner();

    ws::start(
        session::WsChatSession {
            id: 0,
            heart_beat: Instant::now(),
            room: conversation_id,
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/conversation")
            .service(get_conversation_by_id)
            .service(get_conversations),
    );
}
