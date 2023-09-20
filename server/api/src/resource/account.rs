use actix_web::{get, web, Responder, Result};
use entity::prelude::Account;
use sea_orm::EntityTrait;

use crate::AppState;

#[get("/")]
pub async fn list_accounts(data: web::Data<AppState>) -> Result<impl Responder> {
    let conn = &data.conn;
    let accounts = Account::find()
        .all(conn)
        .await
        .expect("Failed to load accounts");

    Ok(web::Json(accounts))
}
