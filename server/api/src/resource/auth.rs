use actix_identity::Identity;
use actix_session::Session;
use actix_web::{error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use entity::{account, prelude::Account};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use std::any::Any;

use crate::AppState;

#[get("/whoami")]
async fn whoami(user: Option<Identity>) -> Result<impl Responder> {
    let message = if let Some(user) = user {
        format!("Welcome! {}", user.id().unwrap())
    } else {
        "Welcome Anonymous!".to_owned()
    };

    Ok(message)
}

#[post("/login/{account_name}")]
async fn login(
    request: HttpRequest,
    path: web::Path<String>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let account_name: String = path.into_inner().parse().unwrap();

    let account = Account::find()
        .filter(account::Column::Name.eq(&account_name))
        .one(&data.conn)
        .await
        .map_err(|err| error::ErrorServiceUnavailable(err))?;

    let account = account
        .ok_or("Failed to find associated account")
        .map_err(|err| error::ErrorNotFound(err))?;

    let account_id = account.id.to_string();

    let _ = Identity::login(&request.extensions(), account_id.to_owned());
    Ok(account_id)
}

#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::Ok()
}

pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(login)
            .service(logout)
            .service(whoami),
    );
}
