use crate::Mutation;
use sea_orm::prelude::*;
use sea_orm::Set;

impl Mutation {
    pub async fn create_review(
        db: &DbConn,
        review: entity::dto::review::ReviewDTO,
    ) -> Result<entity::review::Model, DbErr> {
        entity::review::ActiveModel {
            reviewer_id: Set(review.reviewer_id),
            recipient_id: Set(review.recipient_id),
            score: Set(review.score),
            ..Default::default()
        }
        .insert(db)
        .await
    }
}
