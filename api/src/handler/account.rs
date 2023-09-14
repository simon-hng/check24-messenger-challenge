use crate::{establish_connection, models::Account};
use actix_web::{get, web, Responder, Result};
use diesel::prelude::*;

#[get("/")]
pub async fn list_accounts() -> Result<impl Responder> {
    use crate::schema::account::dsl::*;

    let connection = &mut establish_connection();
    let results = account
        .select(Account::as_select())
        .load(connection)
        .expect("failed to load accounts");

    Ok(web::Json(results))
}

#[get("/{id}")]
pub async fn get_account_by_id(path: web::Path<String>) -> Result<impl Responder> {
    use crate::schema::account::dsl::*;

    let account_id: i32 = path.into_inner().parse().unwrap();
    let connection = &mut establish_connection();
    let results = account
        .filter(id.eq(account_id))
        .first::<crate::models::Account>(connection)
        .expect("failed to load accounts");

    Ok(web::Json(results))
}
