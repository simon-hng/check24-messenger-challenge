use crate::Query;
use ::entity::{account, conversation, prelude::Account};
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_account_by_id(
        db: &DbConn,
        account_id: Uuid,
    ) -> Result<Option<account::Model>, DbErr> {
        Account::find_by_id(account_id).one(db).await
    }

    pub async fn find_account_by_name(
        db: &DbConn,
        account_name: String,
    ) -> Result<Option<account::Model>, DbErr> {
        Account::find()
            .filter(account::Column::Name.eq(&account_name))
            .one(db)
            .await
    }

    pub async fn find_account_by_conversation(
        db: &DbConn,
        conversation: conversation::Model,
    ) -> Result<Vec<account::Model>, DbErr> {
        conversation.find_related(Account).all(db).await
    }
}
