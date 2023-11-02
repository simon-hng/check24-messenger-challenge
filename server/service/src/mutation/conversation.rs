use crate::Mutation;
use actix_web::web::Json;
use sea_orm::prelude::*;
use sea_orm::Set;

impl Mutation {
    pub async fn create_conversation(
        db: &DatabaseConnection,
        conversation: Json<entity::active::NewConversation>,
        sender_id: Uuid,
    ) -> Result<entity::conversation::Model, DbErr> {
        let db_conversation = entity::conversation::ActiveModel {
            state: Set(conversation.state.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        entity::conversation_account::ActiveModel {
            conversation_id: Set(db_conversation.id.to_owned()),
            account_id: Set(conversation.recipient.id.to_owned()),
        }
        .insert(db)
        .await?;

        entity::conversation_account::ActiveModel {
            conversation_id: Set(db_conversation.id.to_owned()),
            account_id: Set(sender_id),
        }
        .insert(db)
        .await?;

        Ok(db_conversation)
    }
}
