use actix_identity::Identity;

use actix_web::{error, get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder, Result};
use serde::Deserialize;
use service::Query;

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

#[derive(Deserialize)]
struct Login {
    account_name: String,
}

#[post("/login")]
async fn login(
    request: HttpRequest,
    login: web::Json<Login>,
    data: web::Data<AppState>,
) -> Result<impl Responder> {
    let account_name = login.into_inner().account_name;

    let account = Query::find_account_by_name(&data.conn, account_name)
        .await
        .map_err(|err| error::ErrorServiceUnavailable(err))?;

    let account = account
        .ok_or("Failed to find associated account")
        .map_err(|err| error::ErrorNotFound(err))?;

    let account_id = account.id.to_string();

    let _ = Identity::login(&request.extensions(), account_id.to_owned());
    Ok(web::Json(account))
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
