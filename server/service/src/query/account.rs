use crate::Query;
use entity::prelude::*;
use sea_orm::prelude::*;

impl Query {
    pub async fn find_account_by_id(
        db: &DbConn,
        account_id: Uuid,
    ) -> Result<Option<entity::account::Model>, DbErr> {
        Account::find_by_id(account_id).one(db).await
    }

    pub async fn find_account_by_name(
        db: &DbConn,
        account_name: String,
    ) -> Result<Option<entity::account::Model>, DbErr> {
        Account::find()
            .filter(entity::account::Column::Name.eq(&account_name))
            .one(db)
            .await
    }

    pub async fn find_account_by_conversation(
        db: &DbConn,
        conversation: entity::conversation::Model,
    ) -> Result<Vec<entity::account::Model>, DbErr> {
        conversation.find_related(Account).all(db).await
    }
}
