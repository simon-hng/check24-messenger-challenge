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
    use crate::schema::Conversation::dsl::*;

    let connection = &mut establish_connection();
    let results = Conversation
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
            hb: Instant::now(),
            room: "main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
