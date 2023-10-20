use crate::Mutation;
use actix_web::web::Json;
use entity::active::NewConversation;
use entity::{conversation, conversation_account};
use sea_orm::prelude::Uuid;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, Set};

impl Mutation {
    pub async fn create_conversation(
        db: &DatabaseConnection,
        conversation: Json<NewConversation>,
        sender_id: Uuid,
    ) -> Result<conversation::Model, DbErr> {
        let db_conversation = conversation::ActiveModel {
            state: Set(conversation.state.to_owned()),
            ..Default::default()
        }
        .insert(db)
        .await?;

        conversation_account::ActiveModel {
            conversation_id: Set(db_conversation.id.to_owned()),
            account_id: Set(conversation.recipient.id.to_owned()),
        }
        .insert(db)
        .await?;

        conversation_account::ActiveModel {
            conversation_id: Set(db_conversation.id.to_owned()),
            account_id: Set(sender_id),
        }
        .insert(db)
        .await?;

        Ok(db_conversation)
    }
}
