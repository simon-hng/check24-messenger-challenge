use std::time::Instant;

use crate::{
    establish_connection,
    handler::{server, session},
    models,
};
use actix::*;
use actix_web::*;
use actix_web_actors::ws;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct ConversationInfo {
    id: i32,
    name: String,
    last_message: Option<String>,
    updated_at: chrono::NaiveDateTime,
    count_unread: i32,
}

#[get("/")]
async fn list_chats() -> Result<impl Responder> {
    use crate::schema::conversation::dsl::*;

    let connection = &mut establish_connection();
    let results = conversation
        .select(models::Conversation::as_select())
        .load(connection)
        .expect("Error loading conversations");

    let conversation_info: Vec<ConversationInfo> = results
        .iter()
        .map(|conv| ConversationInfo {
            id: conv.id,
            name: conv.customer_name.to_owned(),
            updated_at: conv.updated_at,
            count_unread: 1,
            last_message: None,
        })
        .collect();

    Ok(web::Json(conversation_info))
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
async fn get_chat_by_id(path: web::Path<String>) -> Result<impl Responder> {
    use crate::schema::conversation::dsl::*;

    let conversation_id: i32 = path.into_inner().parse().unwrap();
    let connection = &mut establish_connection();
    let conv = conversation
        .find(conversation_id)
        .first::<crate::models::Conversation>(connection)
        .expect("failed to load accounts");

    let response = ConversationInfo {
        id: conv.id,
        name: conv.customer_name.to_owned(),
        updated_at: conv.updated_at,
        count_unread: 1,
        last_message: None,
    };

    Ok(web::Json(response))
}

#[get("/{id}")]
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
