use actix_identity::Identity;
use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use entity::{account, prelude::Account};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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
) -> impl Responder {
    let account_name: String = path.into_inner().parse().unwrap();

    let account = Account::find()
        .filter(account::Column::AccountName.eq(&account_name))
        .one(&data.conn)
        .await
        .unwrap()
        .unwrap();

    // TODO: Error handling. This needs to throw an error that is mapped to 404 not found

    let _ = Identity::login(&request.extensions(), account.id.to_string());
    HttpResponse::Ok()
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
