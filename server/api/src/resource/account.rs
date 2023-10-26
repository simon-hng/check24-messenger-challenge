use actix_web::{error, get, web, Responder, Result};
use entity::prelude::Account;
use sea_orm::prelude::Uuid;
use sea_orm::EntityTrait;

use crate::AppState;

#[get("/")]
pub async fn list_accounts(data: web::Data<AppState>) -> Result<impl Responder> {
    let accounts = Account::find()
        .all(&data.conn)
        .await
        .expect("Failed to load accounts");

    Ok(web::Json(accounts))
}

#[get("/{id}")]
pub async fn get_account_by_id(
    data: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> Result<impl Responder> {
    let account_id = path.into_inner();

    let account = Account::find_by_id(account_id)
        .one(&data.conn)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(web::Json(account))
}
