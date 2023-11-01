mod message;

use crate::resource::auth::get_user_id;
use crate::AppState;
use actix_identity::Identity;
use actix_web::*;
use entity::active::NewConversation;
use sea_orm::prelude::Uuid;
use service::{Mutation, Query};

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

    let response = Query::get_conversation_dtos(&data.conn, user_id)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(web::Json(response))
}

#[get("/{id}")]
async fn get_conversation_by_id(
    user: Identity,
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let account_id = get_user_id(user)?;

    let conversation_id = path.into_inner();

    let response = Query::get_conversation_dto(&data.conn, conversation_id, account_id)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

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
