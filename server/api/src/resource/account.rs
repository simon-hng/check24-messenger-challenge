use actix_web::*;
use entity::prelude::Account;
use sea_orm::EntityTrait;
use service::{Mutation, Query};

use crate::AppState;

#[get("")]
pub async fn list_accounts(data: web::Data<AppState>) -> Result<impl Responder> {
    let accounts = Account::find().all(&data.conn).await.map_err(|err| {
        error::ErrorInternalServerError(format!("Failed to list accounts: {}", err))
    })?;

    Ok(web::Json(accounts))
}

#[post("")]
pub async fn create_account(
    data: web::Data<AppState>,
    account: web::Json<entity::dto::account::CreateAccountDTO>,
) -> Result<impl Responder> {
    let account = account.into_inner();

    let existing = Query::find_account_by_name(&data.conn, account.name.to_owned())
        .await
        .map_err(|err| {
            error::ErrorInternalServerError(format!("Failed to find account by name: {}", err))
        })?;

    if existing.is_some() {
        return Err(error::ErrorBadRequest(format!(
            "Account with name {} already exists",
            account.name
        )));
    }

    let db_account = Mutation::create_account(&data.conn, account)
        .await
        .map_err(|err| {
            error::ErrorInternalServerError(format!("Failed to create account: {}", err))
        })?;

    Ok(web::Json(db_account))
}
pub fn init_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/account")
            .service(list_accounts)
            .service(create_account),
    );
}
