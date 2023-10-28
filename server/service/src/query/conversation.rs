use crate::Query;
use ::entity::{
    conversation, dto,
    prelude::{Account, Conversation},
};
use actix::fut::ok;
use sea_orm::prelude::Uuid;
use sea_orm::*;

impl Query {
    pub async fn find_conversation_by_account_id(
        db: &DbConn,
        account_id: Uuid,
    ) -> Result<Vec<conversation::Model>, DbErr> {
        let account = Account::find_by_id(account_id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound("not found".to_string()))?;

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

    pub async fn get_conversation_dto(
        db: &DbConn,
        conversation_id: Uuid,
        account_id: Uuid,
    ) -> Result<dto::conversation_dto::ConversationDTO, DbErr> {
        let conversation = Query::find_conversation_by_id(db, conversation_id)
            .await?
            .ok_or(DbErr::RecordNotFound("not found".to_string()))?;

        let messages = Query::find_messages_by_conversation(db, conversation.to_owned()).await?;

        let participants = Query::find_account_by_conversation(db, conversation.to_owned()).await?;

        let partner = participants
            .iter()
            .find(|partner| partner.id != account_id)
            .unwrap();

        let response = dto::conversation_dto::ConversationDTO {
            conversation,
            messages: Some(messages),
            partner: Some(partner.to_owned()),
        };

        Ok(response)
    }

    pub async fn get_conversation_dtos_by_account_id(
        db: &DbConn,
        account_id: Uuid,
    ) -> Result<Vec<dto::conversation_dto::ConversationDTO>, DbErr> {
        let conversations = Query::find_conversation_by_account_id(db, account_id).await?;

        let mut response: Vec<dto::conversation_dto::ConversationDTO> = Vec::new();

        for conversation in conversations.iter() {
            let messages =
                Query::find_messages_by_conversation(db, conversation.to_owned()).await?;

            let participants =
                Query::find_account_by_conversation(db, conversation.to_owned()).await?;

            let partner = participants
                .iter()
                .find(|partner| partner.id != account_id)
                .unwrap();

            response.push(dto::conversation_dto::ConversationDTO {
                conversation: conversation.to_owned(),
                messages: Some(messages),
                partner: Some(partner.to_owned()),
            })
        }

        Ok(response)
    }
}
