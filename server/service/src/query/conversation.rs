use crate::Query;
use entity::prelude::*;
use sea_orm::prelude::*;

impl Query {
    pub async fn find_conversation_by_account_id(
        db: &DbConn,
        account_id: Uuid,
    ) -> Result<Vec<::entity::conversation::Model>, DbErr> {
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
    ) -> Result<Option<::entity::conversation::Model>, DbErr> {
        let conversation = Conversation::find_by_id(conversation_id).one(db).await;

        conversation
    }

    pub async fn get_conversation_dto(
        db: &DbConn,
        conversation_id: Uuid,
        account_id: Uuid,
    ) -> Result<::entity::dto::conversation_dto::ConversationDTO, DbErr> {
        let conversation = Query::find_conversation_by_id(db, conversation_id)
            .await?
            .ok_or(DbErr::RecordNotFound("not found".to_string()))?;

        let messages = Query::find_messages_by_conversation(
            db,
            conversation.to_owned(),
            Some(::entity::api::message_api::MessageQueryParams {
                limit: Some(10),
                before: None,
            }),
        )
        .await?;

        let participants = Query::find_account_by_conversation(db, conversation.to_owned()).await?;

        let partner = participants
            .iter()
            .find(|partner| partner.id != account_id)
            .unwrap();

        let response = ::entity::dto::conversation_dto::ConversationDTO {
            conversation,
            messages: Some(messages),
            partner: Some(partner.to_owned()),
            unread_messages_count: None,
        };

        Ok(response)
    }

    pub async fn get_conversation_dtos(
        db: &DbConn,
        user_id: Uuid,
    ) -> Result<Vec<::entity::dto::conversation_dto::ConversationDTO>, DbErr> {
        let conversations = Query::find_conversation_by_account_id(db, user_id).await?;

        let mut response: Vec<::entity::dto::conversation_dto::ConversationDTO> = Vec::new();

        for conversation in conversations.iter() {
            let messages = Query::find_messages_by_conversation(
                db,
                conversation.to_owned(),
                Some(::entity::api::message_api::MessageQueryParams {
                    limit: Some(1),
                    before: None,
                }),
            )
            .await?;

            let participants =
                Query::find_account_by_conversation(db, conversation.to_owned()).await?;

            let partner = participants
                .iter()
                .find(|partner| partner.id != user_id)
                .unwrap();

            let count_unread = Query::find_count_unread_messages_by_conversation_for_account(
                db,
                conversation.to_owned(),
                user_id,
            )
            .await?;

            response.push(::entity::dto::conversation_dto::ConversationDTO {
                conversation: conversation.to_owned(),
                messages: Some(messages),
                partner: Some(partner.to_owned()),
                unread_messages_count: Some(count_unread),
            })
        }

        Ok(response)
    }
}
