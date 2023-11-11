use crate::Mutation;
use sea_orm::prelude::*;
use sea_orm::Set;

impl Mutation {
    pub async fn create_account(
        db: &DbConn,
        account: entity::dto::account::CreateAccountDTO,
    ) -> Result<entity::account::Model, DbErr> {
        entity::account::ActiveModel {
            name: Set(account.name),
            picture: Set(account.picture),
            account_type: Set(account.account_type),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
