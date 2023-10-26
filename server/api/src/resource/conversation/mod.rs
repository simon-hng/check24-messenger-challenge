mod message;
use crate::AppState;
use actix_identity::Identity;
use actix_web::*;
use entity::account;
use entity::active::NewConversation;
use entity::conversation::Model;
use sea_orm::prelude::Uuid;
use serde::Serialize;
use service::{Mutation, Query};

#[derive(Serialize)]
struct ConversationInfo {
    id: i32,
    name: String,
    last_message: Option<String>,
    updated_at: chrono::NaiveDateTime,
    count_unread: i32,
}

#[post("")]
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

#[get("")]
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

    let mut response: Vec<ConversationDTO> = Vec::new();

    for conversation in conversations.iter() {
        let participants = Query::find_account_by_conversation(&data.conn, conversation.to_owned())
            .await
            .map_err(|err| error::ErrorInternalServerError(err))?;

        response.push(ConversationDTO {
            conversation: conversation.to_owned(),
            participants: Some(participants),
            messages: None,
        })
    }

    Ok(web::Json(response))
}

#[derive(Serialize)]
struct ConversationDTO {
    conversation: Model,
    participants: Option<Vec<account::Model>>,
    messages: Option<Vec<entity::message::Model>>,
}
#[get("/{id}")]
async fn get_conversation_by_id(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let conversation_id = path.into_inner();

    let conversation = Query::find_conversation_by_id(&data.conn, conversation_id)
        .await
        .map_err(|err| match err {
            sea_orm::DbErr::Conn(message) => error::ErrorServiceUnavailable(message),
            _ => error::ErrorInternalServerError(""),
        })?
        .ok_or(error::ErrorNotFound("Conversation not found"))?;

    let accounts = Query::find_account_by_conversation(&data.conn, conversation.to_owned())
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    let messages = Query::find_messages_by_conversation(&data.conn, conversation.to_owned())
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    let response = ConversationDTO {
        conversation,
        participants: Some(accounts),
        messages: Some(messages),
    };

    Ok(web::Json(response))
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/conversation")
            .service(get_conversation_by_id)
            .service(get_conversations)
            .service(create_conversation)
            .service(web::scope("/{id}").configure(message::init_service)),
    );
}
