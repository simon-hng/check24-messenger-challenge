use crate::Query;
use entity::prelude::Review;
use sea_orm::prelude::*;

impl Query {
    pub async fn find_review_by_conversation(
        db: &DbConn,
        conversation_id: Uuid,
    ) -> Result<Option<<Review as EntityTrait>::Model>, DbErr> {
        Review::find()
            .filter(entity::review::Column::ConversationId.eq(conversation_id))
            .one(db)
            .await
    }
}
