use crate::Query;
use ::entity::{
    conversation,
    prelude::{Account, Conversation},
};
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_conversation_by_account_id(
        db: &DbConn,
        account_id: Uuid,
    ) -> Result<Vec<conversation::Model>, DbErr> {
        let account = Account::find_by_id(account_id).one(db).await?;

        let account = account
            .ok_or("Failed to find associated account")
            .map_err(|err| DbErr::RecordNotFound(err.to_string()))?;

        let conversations = account.find_related(Conversation).all(db).await;

        conversations
    }

    pub async fn find_conversation_by_id(
        db: &DbConn,
        conversation_id: Uuid,
    ) -> Result<Option<conversation::Model>, DbErr> {
        let conversation = Conversation::find_by_id(conversation_id).one(db).await;

        conversation
    }
}
