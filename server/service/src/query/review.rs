use crate::Query;
use entity::prelude::Review;
use sea_orm::prelude::*;

impl Query {
    pub async fn find_review_by_reviewer_and_recipient(
        db: &DbConn,
        reviewer_id: Uuid,
        recipient_id: Uuid,
    ) -> Result<Option<<Review as EntityTrait>::Model>, DbErr> {
        Review::find()
            .filter(entity::review::Column::ReviewerId.eq(reviewer_id))
            .filter(entity::review::Column::RecipientId.eq(recipient_id))
            .one(db)
            .await
    }
}
