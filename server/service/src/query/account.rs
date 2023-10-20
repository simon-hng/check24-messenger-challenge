use crate::Query;
use ::entity::{account, prelude::Account};
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_account_by_id(
        db: &DbConn,
        account_id: Uuid,
    ) -> Result<Option<account::Model>, DbErr> {
        Account::find_by_id(account_id).one(db).await
    }
}
