use actix_web::*;
use entity::prelude::Account;
use sea_orm::EntityTrait;
use service::Mutation;

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
