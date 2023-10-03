use ::entity::{
    account,
    prelude::{Account, Conversation},
};
use actix_identity::Identity;
use actix_web::{error, get, web, Responder, Result};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serde::Serialize;

use crate::AppState;

#[derive(Serialize)]
struct ConversationInfo {
    id: i32,
    name: String,
    last_message: Option<String>,
    updated_at: chrono::NaiveDateTime,
    count_unread: i32,
}

#[get("/")]
async fn get_conversations(user: Identity, data: web::Data<AppState>) -> Result<impl Responder> {
    let user_id: i32 = user
        .id()
        .map_err(|err| error::ErrorUnauthorized(err))?
        .parse()
        .map_err(|err| error::ErrorUnauthorized(err))?;

    let account = Account::find_by_id(user_id)
        .one(&data.conn)
        .await
        .map_err(|err| error::ErrorServiceUnavailable(err))?;

    let account = account
        .ok_or("Failed to find associated account")
        .map_err(|err| error::ErrorNotFound(err))?;

    let conversations = account
        .find_related(Conversation)
        .all(&data.conn)
        .await
        .unwrap_or(Vec::new());

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

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/conversation")
            .service(get_conversation_by_id)
            .service(get_conversations),
    );
}
