use actix_identity::Identity;
use actix_web::*;
use entity::active::NewConversation;
use sea_orm::prelude::Uuid;
use serde::Serialize;
use service::{Mutation, Query};

use crate::AppState;

#[derive(Serialize)]
struct ConversationInfo {
    id: i32,
    name: String,
    last_message: Option<String>,
    updated_at: chrono::NaiveDateTime,
    count_unread: i32,
}

#[post("/")]
async fn create_conversation(
    user: Identity,
    data: web::Data<AppState>,
    conversation: web::Json<NewConversation>,
) -> Result<impl Responder> {
    let user_id: Uuid = user
        .id()
        .map_err(|err| error::ErrorUnauthorized(err))?
        .parse()
        .map_err(|err| error::ErrorUnauthorized(err))?;

    let db_conversation = Mutation::create_conversation(&data.conn, conversation, user_id)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::Created().json(db_conversation))
}

#[get("/")]
async fn get_conversations(user: Identity, data: web::Data<AppState>) -> Result<impl Responder> {
    let user_id = user
        .id()
        .map_err(|err| error::ErrorUnauthorized(err))?
        .parse()
        .map_err(|err| error::ErrorUnauthorized(err))?;

    let conversations = Query::find_conversation_by_account_id(&data.conn, user_id)
        .await
        .map_err(|err| match err {
            sea_orm::DbErr::Conn(message) => error::ErrorServiceUnavailable(message),
            sea_orm::DbErr::RecordNotFound(message) => error::ErrorNotFound(message),
            _ => error::ErrorInternalServerError(""),
        })?;

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
    let conversation_id = path.into_inner().parse().unwrap();

    let conversation = Query::find_conversation_by_id(&data.conn, conversation_id)
        .await
        .map_err(|err| match err {
            sea_orm::DbErr::Conn(message) => error::ErrorServiceUnavailable(message),
            sea_orm::DbErr::RecordNotFound(message) => error::ErrorNotFound(message),
            _ => error::ErrorInternalServerError(""),
        })?;

    Ok(web::Json(conversation))
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/conversation")
            .service(get_conversation_by_id)
            .service(get_conversations)
            .service(create_conversation),
    );
}
