use crate::Query;
use ::entity::prelude::Message;
use sea_orm::*;

impl Query {
    pub async fn find_messages_by_conversation(
        db: &DbConn,
        conversation: ::entity::conversation::Model,
        params: Option<::entity::api::message_api::MessageQueryParams>,
    ) -> Result<Vec<::entity::message::Model>, DbErr> {
        let mut query = conversation
            .find_related(Message)
            .order_by_asc(::entity::message::Column::CreatedAt)
            .cursor_by(::entity::message::Column::CreatedAt);

        if let Some(params) = params {
            let mut query_ref = &mut query;
            if let Some(limit) = params.limit {
                query_ref = query_ref.last(limit);
            }
            if let Some(before) = params.before {
                query_ref = query_ref.before(before);
            }

            return query_ref.all(db).await;
        }

        query.all(db).await
    }
}
