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

#[get("/{id}")]
pub async fn get_account_by_id(path: web::Path<String>) -> Result<impl Responder> {
    /*
    use crate::schema::account::dsl::*;

    let account_id: i32 = path.into_inner().parse().unwrap();
    let connection = &mut establish_connection();
    let results = account
        .find(account_id)
        .first::<crate::models::Account>(connection)
        .expect("failed to load accounts");
    */
    Ok("TODO")
}
